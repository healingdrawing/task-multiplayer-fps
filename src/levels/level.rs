use bevy::math::Vec3;

use crate::GAME_LEVEL;

use crate::levels::level1::LEVEL1;

/// check the cell of the level is able to be used as moving target
/// 
/// (not a wall not a spawn position, and inside the level indices)
pub fn can_move_to(x:f32, y:f32) -> bool{
  let x = x.round() as usize;
  let y = y.round() as usize;
  // 2 and 22 is 0..24 +- 2(two first/last cells which is border and spawn area)
  if x < 2 || x > 22 || y < 2 || y > 22{return false}
  
  match GAME_LEVEL{
    1 => {LEVEL1[y][x] == 0}, // if cell of level1 is 0 (empty), can move to it
    _ => {false} //todo: extend to LEVEL2 and LEVEL3, properly
  }
}

pub const SPAWN_POSITIONS: [[u8; 2];40] = [
  [1,3],[1,5],[1,7],[1,9],[1,11],[1,13],[1,15],[1,17],[1,19],[1,21], // left 10
  [23,3],[23,5],[23,7],[23,9],[23,11],[23,13],[23,15],[23,17],[23,19],[23,21], // right 10
  [3,1],[5,1],[7,1],[9,1],[11,1],[13,1],[15,1],[17,1],[19,1],[21,1], // bottom 10
  [3,23],[5,23],[7,23],[9,23],[11,23],[13,23],[15,23],[17,23],[19,23],[21,23], // top 10
];

/// return random spawn position, from list of available cells to spawn
pub fn get_random_spawn_position() -> Vec3{
  let index = rand::random::<u8>() % 40;
  let random_spawn_position = SPAWN_POSITIONS[index as usize];
  let x = random_spawn_position[0] as f32;
  let y = random_spawn_position[1] as f32;
  let z = match index{
    0..=9 => {0.0}, // right directed
    10..=19 => {180.0}, // left directed
    20..=29 => {90.0}, // up directed
    30..=39 => {270.0}, // down directed
    _ => {
      println!("Error: get_random_spawn_position() index is out of range");
      0.0
    } // should never fire
  };
  Vec3::new(x, y, z)
}
