
use bevy::prelude::*;

/// This system will then change the title during execution
pub fn change_window_title(mut windows: Query<&mut Window>, time: Res<Time>) {
  let mut window = windows.single_mut();
  window.title = format!(
    "Seconds since startup: {}",
    time.elapsed().as_secs_f32().round()
  );
}
