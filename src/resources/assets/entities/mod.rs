use crate::proto::PackedEntity;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::EntityUpdateProps;

pub mod bee;
pub mod drop;
pub mod fade;
pub mod flame;
pub mod flamesniper;
pub mod homing;
pub mod homingsniper;
pub mod immune;
pub mod normal;
pub mod slow;
pub mod sniper;
pub mod wall;

pub trait EntityLogic {
  fn update(&mut self, props: &mut EntityUpdateProps);
  fn interact(&mut self, player: &mut HeroWrapper);
  fn pack(&self) -> PackedEntity;
  fn entity(&self) -> &Entity;
  fn entity_mut(&mut self) -> &mut Entity;
}
