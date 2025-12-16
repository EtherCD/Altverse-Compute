use crate::proto::package::Kind;
use crate::proto::{Package, Packages};
use crate::resources::utils::input::Input;
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
    for mut client in self.clients.values_mut() {
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

enum Interact {
  KNOK,
}

pub struct InteractBus {
  events: HashMap<i64, Vec<Interact>>,
}

impl InteractBus {
  pub fn new() -> Self {
    Self {
      events: HashMap::new(),
    }
  }

  pub fn push_event(&mut self, event: Interact) {
    // self.events.push(event);
  }
}
