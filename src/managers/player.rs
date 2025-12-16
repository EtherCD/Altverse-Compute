use crate::bus::{EventBus, NetworkBus};
use crate::proto::package::Kind;
use crate::proto::{PackedPlayer, PartialPlayer, Players, UpdatePlayersMap};
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::utils::join::JoinProps;
use crate::resources::world::World;
use crate::resources::{PlayerUpdateProps, UpdateProps};
use napi::{Error, Status};
use std::collections::HashMap;

pub struct PlayersManager {
  pub players: HashMap<i64, HeroWrapper>,
  pub start_packages: HashMap<u32, PackedPlayer>,
  pub end_packages: HashMap<u32, PackedPlayer>,
  pub players_diff: HashMap<u32, PartialPlayer>,
  pub players_to_remove: Vec<u32>,
}

impl PlayersManager {
  pub fn new() -> Self {
    Self {
      players: HashMap::new(),
      start_packages: HashMap::new(),
      end_packages: HashMap::new(),
      players_diff: HashMap::new(),
      players_to_remove: Vec::new(),
    }
  }

  pub fn join(
    &mut self,
    player_props: &JoinProps,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
  ) -> Result<(), Error> {
    let hero = HeroWrapper::new("maven", player_props.clone())?;
    let player = hero.player().clone();
    let player_id = hero.player().id;
    let world_name = hero.player().world.clone();

    self.players.insert(player_id, hero);
    if let Some(_) = self.players.get(&player_id) {
      if let Some(world) = worlds.get_mut(&world_name) {
        world.join(&player);

        let packed_player = player.pack();

        network_bus.add_global_package(Kind::NewPlayer(packed_player.clone()));
        network_bus.add_direct_package(
          player_id,
          Kind::AreaInit(world.pack_area(player.area as usize)),
        );

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
    if let Some(hero) = self.players.get(&player_id) {
      let player = hero.player();
      if let Some(world) = worlds.get_mut(&player.world) {
        world.leave(&player);
      }
      self.players.remove(&player_id);
    }
    network_bus.remove_client(player_id);
    network_bus.add_global_package(Kind::ClosePlayer(player_id));
  }

  pub fn snapshot_start(&mut self) {
    self.start_packages = self.pack_players();
  }

  pub fn snapshot_end(&mut self, network_bus: &mut NetworkBus) {
    self.end_packages = self.pack_players();
    self.players_diff.clear();

    for (id, player) in self.end_packages.iter() {
      if let Some(old_player) = self.start_packages.get(&id) {
        let (diff, changed) = old_player.diff(&player);
        if changed {
          self.players_diff.insert(*id, diff);
        }
      }
    }

    if !self.players_diff.is_empty() {
      network_bus.add_global_package(Kind::UpdatePlayers(UpdatePlayersMap {
        items: self.players_diff.clone(),
      }))
    }
  }

  pub fn update_behavior(
    &mut self,
    update_props: &UpdateProps,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
    event_bus: &mut EventBus,
  ) {
    let players_clone = &self.players.clone();

    for (id, hero) in self.players.iter_mut() {
      let player = hero.player();
      if let Some(worlds) = worlds.get_mut(&player.world) {
        if let Some(area) = worlds.areas.get_mut(player.area as usize) {
          let mut update_player_props = PlayerUpdateProps {
            time_fix: update_props.time_fix,
            delta: update_props.delta,
            players: area.get_players_vec(&players_clone),
            event_bus,
          };
          hero.update(&mut update_player_props);
          let boundary = area.as_boundary_player();
          hero.collide(boundary);
          if let Some(client) = network_bus.clients.get_mut(id) {
            hero.input(&mut client.input);
          }
        }
      }
    }

    event_bus.process_players_events(&mut self.players);
  }

  pub fn check_players_to_remove(&mut self) -> Vec<u32> {
    self.players_to_remove.clear();

    for (id, hero) in self.players.iter_mut() {
      if hero.player().to_delete {
        self.players_to_remove.push(*id as u32);
      }
    }

    self.players_to_remove.clone()
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

  pub(crate) fn pack_players(&self) -> HashMap<u32, PackedPlayer> {
    let mut result = HashMap::new();

    for (id, hero) in self.players.iter() {
      result.insert(*id as u32, hero.pack());
    }

    result
  }
}
