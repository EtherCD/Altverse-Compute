use crate::{
  assets::entity::Enemies,
  network::PackedEntity,
  units::{entity::Entity, player::Player, structures::EntityUpdateProps},
};

pub trait Enemy {
  fn update(&mut self, props: &EntityUpdateProps);
  fn interact(&mut self, player: &mut Player);
  fn pack(&self) -> PackedEntity;
  fn is_to_remove(&self) -> bool;
  fn get_nested_entities(&self) -> Vec<Enemies>;
  fn clear_nested_entities(&mut self);
}
