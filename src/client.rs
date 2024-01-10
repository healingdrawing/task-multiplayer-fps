use crate::level::can_move_to;
use crate::protocol::Direction;
use crate::protocol::*;
use crate::shared::{shared_config, shared_movement_behaviour, shared_player_shot};
use crate::{Transports, KEY, PROTOCOL_ID};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use lightyear::prelude::client::*;
use lightyear::{prelude::*, inputs};
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use crate::MyCameraMarker;

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
    app.add_systems(
      FixedUpdate,
      player_movement.in_set(FixedUpdateSet::Main)
    );
    app.add_systems(
      FixedUpdate,
      player_shot.in_set(FixedUpdateSet::Main),
    );
    
    // app.add_systems(FixedUpdate, update_camera_position.in_set(FixedUpdateSet::Main));
    
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
        draw_lines,
        update_camera_position,
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
  shot: bool,
}

// System that reads from peripherals and adds inputs to the buffer
pub(crate) fn buffer_input(
  mut client: ResMut<Client<MyProtocol>>,
  mut key_states: ResMut<KeyStates>,
  keypress: Res<Input<KeyCode>>
) {
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
  
  // shot on server side will be calculated only after release of space key
  // Delete event used for server side shot.
  if !key_states.shot && keypress.just_pressed(KeyCode::Space){
    key_states.shot = true;
  }
  if key_states.shot && keypress.just_released(KeyCode::Space) {
    key_states.shot = false;
    return client.add_input(Inputs::Delete);
  }
  
  // if keypress.pressed(KeyCode::Delete) {
    //   // currently, inputs is an enum and we can only add one input per tick
    //   return client.add_input(Inputs::Delete);
    // }
    
    if keypress.pressed(KeyCode::Space) {
      // draw gizmo lines of shot on client
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
  
  pub(crate) fn player_shot(
    plugin: Res<MyClientPlugin>,
    mut position_query: Query<(&mut PlayerPosition, &PlayerId)>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
  ) {
    // if PlayerPosition::mode() != ComponentSyncMode::Full { return; }
    // println!("Found {} entities in query", position_query.iter().count());
    
    for input in input_reader.read() {
      if let Some(input) = input.input() {
        match input {
          Inputs::Delete => {
            
            // first fill the cells affected by the shot
            let mut cells = Vec::new();
            
            for (position, player_id) in &mut position_query.iter_mut() {
              if player_id.0 == plugin.client_id
              && position.x > 1.0 && position.x < 23.0
              && position.y > 1.0 && position.y < 23.0{
                
                // increment to front of the player
                let (dx, dy) = match position.z as i32 {
                  0 => (1.0, 0.0),
                  90 => (0.0, 1.0),
                  180 => (-1.0, 0.0),
                  270 => (0.0, -1.0),
                  _ => (0.0, 0.0),
                };
                let (mut cx, mut cy) = (position.x + dx, position.y + dy);
                while can_move_to(cx,cy) {
                  cells.push((cx, cy));
                  cx += dx;
                  cy += dy;
                }
                
              }
            }
            
            // then check if any other player is in the cells
            for (mut position, player_id) in &mut position_query.iter_mut() {
              
              shared_player_shot(&mut position, &mut cells);
              
            }
            
          },
          _ => {},
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
          // commands.entity(*entity).insert(gltf);
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
  
  /// System that draws the lines of the player shot.
  
  use rand::prelude::*;
  use std::f32::consts::PI;
  
  fn circle_xy(r: f32) -> (f32, f32) {
    let mut rng = thread_rng();
    
    // Generate a random angle
    let angle = rng.gen_range(0.0..2.0 * PI);
    
    // Generate a random distance from the center to the edge of the circle
    let distance = rng.gen_range(0.0..r);
    
    // Calculate the x and y coordinates
    let x = r * angle.cos();
    let y = r * angle.sin();
    
    (x, y)
  }
  
  /// Generate a random radius
  fn radius(r: f32) -> f32 { thread_rng().gen_range(0.0..r) }
  
  pub(crate) fn draw_lines(
    mut gizmos: Gizmos,
    players: Query<(&PlayerPosition, &PlayerId)>,
    plugin: Res<MyClientPlugin>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
  ) {
    for (position, player_id) in &players {
      if player_id.0 == plugin.client_id
      && position.x > 1.0 && position.x < 23.0
      && position.y > 1.0 && position.y < 23.0
      {
        
        if PlayerPosition::mode() != ComponentSyncMode::Full {
          return;
        }
        for input in input_reader.read() {
          if let Some(input) = input.input() {
            
            match input {
              Inputs::Spawn => {
                
                let look_at_target = match position.z as i32 {
                  0 => Vec3::new(position.x + 0.4 , position.y, 0.0),
                  90 => Vec3::new(position.x, position.y + 0.4, 0.0),
                  180 => Vec3::new(position.x - 0.4, position.y, 0.0),
                  270 => Vec3::new(position.x, position.y - 0.4, 0.0),
                  _ => Vec3::new(position.x, position.y, 0.0),
                };
                
                // randomize the start point of the shot around circle
                // use dx, dy pairs to randomize the start point of the shot
                let (d, dz) = circle_xy(0.05);
                let look_from = match position.z as i32 {
                  0 => Vec3::new(position.x - 0.5 , position.y + d, dz),
                  90 => Vec3::new(position.x + d, position.y - 0.5, dz),
                  180 => Vec3::new(position.x + 0.5, position.y + d, dz),
                  270 => Vec3::new(position.x + d, position.y + 0.5, dz),
                  _ => Vec3::new(position.x + d, position.y, dz),
                };
                
                gizmos.line(
                  look_from,
                  look_at_target,
                  Color::RED,
                );
                
                let normal = match position.z as i32 {
                  0 => Vec3::X,
                  90 => Vec3::Y,
                  180 => Vec3::X,
                  270 => Vec3::Y,
                  _ => Vec3::Z,
                };
                gizmos.circle(look_at_target, normal, radius(0.05), Color::RED);
                
              },
              _ => continue,
            };
            
          }
        }
        
      }
    }
  }
  
  /// System that updates the player positions.
  pub(crate) fn update_camera_position(
    mut cameras: Query<(&Camera, With<MyCameraMarker>, &mut Transform)>,
    // mut players: Query<(&PlayerPosition, &PlayerId)>,
    mut players: Query<(&PlayerPosition, &PlayerId), With<Confirmed>>,
    plugin: Res<MyClientPlugin>,
  ) {
    for (position, player_id) in &mut players.iter_mut() {
      if player_id.0 == plugin.client_id {
        
        let (camera, _, mut transform) = cameras.single_mut();
        
        let look_at_target = match position.z as i32 {
          0 => Vec3::new(position.x + 1.0 , position.y, 0.0),
          90 => Vec3::new(position.x, position.y + 1.0, 0.0),
          180 => Vec3::new(position.x - 1.0, position.y, 0.0),
          270 => Vec3::new(position.x, position.y - 1.0, 0.0),
          _ => Vec3::new(position.x, position.y, 0.0),
        };
        
        let look_from = match position.z as i32 {
          0 => Vec3::new(position.x - 0.5 , position.y, 0.0),
          90 => Vec3::new(position.x, position.y - 0.5, 0.0),
          180 => Vec3::new(position.x + 0.5, position.y, 0.0),
          270 => Vec3::new(position.x, position.y + 0.5, 0.0),
          _ => Vec3::new(position.x, position.y, 0.0),
        };
        
        transform.look_at(
          look_at_target,
          Vec3::Z,
        );
        transform.translation.x = look_from.x;
        transform.translation.y = look_from.y;
        transform.translation.z = look_from.z;
        // transform.rotation = Quat::from_rotation_z(position.z.to_radians());
        
        break;
        
        
        // println!("position: {:?}", position); // todo: remove this
      }
    }
  }
  