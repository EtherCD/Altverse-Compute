use std::collections::HashMap;

use crate::{
  network::PackedEntity,
  units::{
    entity::Entity,
    player::Player,
    // random,
    structures::{Boundary, EntityProps, UpdateProps},
  },
  world::RawArea,
};

pub struct Area {
  pub entities: HashMap<i64, Entity>,
  next_id: i64,
  pub w: f64,
  pub h: f64,
  pub players: Vec<i64>,
  props: RawArea,
}

impl Area {
  pub fn new(props: RawArea) -> Self {
    Self {
      entities: HashMap::new(),
      next_id: 0,
      w: props.w,
      h: props.h,
      players: Vec::new(),
      props: props,
    }
  }

  pub fn join(&mut self, player: &Player) {
    if self.players.len() == 0 {
      self.init();
    }
    self.players.push(player.id);
  }

  pub fn leave(&mut self, player: &Player) {
    self.players.remove(player.id as usize);
    if self.players.len() == 0 {
      self.entities = HashMap::new();
    }
  }

  pub fn init(&mut self) {
    if self.props.enemies.len() != 0 {
      for entity in &self.props.enemies {
        for _ in 0..entity.count {
          // let type_name = entity
          //   .types
          //   .get(random(0.0, entity.types.len() as f64 - 1.0).round() as usize);
          self.entities.insert(
            self.next_id,
            Entity::new(EntityProps {
              type_id: 0,
              radius: entity.radius,
              speed: entity.speed,
              boundary: Boundary {
                x: 0.0,
                y: 0.0,
                w: self.w,
                h: self.h,
              },
            }),
          );
          self.next_id += 1;
        }
      }
    }
  }

  pub fn update(&mut self, update: &UpdateProps) {
    for (_, entity) in &mut self.entities {
      entity.update(update.clone());
    }
  }

  pub fn get_entities(&self) -> HashMap<i64, PackedEntity> {
    let mut res: HashMap<i64, PackedEntity> = HashMap::new();

    for (id, entity) in &self.entities {
      res.insert(*id, entity.pack());
    }

    res
  }

  pub fn as_boundary(&self) -> Boundary {
    return Boundary {
      x: 0.0,
      y: 0.0,
      w: self.w,
      h: self.h,
    };
  }
}
