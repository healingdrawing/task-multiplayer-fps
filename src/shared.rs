use crate::level::can_move_to;
use crate::protocol::*;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::*;
use std::time::Duration;
use tracing::Level;
use crate::GAME_LEVEL;

pub fn shared_config() -> SharedConfig {
  SharedConfig {
    enable_replication: true,
    client_send_interval: Duration::default(),
    server_send_interval: Duration::from_millis(40),
    // server_send_interval: Duration::from_millis(100),
    tick: TickConfig {
      tick_duration: Duration::from_secs_f64(1.0 / 64.0),
    },
    log: LogConfig {
      level: Level::INFO,
      filter: "wgpu=error,wgpu_hal=error,naga=warn,bevy_app=info,bevy_render=warn,quinn=warn"
      .to_string(),
    },
  }
}

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
  fn build(&self, app: &mut App) {
    if app.is_plugin_added::<RenderPlugin>() {
      app.add_systems(Update, draw_boxes);
      app.add_systems(FixedUpdate, update_player_positions);
    }
  }
}

use std::f32::consts::PI;
const STEP_MOVE: f32 = 1.0; // expect this as one meter of world system
const STEP_ANGLE: f32 = 90.0;

/// only 0, 90, 180, 270 degrees are allowed. So calculate the closest one
fn strict_angle_degrees(angle_degrees: f32, appendix_degrees:f32) -> f32 {
  // calculate positive rotation angle in degrees. [0, 360)
  let angle_degrees = ( angle_degrees + appendix_degrees ) % 360.0;
  let positive_angle_degrees =
  if angle_degrees < 0.0 { angle_degrees + 360.0 } else {angle_degrees};
  // calculate the closest angle in degrees. [0, 90, 180, 270]
  let result = (positive_angle_degrees / 90.0).round() * 90.0;
  if result == 360.0{0.0}else{result}
}

pub(crate) fn shared_movement_behaviour(
  position: &mut PlayerPosition,
  input: &Inputs,
) {
  match input {
    Inputs::Direction(direction) => {
      if direction.up {
        let dx = position.z.to_radians().cos() * STEP_MOVE;
        let dy = position.z.to_radians().sin() * STEP_MOVE;
        if can_move_to(position.x + dx, position.y + dy) {
          position.x += dx;
          position.y += dy;
        }
        
      }
      if direction.down {
        let dx = position.z.to_radians().cos() * STEP_MOVE;
        let dy = position.z.to_radians().sin() * STEP_MOVE;
        if can_move_to(position.x - dx, position.y - dy) {
          position.x -= dx;
          position.y -= dy;
        }
        
      }
      if direction.left {
        position.z = strict_angle_degrees(position.z, STEP_ANGLE);
        println!("position.z: {}", position.z);
      }
      if direction.right {
        position.z = strict_angle_degrees(position.z, -STEP_ANGLE);
        println!("position.z: {}", position.z);
      }
    }
    _ => {}
  }
}

/*
// This system defines how we update the player's positions when we receive an input
pub(crate) fn shared_movement_behaviour_example(
  position: &mut PlayerPosition,
  input: &Inputs
) {
  const MOVE_SPEED: f32 = 10.0;
  match input {
    Inputs::Direction(direction) => {
      if direction.up {
        position.y += MOVE_SPEED;
      }
      if direction.down {
        position.y -= MOVE_SPEED;
      }
      if direction.left {
        position.x -= MOVE_SPEED;
      }
      if direction.right {
        position.x += MOVE_SPEED;
      }
    }
    _ => {}
  }
}
*/

/// System that draws the boxed of the player positions.
/// The components should be replicated from the server to the client
pub(crate) fn draw_boxes(mut gizmos: Gizmos, players: Query<(&PlayerPosition, &PlayerColor)>) {
  for (position, color) in &players {
    gizmos.rect(
      Vec3::new(position.x, position.y, 0.0),
      Quat::IDENTITY,
      Vec2::ONE,// * 50.0,
      color.0,
    );
  }
}

/// System that updates the player positions.
pub(crate) fn update_player_positions(
  mut players: Query<(&PlayerPosition, &mut Transform)>,
) {
  for (position, mut transform) in &mut players.iter_mut() {

    transform.translation.x = position.x;

    transform.translation.y = position.y;
    
    //todo: check this. it looks absolutely raw. But first fix, blender export axes
    transform.rotation = Quat::from_rotation_z(position.z.to_radians());
    
    println!("position: {:?}", position);

  }
}