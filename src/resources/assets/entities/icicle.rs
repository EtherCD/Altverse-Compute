use crate::proto::PackedEntity;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::{random, AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Icicle {
  entity: Entity,
  timer: i64,
  wall_hit: bool,
}

impl Icicle {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 25;
    entity.vel.x = 0.0;
    entity.vel.y = ((random(0.0, 1.0) * 2.0).floor() * 2.0 - 1.0) * entity.speed;
    entity.collide();
    Self { entity, wall_hit: false, timer: 0 }
  }

  fn collide(&mut self) {
    let entity = &mut self.entity;
    if entity.pos.x - entity.radius < entity.boundary.x {
      entity.pos.x = entity.boundary.x + entity.radius;
      entity.vel.x = entity.vel.x.abs()
    }
    if entity.pos.x + entity.radius > entity.boundary.x + entity.boundary.w {
      entity.pos.x = entity.boundary.x + entity.boundary.w - entity.radius;
      entity.vel.x = -(entity.vel.x.abs())
    }
    if entity.pos.y - entity.radius < entity.boundary.y {
      entity.pos.y = entity.boundary.y + entity.radius;
      entity.vel.y = entity.vel.y.abs();
      self.wall_hit = true;
      entity.vel_to_angle();
    }
    if entity.pos.y + entity.radius > entity.boundary.y + entity.boundary.h {
      entity.pos.y = entity.boundary.y + entity.boundary.h - entity.radius;
      entity.vel.y = -(entity.vel.y.abs());
      self.wall_hit = true;
      entity.vel_to_angle();
    }
  }
}

impl EntityLogic for Icicle {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    if self.wall_hit {
      self.timer += props.delta;
      self.entity.friction = 1.0;
      if self.timer > 2500 {
        self.timer = 0;
        self.wall_hit = false;
        self.entity.friction = 0.0;
        self.entity.angle_to_vel();
      }
    }
    self.entity.update(props);

    self.collide();
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
