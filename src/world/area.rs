use std::collections::HashMap;

use crate::{
  assets::entity::EnemyWrapper,
  network::{NetworkClient, Package, PackedEntity, PackedPlayer},
  units::{
    player::Player,
    random,
    structures::{
      AdditionalEntityProps, Boundary, EntityProps, EntityUpdateProps, UpdateProps, distance,
    },
  },
  world::RawArea,
};

pub struct Area {
  pub entities: HashMap<i64, EnemyWrapper>,
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

  pub fn leave(&mut self, id: i64) {
    self.players.retain(|&item| item != id);
    if self.players.len() == 0 {
      self.entities.clear();
    }
  }

  pub fn init(&mut self) {
    if self.props.enemies.len() != 0 {
      for entity in &self.props.enemies {
        let props = EntityProps {
          type_id: 0,
          radius: entity.radius,
          speed: entity.speed,
          boundary: Boundary {
            x: 0.0,
            y: 0.0,
            w: self.w,
            h: self.h,
          },
        };
        for num in 0..entity.count {
          let type_name = entity
            .types
            .get(random(0.0, entity.types.len() as f64 - 1.0).round() as usize)
            .unwrap();
          let additional = AdditionalEntityProps {
            count: entity.count as u64,
            num: num as u64,
            inverse: false,
          };

          if let Ok(entity) = EnemyWrapper::new(type_name.as_str(), &mut props.clone(), additional)
          {
            self.entities.insert(self.next_id, entity);
            self.next_id += 1;
          }
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

    let old_entities = self.get_enemies();
    let old_players = self.get_players(players);

    let entity_update = EntityUpdateProps {
      delta: update.delta,
      time_fix: update.time_fix,
      players: self.get_players_vec(players),
    };

    let mut new_entities_created = HashMap::new();
    let mut nested_enemies = Vec::new();
    let mut entities_to_remove = Vec::new();

    for (_, entity) in self.entities.iter_mut() {
      entity.update(&entity_update);
      for nested in entity.get_nested_entities() {
        nested_enemies.push(nested);
      }
      entity.clear_nested_entities();
    }

    for enemy in nested_enemies {
      new_entities_created.insert(self.next_id, enemy.clone().pack());
      self.add_enemy(enemy);
    }

    for (_, entity) in self.entities.iter_mut() {
      for id in self.players.iter() {
        if let Some(player) = players.get_mut(&id) {
          entity.interact(player);
        }
      }
    }

    self.entities.retain(|id, entity| {
      if entity.is_to_remove() {
        entities_to_remove.push(*id);
        false
      } else {
        true
      }
    });

    for id in self.players.iter() {
      if let Some(player) = players.get_mut(id) {
        if let Some(client) = clients.get_mut(id) {
          player.input(&mut client.input);
        }
        player.update(update);
        player.collide(self.as_boundary_player());
      }
    }

    let player_ids: Vec<i64> = self.players.iter().copied().collect();
    for &first_id in &player_ids {
      for &second_id in &player_ids {
        if first_id == second_id {
          continue;
        }

        let can_rescue = {
          let first_player = players.get(&first_id).unwrap();
          let second_player = players.get(&second_id).unwrap();
          second_player.downed
            && distance(
              second_player.pos.x - first_player.pos.x,
              second_player.pos.y - first_player.pos.y,
            ) <= first_player.radius + second_player.radius
        };

        if can_rescue {
          if let Some(second_player) = players.get_mut(&second_id) {
            second_player.res();
          }
        }
      }
    }

    let new_entities = self.get_enemies();
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
        let diff = old_player.diff(&player);
        if diff.len() > 0 {
          players_diffs.insert(id, diff);
        }
      }
    }

    if !&new_entities_created.is_empty() {
      packages.push(Package::NewEntities(new_entities_created))
    }

    for i in entities_to_remove.iter() {
      self.entities.remove(i);
    }

    if !&entities_to_remove.is_empty() {
      packages.push(Package::CloseEntities(entities_to_remove));
    }

    if !entities_diffs.is_empty() {
      packages.push(Package::UpdateEntities(entities_diffs));
    }
    if !players_diffs.is_empty() {
      packages.push(Package::UpdatePlayers(players_diffs));
    }

    packages
  }

  fn add_enemy(&mut self, entity: EnemyWrapper) {
    self.entities.insert(self.next_id, entity);
    self.next_id += 1;
  }

  pub fn get_enemies(&self) -> HashMap<i64, PackedEntity> {
    let mut res: HashMap<i64, PackedEntity> = HashMap::new();

    for (id, entity) in &self.entities {
      res.insert(*id, entity.pack());
    }

    res
  }

  pub fn get_players_vec<'a>(&self, players: &'a HashMap<i64, Player>) -> Vec<&'a Player> {
    let mut arr: Vec<&Player> = Vec::new();

    for id in &self.players {
      if let Some(player) = players.get(id) {
        arr.push(player);
      }
    }

    return arr;
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
