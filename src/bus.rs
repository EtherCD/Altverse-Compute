use crate::packager::Package;
use std::collections::HashMap;

pub struct PackagesBus {
  packages: HashMap<i64, Vec<Package>>,
  players_id: Vec<i64>,
}

impl PackagesBus {
  pub fn new() -> Self {
    Self {
      packages: HashMap::new(),
      players_id: Vec::new(),
    }
  }

  pub fn add_global_package(&mut self, package: Package) {
    for (id, packages) in self.packages.iter_mut() {
      packages.push(package.clone());
    }
  }

  pub fn add_direct_package(&mut self, id: i64, package: Package) {
    if let Some(packages) = self.packages.get_mut(&id) {
      packages.push(package);
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
    self.events.push(event);
  }
}
