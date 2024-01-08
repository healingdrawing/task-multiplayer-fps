use crate::protocol::Direction;
use crate::protocol::*;
use crate::shared::{shared_config, shared_movement_behaviour};
use crate::{Transports, KEY, PROTOCOL_ID};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

#[derive(Resource, Clone, Copy)]
pub struct MyClientPlugin {
  pub(crate) client_id: ClientId,
  pub(crate) client_port: u16,
  pub(crate) server_addr: Ipv4Addr,
  pub(crate) server_port: u16,
  pub(crate) transport: Transports,
}

impl Plugin for MyClientPlugin {
  fn build(&self, app: &mut App) {
    let server_addr = SocketAddr::new(self.server_addr.into(), self.server_port);
    let auth = Authentication::Manual {
      server_addr,
      client_id: self.client_id,
      private_key: KEY,
      protocol_id: PROTOCOL_ID,
    };
    let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), self.client_port);
    let link_conditioner = LinkConditionerConfig {
      incoming_latency: Duration::from_millis(200),
      incoming_jitter: Duration::from_millis(20),
      incoming_loss: 0.05,
    };
    let transport = match self.transport {
      Transports::Udp => TransportConfig::UdpSocket(client_addr),
      Transports::Webtransport => TransportConfig::WebTransportClient {
        client_addr,
        server_addr,
      },
    };
    let io = Io::from_config(
      &IoConfig::from_transport(transport).with_conditioner(link_conditioner),
    );
    let config = ClientConfig {
      shared: shared_config().clone(),
      input: InputConfig::default(),
      netcode: Default::default(),
      ping: PingConfig::default(),
      sync: SyncConfig::default(),
      prediction: PredictionConfig::default(),
      // we are sending updates every frame (60fps), let's add a delay of 6 network-ticks
      interpolation: InterpolationConfig::default()
      .with_delay(InterpolationDelay::default().with_send_interval_ratio(2.0)),
    };
    let plugin_config = PluginConfig::new(config, io, protocol(), auth);
    app.add_plugins(ClientPlugin::new(plugin_config));
    app.add_plugins(crate::shared::SharedPlugin);
    app.insert_resource(self.clone());
    app.add_systems(Startup, init);
    app.add_systems(
      FixedUpdate,
      buffer_input.in_set(InputSystemSet::BufferInputs),
    );
    app.add_systems(FixedUpdate, player_movement.in_set(FixedUpdateSet::Main));
    app.add_systems(
      Update,
      (
        receive_message1,
        receive_entity_spawn,
        receive_entity_despawn,
        handle_player_spawn,
        handle_predicted_spawn,
        handle_interpolated_spawn,
        draw_boxes,
      ),
    );
  }
}

// Startup system for the client
pub(crate) fn init(
  mut commands: Commands,
  mut client: ResMut<Client<MyProtocol>>, // Add the missing generic argument
  plugin: Res<MyClientPlugin>,
) {
  // commands.spawn(Camera3dBundle::default());
  
  // commands.spawn(Camera2dBundle::default()); // todo: replace to 3d camera with gltf scene loaded from file
  
  commands.spawn(TextBundle::from_section(
    format!("Client {}", plugin.client_id),
    TextStyle {
      font_size: 30.0,
      color: Color::WHITE,
      ..default()
    },
  ));
  
  client.connect();
  // client.set_base_relative_speed(0.001);
}

/// attempt to fix multifiring of just_released
#[derive(Resource, Clone, Copy, Default)]
pub struct KeyStates {
  up: bool,
  down: bool,
  left: bool,
  right: bool,
}

// System that reads from peripherals and adds inputs to the buffer
pub(crate) fn buffer_input(
  mut client: ResMut<Client<MyProtocol>>,
  mut key_states: ResMut<KeyStates>,
  keypress: Res<Input<KeyCode>>) {
    let mut direction = Direction {
      up: false,
      down: false,
      left: false,
      right: false,
    };
    
    // one step per unpress (easier to control/check, and as original game)
    if !key_states.up && keypress.just_pressed(KeyCode::Up){
      key_states.up = true;
    }
    if key_states.up && keypress.just_released(KeyCode::Up) {
      key_states.up = false;
      direction.up = true;
    }
    
    if !key_states.down && keypress.just_pressed(KeyCode::Down){
      key_states.down = true;
    }
    if key_states.down && keypress.just_released(KeyCode::Down) {
      key_states.down = false;
      direction.down = true;
    }
    
    // rotate -+90 degrees after release of key.
    // Only one rotation per keypress for left and right
    if !key_states.left && keypress.just_pressed(KeyCode::Left){
      key_states.left = true;
    }
    if key_states.left && keypress.just_released(KeyCode::Left) {
      key_states.left = false;
      direction.left = true;
    }
    
    if !key_states.right && keypress.just_pressed(KeyCode::Right){
      key_states.right = true;
    }
    if key_states.right && keypress.just_released(KeyCode::Right) {
      key_states.right = false;
      direction.right = true;
    }
    
    if !direction.is_none() {
      return client.add_input(Inputs::Direction(direction));
    }
    
    if keypress.pressed(KeyCode::Delete) {
      // currently, inputs is an enum and we can only add one input per tick
      return client.add_input(Inputs::Delete);
    }
    if keypress.pressed(KeyCode::Space) {
      return client.add_input(Inputs::Spawn);
    }
    // info!("Sending input: {:?} on tick: {:?}", &input, client.tick());
    return client.add_input(Inputs::None);
  }
  
  // The client input only gets applied to predicted entities that we own
  // This works because we only predict the user's controlled entity.
  // If we were predicting more entities, we would have to only apply movement to the player owned one.
  fn player_movement(
    // TODO: maybe make prediction mode a separate component!!!
    mut position_query: Query<&mut PlayerPosition, With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
  ) {
    if PlayerPosition::mode() != ComponentSyncMode::Full {
      return;
    }
    for input in input_reader.read() {
      if let Some(input) = input.input() {
        for mut position in position_query.iter_mut() {
          shared_movement_behaviour(&mut position, input);
        }
      }
    }
  }
  
  // System to receive messages on the client
  pub(crate) fn receive_message1(mut reader: EventReader<MessageEvent<Message1>>) {
    for event in reader.read() {
      info!("Received message: {:?}", event.message());
    }
  }
  
  // Example system to handle EntitySpawn events
  pub(crate) fn receive_entity_spawn(mut reader: EventReader<EntitySpawnEvent>) {
    for event in reader.read() {
      info!("Received entity spawn: {:?}", event.entity());
    }
  }
  
  // Example system to handle EntitySpawn events
  pub(crate) fn receive_entity_despawn(mut reader: EventReader<EntityDespawnEvent>) {
    for event in reader.read() {
      info!("Received entity despawn: {:?}", event.entity());
    }
  }
  
  pub(crate) fn handle_player_spawn(
    mut commands: Commands,
    plugin: Res<MyClientPlugin>,
    mut player_spawn: EventReader<ComponentInsertEvent<PlayerId>>,
    players: Query<&PlayerId>,
    asset_server: ResMut<AssetServer>,
  ) {
    for event in player_spawn.read() {
      let entity = event.entity();
      if let Ok(player_id) = players.get(*entity) {
        
        let gltf = SceneBundle {
          scene: asset_server.load("player.gltf#Scene0"),
          ..default()
        };
        
        if player_id.0 == plugin.client_id {
          // this is the controlled player MINIMAP
          commands.entity(*entity)
          .insert(gltf);
        } else {
          // this is another player GLOBAL
          commands.entity(*entity)
          .insert(gltf);
        }
      }
    }
  }
  
  // When the predicted copy of the client-owned entity is spawned, do stuff
  // - assign it a different saturation
  // - keep track of it in the Global resource
  pub(crate) fn handle_predicted_spawn(mut predicted: Query<&mut PlayerColor, Added<Predicted>>) {
    for mut color in predicted.iter_mut() {
      color.0.set_s(0.3);
    }
  }
  
  // When the predicted copy of the client-owned entity is spawned, do stuff
  // - assign it a different saturation
  // - keep track of it in the Global resource
  pub(crate) fn handle_interpolated_spawn(
    mut interpolated: Query<&mut PlayerColor, Added<Interpolated>>,
  ) {
    for mut color in interpolated.iter_mut() {
      color.0.set_s(0.1);
    }
  }
  
  /// Was moved from src/shared.rs, to avoid black magic.
  /// System that draws the boxed of the player positions.
  /// The components should be replicated from the server to the client
  pub(crate) fn draw_boxes(
    mut gizmos: Gizmos,
    players: Query<(&PlayerPosition, &PlayerColor, &PlayerId)>,
    plugin: Res<MyClientPlugin>,
  ) {
    for (position, color, player_id) in &players {
      if player_id.0 == plugin.client_id {
        gizmos.circle(
          Vec3::new(position.x-25.0, position.y, 0.0),
          Vec3::Z,
          0.1,
          Color::GREEN,
        );
      }
    }
  }