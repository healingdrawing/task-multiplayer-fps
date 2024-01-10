use crate::level::{get_random_spawn_position, can_move_to};
use crate::protocol::*;
use crate::shared::{shared_config, shared_movement_behaviour, shared_player_shot};
use crate::{shared, Transports, KEY, PROTOCOL_ID};
use bevy::prelude::*;
use lightyear::client::components::Confirmed;
use lightyear::client::prediction::Predicted;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

pub struct MyServerPlugin {
  pub(crate) headless: bool,
  pub(crate) port: u16,
  pub(crate) transport: Transports,
}

impl Plugin for MyServerPlugin {
  fn build(&self, app: &mut App) {
    let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), self.port);
    let netcode_config = NetcodeConfig::default()
    .with_protocol_id(PROTOCOL_ID)
    .with_key(KEY);
    let link_conditioner = LinkConditionerConfig {
      incoming_latency: Duration::from_millis(200),
      incoming_jitter: Duration::from_millis(20),
      incoming_loss: 0.05,
    };
    let transport = match self.transport {
      Transports::Udp => TransportConfig::UdpSocket(server_addr),
      Transports::Webtransport => TransportConfig::WebTransportServer {
        server_addr,
        certificate: Certificate::self_signed(&["localhost"]),
      },
    };
    let io = Io::from_config(
      &IoConfig::from_transport(transport).with_conditioner(link_conditioner),
    );
    let config = ServerConfig {
      shared: shared_config().clone(),
      netcode: netcode_config,
      ping: PingConfig::default(),
    };
    let plugin_config = PluginConfig::new(config, io, protocol());
    app.add_plugins(server::ServerPlugin::new(plugin_config));
    app.add_plugins(shared::SharedPlugin);
    app.init_resource::<Global>();
    // app.add_systems(Startup, init); //todo: attempt to decrease the used sources
    // the physics/FixedUpdates systems that consume inputs should be run in this set
    app.add_systems(FixedUpdate, movement.in_set(FixedUpdateSet::Main));
    app.add_systems(FixedUpdate, shooting.in_set(FixedUpdateSet::Main));
    if !self.headless {
      app.add_systems(Update, send_message);
    }
    app.add_systems(Update, handle_connections);
  }
}

#[derive(Resource, Default)]
pub(crate) struct Global {
  pub client_id_to_entity_id: HashMap<ClientId, Entity>,
}

pub(crate) fn init(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
  commands.spawn(TextBundle::from_section(
    "Server",
    TextStyle {
      font_size: 30.0,
      color: Color::WHITE,
      ..default()
    },
  ));
}

/// Server connection system, create a player upon connection
pub(crate) fn handle_connections(
  mut connections: EventReader<ConnectEvent>,
  mut disconnections: EventReader<DisconnectEvent>,
  mut global: ResMut<Global>,
  mut commands: Commands,
) {
  for connection in connections.read() {
    let client_id = connection.context();
    // Generate pseudo random color from client id.
    let h = (((client_id * 30) % 360) as f32) / 360.0;
    let entity = commands.spawn(PlayerBundle::new(
      *client_id,
      get_random_spawn_position(),
      Color::hsl(h, 0.8, 0.5),
    ));
    // Add a mapping from client id to entity id
    global
    .client_id_to_entity_id
    .insert(*client_id, entity.id());
  }
  for disconnection in disconnections.read() {
    let client_id = disconnection.context();
    if let Some(entity) = global.client_id_to_entity_id.remove(client_id) {
      commands.entity(entity).despawn();
    }
  }
}

/// Read client inputs and move players
pub(crate) fn movement(
  mut position_query: Query<&mut PlayerPosition>,
  mut input_reader: EventReader<InputEvent<Inputs>>,
  global: Res<Global>,
  server: Res<Server<MyProtocol>>,
) {
  for input in input_reader.read() {
    let client_id = input.context();
    if let Some(input) = input.input() {
      debug!(
        "Receiving input: {:?} from client: {:?} on tick: {:?}",
        input,
        client_id,
        server.tick()
      );
      if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
        if let Ok(mut position) = position_query.get_mut(*player_entity) {
          shared_movement_behaviour(&mut position, input);
        }
      }
    }
  }
}

/// Read client inputs and move players outside of the map
pub(crate) fn shooting(
  mut position_query: Query<&mut PlayerPosition>,
  mut input_reader: EventReader<InputEvent<Inputs>>,
  global: Res<Global>,
  server: Res<Server<MyProtocol>>,
) {
  for input in input_reader.read() {
    let client_id = input.context();
    if let Some(input) = input.input() {
      debug!(
        "Receiving input: {:?} from client: {:?} on tick: {:?}",
        input,
        client_id,
        server.tick()
      );
      // check the input is Delete
      match input {
        Inputs::Delete => {
          if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
            let position = position_query.get_mut(*player_entity);
            match position {
              Ok(position) => {
                
                // check the player is inside the map, so we can shoot
                if position.x > 1.0 && position.x < 23.0
                && position.y > 1.0 && position.y < 23.0{
                  
                  // first fill the cells affected by the shot
                  let mut cells = Vec::new();
                  
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
                  
                  // then check if any other player is in the cells
                  for mut position in &mut position_query.iter_mut() {
                    
                    shared_player_shot(&mut position, &mut cells);
                    
                  }
                  
                }
                
                
              },
              Err(e) => {println!("Rust Magic Happens again. Error: {:?}", e)},
            }
          }
        },
        _ => {  },
      }
      
    }
  }
}

/// Send messages from server to clients (only in non-headless mode, because otherwise we run with minimal plugins
  /// and cannot do input handling)
  pub(crate) fn send_message(mut server: ResMut<Server<MyProtocol>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::M) {
      // TODO: add way to send message to all
      let message = Message1(5);
      info!("Send message: {:?}", message);
      server
      .send_message_to_target::<Channel1, Message1>(Message1(5), NetworkTarget::All)
      .unwrap_or_else(|e| {
        error!("Failed to send message: {:?}", e);
      });
    }
  }