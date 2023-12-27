use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, DiagnosticsStore}} ;
use bevy_egui::{egui::{self, Align2, RichText, Color32, FontId}, EguiPlugin, EguiContexts};


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
      EguiPlugin,
      LogDiagnosticsPlugin::default(),
      FrameTimeDiagnosticsPlugin::default(),
    )
    
  )
  .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
  
  .add_systems(
    Update,
    (
      update_fps_ui,
      change_title,
      toggle_window_controls,
    )
  )
  .run();
}

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

fn update_fps_ui(
  mut contexts: EguiContexts,
  diagnostics: Res<DiagnosticsStore>,
) {
  let fps = match diagnostics
  .get(FrameTimeDiagnosticsPlugin::FPS)
  .and_then(|fps| fps.smoothed()){
    Some(v) => v.to_string(),
    None => "N/A".to_string()
  };
  
  egui:: Area::new("fps")
  .anchor(Align2::CENTER_TOP, (0., 25.))
  .show(contexts.ctx_mut(), |ui| {
    ui.label(
      RichText::new(format!("{}",fps))
      .color(Color32::WHITE)
      .font(FontId::proportional(72.0)),
    );
  });
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