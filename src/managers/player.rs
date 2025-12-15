use crate::bus::PackagesBus;
use crate::packager::{Package, PackedPlayer};
use crate::resources::player::Player;
use crate::resources::utils::join::JoinProps;
use crate::resources::world::World;
use crate::resources::UpdateProps;
use napi::{Error, Status};
use std::collections::HashMap;

pub struct PlayersManager {
  pub players: HashMap<i64, Player>,
}

impl PlayersManager {
  pub fn new() -> Self {
    Self {
      players: HashMap::new(),
    }
  }

  pub fn join(
    &mut self,
    player_props: JoinProps,
    worlds: &mut HashMap<String, World>,
    packages_bus: &mut PackagesBus,
  ) -> Result<(), Error> {
    let player = Player::new(player_props);
    if let Some(world) = worlds.get_mut(&player.world) {
      world.join(&player);

      packages_bus.add_global_package(Package::NewPlayer(player.pack()));
      packages_bus.add_direct_package(
        player.id,
        Package::AreaInit(world.pack_area(player.area as usize)),
      );
      packages_bus.add_direct_package(player.id, Package::Players(self.pack_players()));
      packages_bus.add_direct_package(player.id, Package::MySelf(player.pack()));

      self.players.insert(player.id, player.clone());

      return Ok(());
    }
    Err(Error::new(
      Status::InvalidArg,
      "The world specified in the configuration does not exist.".to_string()
        + player.world.as_str(),
    ))
  }

  pub fn leave(&mut self, player_id: i64, packages_bus: &mut PackagesBus) {
    if let Some(player) = self.players.get(&player_id) {
      self.players.remove(&player_id);
    }
    packages_bus.add_global_package(Package::ClosePlayer(player_id));
  }

  pub fn update_behavior(
    &mut self,
    update_props: &UpdateProps,
    worlds: &mut HashMap<String, World>,
  ) {
    for (_, player) in self.players.iter_mut() {
      if let Some(worlds) = worlds.get_mut(&player.world) {
        if let Some(area) = worlds.areas.get_mut(&player.area as usize) {
          player.update(update_props);
          let boundary = area.as_boundary_player();
          player.collide(boundary);
        }
      }
    }
  }

  // pub fn update_interact(
  //   &mut self,
  //   update_props: &UpdateProps,
  //   worlds: &mut HashMap<String, World>,
  // ) {
  //   for (_, player) in self.players.iter_mut() {
  //     if let Some(worlds) = worlds.get_mut(&player.world) {
  //       if let Some(area) = worlds.areas.get_mut(&player.area as usize) {
  //         for (_, ent) in area.entities.iter_mut() {}
  //       }
  //     }
  //   }
  // }

  pub(crate) fn pack_players(&self) -> HashMap<i64, PackedPlayer> {
    let mut result = HashMap::new();

    for (id, player) in self.players.iter() {
      result.insert(*id, player.pack());
    }

    result
  }
}
