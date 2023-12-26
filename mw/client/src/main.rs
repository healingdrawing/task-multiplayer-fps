use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}} ;

fn main() {
  App::new()
  .add_plugins(
    (
      DefaultPlugins.set(
        WindowPlugin {
          primary_window: Some(
            Window {
              resizable: false,
              resolution: (1000., 1000.).into(),
              title: "asspain + headache + brainfuck".into(),
              // enabled_buttons: bevy::window::EnabledButtons{ maximize: false, ..default() },
              ..default()
            }
          ),
          ..default()
        }
      ),
      LogDiagnosticsPlugin::default(),
      FrameTimeDiagnosticsPlugin,
    )
    
  ).insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
  .add_systems(
    Update,
    (
      change_title,
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

/// This system will then change the title during execution
fn change_title(mut windows: Query<&mut Window>, time: Res<Time>) {
  let mut window = windows.single_mut();
  window.title = format!(
    "Seconds since startup: {}",
    time.elapsed().as_secs_f32().round()
  );
}