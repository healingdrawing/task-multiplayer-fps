// use bevy::diagnostic::LogDiagnosticsPlugin; // to print fps in terminal
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;

mod gui;
use gui::title::show_time::change_window_title;
use gui::fps::show_fps::show_fps_ui;

mod keyboard;
use keyboard::activity::listen_keys;

fn main() {
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
}
