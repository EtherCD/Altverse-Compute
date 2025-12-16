use crate::proto::PackedEntity;
use crate::resources::player::Player;
use crate::resources::EntityUpdateProps;

pub mod flame;
pub mod normal;

pub trait EntityLogic {
  fn update(&mut self, props: &mut EntityUpdateProps);
  fn interact(&mut self, player: &mut Player);
  fn pack(&self) -> PackedEntity;
  fn is_to_remove(&self) -> bool;
}
