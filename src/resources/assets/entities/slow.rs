use crate::proto::PackedEntity;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::{distance, AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Slow {
  entity: Entity,
  aura: f64,
}

impl Slow {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 0;
    Self {
      entity,
      aura: 150.0,
    }
  }
}

impl EntityLogic for Slow {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
  }

  fn interact<'a>(&mut self, hero: &mut HeroWrapper) {
    let player = hero.player_mut();
    if !self.entity.harmless
      && player.pos.x > -player.radius
      && player.pos.x - player.radius < self.entity.boundary.w
    {
      if !player.immortal && !player.downed {
        if distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        ) <= self.entity.radius + player.radius
        {
          player.knock();
        }
        if distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        ) <= self.entity.aura + player.radius
        {}
      }
    }
  }

  fn pack(&self) -> PackedEntity {
    self.entity.pack()
  }

  fn entity(&self) -> &Entity {
    &self.entity
  }

  fn entity_mut(&mut self) -> &mut Entity {
    &mut self.entity
  }
}
