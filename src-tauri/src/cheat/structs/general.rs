use serde::Serialize;
use crate::cheat::structs::vectors::Vector2;

#[derive(Clone, Serialize)]
pub struct Entity {
  pub name: String,
  pub health: i32,
  pub screen_pos: Vector2
}


pub struct Addresses {
    pub player_count: usize,
    pub local_player: usize,
    pub entity_list: usize,
    pub view_matrix: usize
}
