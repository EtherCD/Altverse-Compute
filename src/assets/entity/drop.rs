use crate::{
  assets::{enemy::Enemy, entity::EnemyWrapper},
  units::{
    entity::Entity,
    random,
    structures::{AdditionalEntityProps, EntityProps},
    vector::Vector,
  },
};

#[derive(Clone)]
pub struct DropEntity {
  entity: Entity,
  time_at_some_surface: f64,
  speed_time: f64,
  start_time: f64,
  spawned: bool,
}

impl DropEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.vel.x = 0.0;
    entity.vel.y = entity.speed;
    Self {
      entity,
      time_at_some_surface: 0.0,
      speed_time: 500.0,
      start_time: 0.0,
      spawned: true,
    }
  }

  fn re_spawn(&mut self) {
    self.entity.pos.x = random(
      self.entity.radius,
      self.entity.boundary.w - self.entity.radius,
    );
    self.entity.pos.y = self.entity.radius + 1.0;
    self.entity.vel = Vector::new(None, None);
    self.time_at_some_surface = random(1000.0, 2000.0);
    self.start_time = self.time_at_some_surface + 0.0;
    self.entity.harmless = true;
  }

  fn collide(&mut self) {
    let entity = &mut self.entity;
    if entity.pos.x - entity.radius < entity.boundary.x {
      entity.pos.x = entity.boundary.x + entity.radius;
      entity.vel.x = entity.vel.x.abs();
    }
    if entity.pos.x + entity.radius > entity.boundary.x + entity.boundary.w {
      entity.pos.x = entity.boundary.x + entity.boundary.w - entity.radius;
      entity.vel.x = -(entity.vel.x.abs());
    }
    if entity.pos.y - entity.radius < entity.boundary.y {
      entity.pos.y = entity.boundary.y + entity.radius;
      entity.vel.y = entity.vel.y.abs();
    }
    if entity.pos.y + entity.radius > entity.boundary.y + entity.boundary.h {
      entity.pos.y -= 1.0;
      entity.vel.y = 0.0;
      self.time_at_some_surface = -1000.0;
      self.start_time = 1000.0;
      self.spawned = false;
    }
  }
}

impl Enemy for DropEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    self.entity.update(props);
    self.collide();

    if self.time_at_some_surface > 0.0 {
      self.time_at_some_surface -= props.delta as f64;
      self.entity.vel.y = 0.0;
      self.entity.alpha = 1.0 - self.time_at_some_surface / self.start_time;
      self.entity.harmless = true;
      if self.time_at_some_surface <= 0.0 {
        self.time_at_some_surface = 0.0;
        self.entity.harmless = false;
        self.entity.vel.y = self.entity.speed;
        self.speed_time = 500.0;
      }
    } else if self.time_at_some_surface < 0.0 {
      self.entity.vel.y = 0.0;
      self.entity.alpha = -self.time_at_some_surface / self.start_time;
      self.time_at_some_surface += props.delta as f64;
      if self.time_at_some_surface >= 0.0 {
        self.re_spawn();
      }
    } else {
      self.speed_time -= props.delta as f64;
      if self.spawned {
        self.entity.vel.y = self.entity.speed;
      } else {
        self.entity.vel.y = self.entity.speed;
        if self.speed_time > 0.0 {
          self.entity.vel.y *= 1.0 - self.speed_time / 500.0;
        }
      }
    }
  }

  fn interact(&mut self, player: &mut crate::units::player::Player) {
    self.entity.interact(player);
  }

  fn pack(&self) -> crate::network::PackedEntity {
    self.entity.pack()
  }

  fn is_to_remove(&self) -> bool {
    self.entity.to_remove
  }

  fn get_nested_entities(&self) -> Vec<EnemyWrapper> {
    return self.entity.nested_entities.clone();
  }

  fn clear_nested_entities(&mut self) {
    self.entity.nested_entities.clear()
  }
}
