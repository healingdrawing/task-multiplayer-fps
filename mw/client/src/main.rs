// use bevy::diagnostic::LogDiagnosticsPlugin; // to print fps in terminal
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;

mod gui;
use gui::title::show_time::change_window_title;
use gui::fps::show_fps::show_fps_ui;

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
      toggle_window_controls,
    )
  )
  .run();
}


fn toggle_window_controls(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
  let toggle_minimize = input.just_pressed(KeyCode::Key1);
  let toggle_maximize = input.just_pressed(KeyCode::Key2);
  let toggle_close = input.just_pressed(KeyCode::Key3);
  
  if toggle_minimize || toggle_maximize || toggle_close {
    let mut window = windows.single_mut();
    println!("1 or 2 or 3 pressed. not numpad");
    if toggle_minimize {
      window.enabled_buttons.minimize = !window.enabled_buttons.minimize;
    }
    if toggle_maximize {
      window.enabled_buttons.maximize = !window.enabled_buttons.maximize;
    }
    if toggle_close {
      window.enabled_buttons.close = !window.enabled_buttons.close;
    }
  }
}
