#![deny(clippy::all)]

use std::collections::HashMap;

use crate::{
  config::Config,
  network::{NetworkClient, Package, PackedPlayer},
  structures::GameProps,
  units::{
    player::Player,
    structures::{JoinProps, PlayerProps, UpdateProps},
  },
  world::world::World,
};
use chrono::Utc;
use napi::{Error, Status};
use napi_derive::napi;

mod config;
mod network;
mod structures;
mod units;
mod world;

#[napi]
pub struct Game {
  players: HashMap<i64, Player>,
  clients: HashMap<i64, NetworkClient>,
  last_timestamp: i64,
  worlds: HashMap<String, World>,
  config: Config,
}

#[napi]
impl Game {
  #[napi(constructor)]
  pub fn new(props: &GameProps) -> Result<Game, Error> {
    let worlds = props.load_worlds();
    match worlds {
      Ok(worlds) => {
        return Ok(Game {
          players: HashMap::new(),
          last_timestamp: Utc::now().timestamp_millis(),
          worlds: worlds,
          clients: HashMap::new(),
          config: match props.load_config() {
            Ok(cfg) => cfg,
            Err(e) => return Err(Error::new(Status::InvalidArg, e.to_string())),
          },
        });
      }
      Err(e) => Err(Error::new(Status::InvalidArg, e.to_string())),
    }
  }

  #[napi]
  pub fn join(&mut self, props: &JoinProps) {
    let player = Player::new(PlayerProps {
      name: props.name.clone(),
      id: props.id,
      area: self.config.spawn.area as u32,
      world: self.config.spawn.world.clone(),
    });
    let world = self.worlds.get_mut(&player.world).unwrap();
    world.join(&player);

    self.add_client(props);
    let new_player_package = Package::NewPlayer(player.pack());
    self.send_to_all(new_player_package);
    // let area_init_package = Package::AreaInit(world.pack_area(player.area as usize));
    // self.send_to_client(player.id, area_init_package);
    self.players.insert(props.id, player);
  }

  #[napi]
  pub fn leave(&mut self, id: i64) {
    self.send_to_all(Package::ClosePlayer(id));
    self.players.remove(&id);
    self.clients.remove(&id);
  }

  fn add_client(&mut self, props: &JoinProps) {
    self.clients.insert(
      props.id,
      NetworkClient {
        packages: Vec::new(),
      },
    );
  }

  #[napi]
  pub fn update(&mut self) {
    let time = Utc::now().timestamp_millis();
    let delta = time - self.last_timestamp;
    self.last_timestamp = time;
    let time_fix = delta as f64 / (1000.0 / 30.0);

    println!("{}", delta);

    let update = UpdateProps { delta, time_fix };

    for (_, world) in self.worlds.iter_mut() {
      world.interact(&mut self.players);
    }

    for (_, player) in self.players.iter_mut() {
      player.update(&update);
      let area = self
        .worlds
        .get_mut(&player.world)
        .unwrap()
        .areas
        .get(player.area as usize)
        .unwrap();
      player.collide(area.as_boundary());
    }

    let package = self.get_packed_players();

    self.send_to_all(Package::UpdatePlayers(package));

    // let players_diff = serde_diff::Diff::serializable(&old, &new_packed_players);

    // let a = assert_eq!(self.old_packed_players, new_packed_players);

    // return serde_json::to_string_pretty(&players_diff).unwrap();
  }

  #[napi]
  pub fn get_package_per_player(&mut self, id: i64) -> Result<String, Error> {
    if let Some(val) = self.clients.get_mut(&id) {
      let response = Ok(serde_json::to_string(&val.packages).unwrap());
      val.packages.clear();
      return response;
    }
    Err(Error::new(Status::GenericFailure, "Client is not exists!"))
  }

  pub fn get_packed_players(&mut self) -> HashMap<i64, PackedPlayer> {
    let mut array: HashMap<i64, PackedPlayer> = HashMap::new();
    for (id, player) in self.players.iter_mut() {
      array.insert(*id, player.pack());
    }
    return array;
  }

  fn send_to_client(&mut self, id: i64, package: Package) {
    if let Some(val) = self.clients.get_mut(&id) {
      val.packages.push(package);
    }
  }

  fn send_to_all(&mut self, package: Package) {
    for (_, client) in self.clients.iter_mut() {
      client.packages.push(package.clone());
    }
  }
}
