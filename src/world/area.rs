use std::collections::HashMap;

use crate::{
  CONFIG,
  network::{NetworkClient, Package, PackedEntity, PackedPlayer},
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
    let id = player.id as usize;
    if let Some(_) = self.players.get(id) {
      self.players.remove(id);
    }
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

  pub fn update(
    &mut self,
    update: &UpdateProps,
    players: &mut HashMap<i64, Player>,
    clients: &mut HashMap<i64, NetworkClient>,
  ) -> Vec<Package> {
    let mut packages: Vec<Package> = Vec::new();

    let old_entities = self.get_entities();
    let old_players = self.get_players(players);

    for (_, entity) in self.entities.iter_mut() {
      for id in self.players.iter() {
        entity.update(update.clone());
        if let Some(player) = players.get_mut(&id) {
          entity.interact(player);
        }
      }
    }

    for id in self.players.iter() {
      if let Some(player) = players.get_mut(id) {
        if let Some(client) = clients.get_mut(id) {
          player.input(&client.input);
        }
        player.update(update);
        player.collide(self.as_boundary_player());
      }
    }

    let new_entities = self.get_entities();
    let new_players = self.get_players(players);

    let mut entities_diffs = HashMap::new();
    let mut players_diffs = HashMap::new();

    for (id, entity) in new_entities {
      if let Some(old_entity) = old_entities.get(&id) {
        let diff = old_entity.diff(&entity);
        if diff.len() > 0 {
          entities_diffs.insert(id, diff);
        }
      }
    }

    for (id, player) in new_players {
      if let Some(old_player) = old_players.get(&id) {
        let diff = player.diff(old_player);
        if diff.len() > 0 {
          players_diffs.insert(id, diff);
        }
      }
    }

    if entities_diffs.len() != 0 {
      packages.push(Package::UpdateEntities(entities_diffs));
    }
    if players_diffs.len() != 0 {
      packages.push(Package::UpdatePlayers(players_diffs));
    }

    packages
  }

  pub fn get_entities(&self) -> HashMap<i64, PackedEntity> {
    let mut res: HashMap<i64, PackedEntity> = HashMap::new();

    for (id, entity) in &self.entities {
      res.insert(*id, entity.pack());
    }

    res
  }

  pub fn get_players(&self, players: &HashMap<i64, Player>) -> HashMap<i64, PackedPlayer> {
    let mut res = HashMap::new();

    for id in &self.players {
      if let Some(player) = players.get(id) {
        res.insert(*id, player.pack());
      }
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

  pub fn as_boundary_player(&self) -> Boundary {
    return Boundary {
      x: -10.0 * 32.0,
      y: 0.0,
      w: self.w + 20.0 * 32.0,
      h: self.h,
    };
  }
}
