// use bevy::diagnostic::LogDiagnosticsPlugin; // to print fps in terminal
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;

mod gui;
use gui::title::show_time::change_window_title;
use gui::fps::show_fps::show_fps_ui;

mod keyboard;
use keyboard::activity::listen_keys;

mod user;
use user::info;

mod connection;
use connection::connect;

/// it is a client for the server
fn main() {
  let server_address = info::get_server_address();
  let user_name = info::get_user_name();
  if let Err(error) = connect::connect_to(&server_address, &user_name) {
    println!("Connection error: {}", error);
    return;
  }
  
  // if connection is successful, then start the game. Very raw

  App::new()
  .add_plugins(
    (
      DefaultPlugins.set(
        WindowPlugin {
          primary_window: Some(
            Window {
              resolution: (1000., 1000.).into(),
              title: "Thank you for your help! I want to complete it before February".into(),
              ..default()
            }
          ),
          ..default()
        }
      ),
      EguiPlugin,
      // LogDiagnosticsPlugin::default(), // to print fps in terminal
      FrameTimeDiagnosticsPlugin::default(),
    )
    
  )
  // .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 0.0))) // it is dead/black
  
  .add_systems(
    Update,
    (
      show_fps_ui,
      change_window_title,
      listen_keys,
    )
  )
  .run();
  
  // placeholder for game over. Probably just close the window and/or exit. Not sure how make it the best way
}
