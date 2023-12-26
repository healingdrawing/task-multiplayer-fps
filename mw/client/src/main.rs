use bevy::prelude::* ;

fn main() {
  App::new().add_plugins(DefaultPlugins.set(
    WindowPlugin {
      primary_window: Some(
        Window {
          resizable: false,
          resolution: (1000., 1000.).into(),
          title: "asspain + headache + brainfuck".into(),
          enabled_buttons: bevy::window::EnabledButtons{
            maximize: false,
            ..default()
          },
          ..default()
        }
      ),
      ..default()
    }
  ))
  .run();
}
