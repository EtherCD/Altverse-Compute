use crate::proto::PackedPlayer;
use crate::resources::player::Player;
use crate::resources::utils::input::Input;
use crate::resources::{Boundary, PlayerUpdateProps};

pub mod maven;

pub trait Hero {
  fn update(&mut self, props: &mut PlayerUpdateProps);
  fn input(&mut self, input: &mut Input);
  fn knock(&mut self);
  fn res(&mut self);
  fn collide(&mut self, boundary: Boundary);
  fn pack(&self) -> PackedPlayer;
  fn player(&self) -> &Player;
  fn player_mut(&mut self) -> &mut Player;
}
