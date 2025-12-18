use crate::proto::PackedEntity;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Flame {
  entity: Entity,
  timer: f64,
}

impl Flame {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 18;
    Self { entity, timer: 0.0 }
  }
}

impl EntityLogic for Flame {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;
    if self.timer >= 32.0 * ((self.entity.radius * 2.0) / self.entity.speed) {
      let mut trail = FlameTrail::new(
        EntityProps {
          id: 1,
          type_id: 19,
          radius: self.entity.radius,
          speed: 0.0,
          boundary: self.entity.boundary,
        },
        AdditionalEntityProps {
          count: 0,
          num: 0,
          inverse: false,
        },
      );
      trail.entity.pos = self.entity.pos.clone();

      trail.owner_speed = self.entity.speed;
      self.timer = 0.0;
      props.event_bus.add_entity(EntityWrapper::FlameTrail(trail));
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

#[derive(Clone)]
pub struct FlameTrail {
  pub entity: Entity,
  timer: f64,
  pub owner_speed: f64,
}

impl FlameTrail {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
      timer: 0.0,
      owner_speed: 0.0,
    }
  }
}

impl EntityLogic for FlameTrail {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;
    self.entity.alpha = 1.0 - self.timer / (5000.0 / self.owner_speed);
    if self.timer >= 5000.0 / self.owner_speed {
      self.entity.to_remove = true;
    }
    if self.timer >= 3500.0 {
      self.entity.harmless = true;
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
