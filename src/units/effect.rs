use crate::units::{player::Player, structures::UpdateProps};

pub trait Effect {
  fn update(&mut self, update: &UpdateProps) -> bool;
  fn clear(&mut self);
}
