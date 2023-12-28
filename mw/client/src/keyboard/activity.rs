use bevy::prelude::*;

pub fn listen_keys(input: Res<Input<KeyCode>>, mut _windows: Query<&mut Window>) {
  let toggle_minimize = input.just_pressed(KeyCode::Key1);
  let toggle_maximize = input.just_pressed(KeyCode::Key2);
  let toggle_close = input.just_pressed(KeyCode::Key3);
  
  if toggle_minimize || toggle_maximize || toggle_close {
    // let mut window = windows.single_mut(); // also uncomment input _windows
    println!("1 or 2 or 3 pressed. Not numpad");
  }

  if input.just_pressed(KeyCode::Key1){
    println!("1 pressed")
  } else if input.just_released(KeyCode::Key1){
    println!("1 released")
  }
  
}
