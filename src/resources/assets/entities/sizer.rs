use crate::proto::PackedEntity;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Sizer {
  entity: Entity,
  min_radius: f64,
  max_radius: f64,
  growing: bool,
}

impl Sizer {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    let radius = entity.radius;
    entity.type_id = 24;
    Self {
      entity,
      min_radius: radius * 2.5,
      max_radius: radius / 2.5,
      growing: true,
    }
  }
}

impl EntityLogic for Sizer {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
    if self.growing {
      self.entity.radius += (props.time_fix * 0.08) * self.min_radius;
      if self.entity.radius > self.max_radius {
        self.growing = false;
      }
    } else {
      self.entity.radius -= ((props.delta as f64 / 30.0) * 0.08) * self.min_radius;
      if self.entity.radius < self.min_radius {
        self.growing = true;
      }
    }
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
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
