use crate::proto::package::Kind;
use crate::proto::{Package, Packages};
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::utils::input::Input;
use crate::resources::utils::vector::Vector;
use std::collections::HashMap;

pub struct Client {
  pub packages: Packages,
  pub input: Input,
}

pub struct NetworkBus {
  pub clients: HashMap<i64, Client>,
}

impl NetworkBus {
  pub fn new() -> Self {
    Self {
      clients: HashMap::new(),
    }
  }

  pub fn add_client(&mut self, player_id: i64) {
    self.clients.insert(
      player_id,
      Client {
        input: Input::new(),
        packages: Packages { items: Vec::new() },
      },
    );
  }

  pub fn remove_client(&mut self, player_id: i64) {
    if let Some(_) = self.clients.get(&player_id) {
      self.clients.remove(&player_id);
    }
  }

  pub fn accept_input(&mut self, id: i64, input: &Input) {
    if let Some(client) = self.clients.get_mut(&id) {
      client.input = input.clone();
    }
  }

  pub fn add_global_package(&mut self, package: Kind) {
    for client in self.clients.values_mut() {
      client.packages.items.push(Package {
        kind: Some(package.clone()),
      });
    }
  }

  pub fn add_direct_package(&mut self, id: i64, package: Kind) {
    if let Some(client) = self.clients.get_mut(&id) {
      client.packages.items.push(Package {
        kind: Some(package),
      });
    }
  }

  pub fn clear_packages(&mut self) {
    for (_, client) in self.clients.iter_mut() {
      client.packages.items.clear();
    }
  }
}

#[derive(Clone)]
pub enum PlayerEvent {
  ResPlayerAndMove { player_id: i64, pos: Vector },
}

pub struct EventBus {
  pub entities_to_spawn: Vec<EntityWrapper>,
  pub players_events: Vec<PlayerEvent>,
}

impl EventBus {
  pub fn new() -> Self {
    Self {
      entities_to_spawn: Vec::new(),
      players_events: Vec::new(),
    }
  }

  pub fn add_entity(&mut self, entity: EntityWrapper) {
    self.entities_to_spawn.push(entity);
  }

  pub fn respawn_player_and_move(&mut self, player_id: i64, pos: Vector) {
    self
      .players_events
      .push(PlayerEvent::ResPlayerAndMove { player_id, pos });
  }

  pub fn process_players_events(&mut self, players: &mut HashMap<i64, HeroWrapper>) {
    for event in self.players_events.iter() {
      match event {
        PlayerEvent::ResPlayerAndMove { player_id, pos } => {
          if let Some(player) = players.get_mut(player_id) {
            player.res();
            // player.player_mut().pos.x = pos.x;
            // player.player_mut().pos.y = pos.y;
          }
        }
      }
    }
  }
}
