use crate::{
  network::PackedEntity,
  units::{player::Player, structures::UpdateProps},
};

pub trait Enemy {
  fn update(&mut self, props: UpdateProps);
  fn interact(&mut self, player: &mut Player);
  fn pack(&self) -> PackedEntity;
}
