use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore}} ;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_egui::{egui::{self, Align2, RichText, Color32, FontId}, EguiContexts};

/// This system will show the FPS on the screen
pub fn show_fps_ui(
  mut contexts: EguiContexts,
  diagnostics: Res<DiagnosticsStore>,
) {
  let fps = match diagnostics
  .get(FrameTimeDiagnosticsPlugin::FPS)
  .and_then(|fps| fps.smoothed()){
    Some(v) => v.round().to_string(),
    None => "N/A".to_string()
  };
  
  egui:: Area::new("fps")
  .anchor(Align2::CENTER_TOP, (0., 25.))
  .show(contexts.ctx_mut(), |ui| {
    ui.label(
      RichText::new(format!("fps: {}",fps))
      .color(Color32::WHITE)
      .font(FontId::proportional(72.0)),
    );
  });
}

// try fix ticks issue
pub fn limit_fps_ui(mut settings: ResMut<FramepaceSettings>){
  settings.limiter = Limiter::from_framerate(59.0);
}

// force slowdown gap
pub fn force_sleep(){
  std::thread::sleep(std::time::Duration::from_millis(5));
}
