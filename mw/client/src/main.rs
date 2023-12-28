// use bevy::diagnostic::LogDiagnosticsPlugin; // to print fps in terminal
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;

mod gui;
use gui::title::show_time::change_window_title;
use gui::fps::show_fps::show_fps_ui;

mod keyboard;
use keyboard::activity::listen_keys;

/// it is a client for the server
fn main() {
  
  // placeholder:
  // for request the server ip:port from user terminal input, and store it in the variable
  // only numbers , dots and colon allowed, otherwise print error and loop again to collect the ip:port

  // placehoder:
  // for request the user name from user terminal input, and store it in the variable
  // only the english letters and the numbers are allowed, otherwise print error message and loop again to collect the name

  // placehoder:
  // after collecting the ip:port and the name, try to connect to the server
  // if connection is not successful/fail, then print error message and loop again to collect the ip:port and the name

  // if connection is successful, then start the game

  App::new()
  .add_plugins(
    (
      DefaultPlugins.set(
        WindowPlugin {
          primary_window: Some(
            Window {
              // resizable: false, // it is dead, official bug for x11/macos
              resolution: (1000., 1000.).into(),
              title: "asspain + headache + brainfuck".into(),
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

  // placeholder for game over, just close the window and/or exit. Not sure how make it the best way
}
