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
      // app.add_systems(Update, draw_boxes);
      app.add_systems(FixedUpdate, update_player_positions);
    }
  }
}

use std::f32::consts::PI;
pub const STEP_MOVE: f32 = 1.0; // expect this as one meter of world system
pub const STEP_ANGLE: f32 = 90.0;

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
        let dx = (position.z.to_radians().cos() * STEP_MOVE).round();
        let dy = (position.z.to_radians().sin() * STEP_MOVE).round();
        if can_move_to(position.x + dx, position.y + dy) {
          position.x += dx;
          position.y += dy;
        }
        
      }
      if direction.down {
        let dx = (position.z.to_radians().cos() * STEP_MOVE).round();
        let dy = (position.z.to_radians().sin() * STEP_MOVE).round();
        if can_move_to(position.x - dx, position.y - dy) {
          position.x -= dx;
          position.y -= dy;
        }
        
      }
      if direction.left {
        position.z = strict_angle_degrees(position.z, STEP_ANGLE);
        // println!("position.z: {}", position.z); // todo: remove this
      }
      if direction.right {
        position.z = strict_angle_degrees(position.z, -STEP_ANGLE);
        // println!("position.z: {}", position.z); // todo: remove this
      }
    },
    _ => {}
  }
}

/// System that updates the player positions.
pub(crate) fn update_player_positions(
  mut players: Query<(&PlayerPosition, &mut Transform)>,
) {
  for (position, mut transform) in &mut players.iter_mut() {
    
    // no strict answer is precheck evil or not in this case
    if (transform.translation.x - position.x).abs() > STEP_MOVE*0.5
    || (transform.translation.y - position.y).abs() > STEP_MOVE*0.5
    || (transform.rotation.to_axis_angle().1 - position.z.to_radians()).abs() > STEP_ANGLE.to_radians()*0.5
    {
      transform.translation.x = position.x;
      transform.translation.y = position.y;
      transform.rotation = Quat::from_rotation_z(position.z.to_radians());
    }
    
    // println!("position: {:?}", position); // todo: remove this (spamming every frame)
    
  }
}

pub(crate) fn shared_player_shot(
  position: &mut PlayerPosition,
  cells: &mut Vec<(f32, f32)>,
) {
  
  if position.x > 1.0 && position.x < 23.0
  && position.y > 1.0 && position.y < 23.0
  && cells.contains(&(position.x, position.y))
  {
    position.x = 25.0;
    println!("shared player shot. delete after shot hit:\n{:?}", position);
  }
}
