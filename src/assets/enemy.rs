use crate::{
  assets::entity::EnemyWrapper,
  network::PackedEntity,
  units::{player::Player, structures::EntityUpdateProps},
};

pub trait Enemy {
  fn update(&mut self, props: &EntityUpdateProps);
  fn interact(&mut self, player: &mut Player);
  fn pack(&self) -> PackedEntity;
  fn is_to_remove(&self) -> bool;
  fn get_nested_entities(&self) -> Vec<EnemyWrapper>;
  fn clear_nested_entities(&mut self);
}
