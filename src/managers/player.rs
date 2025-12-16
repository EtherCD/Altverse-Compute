use crate::bus::NetworkBus;
use crate::proto::package::Kind;
use crate::proto::{PackedPlayer, PartialPlayer, Players, UpdatePlayersMap};
use crate::resources::player::Player;
use crate::resources::utils::join::JoinProps;
use crate::resources::world::World;
use crate::resources::UpdateProps;
use napi::{Error, Status};
use std::collections::HashMap;

pub struct PlayersManager {
  pub players: HashMap<i64, Player>,
  pub start_packages: HashMap<i64, PackedPlayer>,
  pub end_packages: HashMap<i64, PackedPlayer>,
  pub players_diff: HashMap<i64, PartialPlayer>,
}

impl PlayersManager {
  pub fn new() -> Self {
    Self {
      players: HashMap::new(),
      start_packages: HashMap::new(),
      end_packages: HashMap::new(),
      players_diff: HashMap::new(),
    }
  }

  pub fn join(
    &mut self,
    player_props: &JoinProps,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
  ) -> Result<(), Error> {
    let player = Player::new(player_props.clone());
    let player_id = player.id;
    let world_name = player.world.clone();

    self.players.insert(player_id, player);
    // Теперь можем безопасно получить ссылку
    if let Some(player_ref) = self.players.get(&player_id) {
      if let Some(world) = worlds.get_mut(&world_name) {
        world.join(player_ref);

        let packed_player = player_ref.pack();

        network_bus.add_global_package(Kind::NewPlayer(packed_player.clone()));
        network_bus.add_direct_package(
          player_id,
          Kind::AreaInit(world.pack_area(player_ref.area as usize)),
        );

        // Проблема в pack_players() - она возвращает HashMap с ссылками
        // Решение: создать owned версию
        let players = self.pack_players();

        network_bus.add_direct_package(player_id, Kind::Players(Players { players }));

        network_bus.add_direct_package(player_id, Kind::Myself(packed_player.clone()));

        return Ok(());
      }
    }

    Err(Error::new(
      Status::InvalidArg,
      format!("World not found: {}", world_name),
    ))
  }

  pub fn leave(
    &mut self,
    player_id: i64,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
  ) {
    if let Some(player) = self.players.get(&player_id) {
      if let Some(world) = worlds.get_mut(&player.world) {
        world.leave(&player);
      }
      self.players.remove(&player_id);
    }
    network_bus.remove_client(player_id);
    network_bus.add_global_package(Kind::ClosePlayer(player_id));
  }

  pub fn update_behavior(
    &mut self,
    update_props: &UpdateProps,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
  ) {
    self.start_packages = self.pack_players();
    if self.players_diff.len() > 0 {
      self.players_diff.clear();
    }

    for (id, player) in self.players.iter_mut() {
      if let Some(worlds) = worlds.get_mut(&player.world) {
        if let Some(area) = worlds.areas.get_mut(player.area as usize) {
          player.update(update_props);
          let boundary = area.as_boundary_player();
          player.collide(boundary);
          if let Some(client) = network_bus.clients.get_mut(id) {
            player.input(&mut client.input);
          }
        }
      }
    }

    self.end_packages = self.pack_players();

    for (id, player) in self.end_packages.iter() {
      if let Some(old_player) = self.start_packages.get(&id) {
        let diff = old_player.diff(&player);
        self.players_diff.insert(*id, diff);
      }
    }

    if !self.players_diff.is_empty() {
      network_bus.add_global_package(Kind::UpdatePlayers(UpdatePlayersMap {
        items: self.players_diff.clone(),
      }))
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
