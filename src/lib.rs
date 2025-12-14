#![deny(clippy::all)]

use std::{collections::HashMap, io, sync::Mutex};

use crate::{
  config::Config,
  network::{NetworkClient, Package, PackedPlayer},
  structures::GameProps,
  units::{
    player::Player,
    structures::{InputProps, JoinProps, PlayerProps, UpdateProps},
  },
  world::{warp::Warp, world::World},
};
use chrono::Utc;
use lazy_static::lazy_static;
use lz4_flex::frame::FrameEncoder;
use napi::{
  Env, Error, Status,
  bindgen_prelude::{Function, JsObjectValue, Null, Object, Uint8ArraySlice},
};
use napi_derive::napi;

mod assets;
mod config;
mod network;
mod structures;
mod units;
mod world;

lazy_static! {
  pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

#[napi]
pub struct Game {
  players: HashMap<i64, Player>,
  clients: HashMap<i64, NetworkClient>,
  last_timestamp: i64,
  worlds: HashMap<String, World>,
  config: Config,

  player_death_callback: Option<Function<'static, i64, Null>>,
}

#[napi]
impl Game {
  #[napi(constructor)]
  pub fn new(props: &GameProps) -> Result<Game, Error> {
    let worlds = props.load_worlds();
    let config = match props.load_config() {
      Ok(cfg) => cfg,
      Err(e) => return Err(Error::new(Status::InvalidArg, e.to_string())),
    };
    *CONFIG.lock().unwrap() = config.clone();
    match worlds {
      Ok(worlds) => {
        return Ok(Game {
          players: HashMap::new(),
          last_timestamp: Utc::now().timestamp_millis(),
          worlds: worlds,
          clients: HashMap::new(),
          config,
          player_death_callback: None,
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
      area: self.config.spawn.area as u64,
      world: self.config.spawn.world.clone(),
    });
    let world = self.worlds.get_mut(&player.world).unwrap();
    world.join(&player);

    let new_player_package = Package::NewPlayer(player.pack());
    let area_init_package = Package::AreaInit(world.pack_area(player.area as usize));
    let my_self_package = Package::MySelf(player.pack());
    let players_package = Package::Players(self.get_packed_players());

    self.add_client(props);
    self.send_to_all(new_player_package);
    self.send_to_client(player.id, area_init_package);
    self.send_to_client(player.id, my_self_package);
    self.send_to_client(player.id, players_package);
    self.players.insert(props.id, player);
  }

  #[napi]
  pub fn leave(&mut self, id: i64) {
    if let Some(player) = self.players.get(&id) {
      let world = self.worlds.get_mut(&player.world).unwrap();
      world.leave(player);
    }

    self.clients.remove(&id);
    self.send_to_all(Package::ClosePlayer(id));
    self.players.remove(&id);
  }

  #[napi]
  pub fn on_player_death(&mut self, callback: Function<'static, i64, Null>) {
    self.player_death_callback = Some(callback);
  }

  #[napi]
  pub fn input(&mut self, id: i64, input: &InputProps) -> Result<(), Error> {
    if let Some(client) = self.clients.get_mut(&id) {
      client.input = input.clone();
      return Ok(());
    } else {
      Ok(())
      // Err(Error::new(Status::InvalidArg, "The client will not find"))
    }
  }

  #[napi]
  pub fn update(&mut self, env: &Env) -> Result<Object<'_>, Error> {
    let config = CONFIG.lock().unwrap();

    let time = Utc::now().timestamp_millis();
    let delta = time - self.last_timestamp;
    self.last_timestamp = time;
    let time_fix = delta as f64 / (1000.0 / 30.0);

    let update = UpdateProps { delta, time_fix };

    let mut clients_packages: HashMap<i64, Vec<Package>> = HashMap::new();

    let mut players_to_delete = Vec::new();
    self.warp(&config);

    for (_, world) in self.worlds.iter_mut() {
      for area in world.areas.iter_mut() {
        let packages = area.update(&update, &mut self.players, &mut self.clients);
        let player_ids: Vec<i64> = area.players.iter().copied().collect();

        for id in player_ids {
          clients_packages.insert(id, packages.clone());
          if self.players.get(&id).unwrap().to_delete {
            players_to_delete.push(id);
          }
        }
      }
    }

    for id in players_to_delete {
      self.leave(id);
      if let Some(callback) = &self.player_death_callback {
        if let Err(_) = callback.call(id) {}
      }
    }

    for (id, packages) in clients_packages {
      self.send_to_client_packages(id, packages);
    }

    Ok(self.packages_as_napi(env)?)
  }

  fn packages_as_napi(&mut self, env: &Env) -> Result<Object<'_>, Error> {
    let mut object = Object::new(env)?;

    for (index, client) in self.clients.iter_mut() {
      let key = env.create_string(index.to_string())?;
      if let Ok(_) = serde_json::to_string(&client.packages) {
        let raw = rmp_serde::to_vec_named(&client.packages).unwrap();
        let mut slice: &[u8] = &raw;
        let mut compressor = FrameEncoder::new(Vec::new());
        io::copy(&mut slice, &mut compressor)?;
        if let Ok(buffer) = compressor.finish() {
          let uint8 = Uint8ArraySlice::from_data(env, buffer)?;
          object.set_property(key, uint8)?;
          client.packages.clear();
        }
      }
    }

    Ok(object)
  }

  fn add_client(&mut self, props: &JoinProps) {
    self.clients.insert(
      props.id,
      NetworkClient {
        packages: Vec::new(),
        input: InputProps::new(),
      },
    );
  }

  fn warp(&mut self, config: &Config) {
    let warp_result = Warp::update(&self.worlds, &self.players);

    for (id, change) in &warp_result {
      if let Some(player) = self.players.get_mut(&id) {
        match change {
          world::warp::Change::NextArea => {
            if let Some(world) = self.worlds.get_mut(&player.world) {
              if let Some(area) = world.areas.get_mut(player.area as usize) {
                area.leave(player.id);
              }
              player.area += 1;
              player.pos.x = -8.0 * 32.0 + player.radius;
              let next_area = world.areas.get_mut(player.area as usize).unwrap();
              next_area.join(player);
              let area_init_package = Package::AreaInit(world.pack_area(player.area as usize));
              self.send_to_client(*id, area_init_package);
              let players_package = Package::Players(self.get_packed_players());
              self.send_to_client(*id, players_package);
            }
          }
          world::warp::Change::PrevArea => {
            if let Some(world) = self.worlds.get_mut(&player.world) {
              if let Some(area) = world.areas.get_mut(player.area as usize) {
                area.leave(player.id);
              }
              player.area -= 1;
              let prev_area = world.areas.get_mut(player.area as usize).unwrap();
              prev_area.join(player);
              player.pos.x = prev_area.w + 8.0 * 32.0 - player.radius;
              let area_init_package = Package::AreaInit(world.pack_area(player.area as usize));
              let players_package = Package::Players(self.get_packed_players());
              self.send_to_client(*id, players_package);
              self.send_to_client(*id, area_init_package);
            }
          }
          world::warp::Change::NextWorld => {
            if let Some(prev_world) = self.worlds.get_mut(&player.world) {
              prev_world.leave(player);
            }
            let next_world_name = Game::get_next_world(&config.worlds, &player.world);
            let next_world = self.worlds.get_mut(&next_world_name).unwrap();
            let area = next_world.areas.get_mut(0).unwrap();
            player.world = next_world_name;
            player.pos.y = area.h - player.radius - 2.0 * 32.0;
            next_world.join(player);
            let area_init_package = Package::AreaInit(next_world.pack_area(player.area as usize));
            let players_package = Package::Players(self.get_packed_players());
            self.send_to_client(*id, area_init_package);
            self.send_to_client(*id, players_package);
          }
          world::warp::Change::PrevWorld => {
            if let Some(prev_world) = self.worlds.get_mut(&player.world) {
              prev_world.leave(player);
            }
            let prev_world_name = Game::get_prev_world(&config.worlds, &player.world);
            let prev_world = self.worlds.get_mut(&prev_world_name).unwrap();
            player.world = prev_world_name;
            player.pos.y = player.radius + 2.0 * 32.0;
            prev_world.join(player);
            let area_init_package = Package::AreaInit(prev_world.pack_area(player.area as usize));
            self.send_to_client(*id, area_init_package);
            let players_package = Package::Players(self.get_packed_players());
            self.send_to_client(*id, players_package);
          }
        };
      }
    }
  }

  fn get_next_world(world_names: &Vec<String>, current_world: &String) -> String {
    let current_index = world_names.iter().position(|name| name == current_world);

    match current_index {
      Some(idx) if idx + 1 < world_names.len() => world_names[idx + 1].clone(),
      _ => world_names.get(0).unwrap().clone(),
    }
  }

  fn get_prev_world(world_names: &Vec<String>, current_world: &String) -> String {
    let current_index = world_names.iter().position(|name| name == current_world);

    match current_index {
      Some(idx) if idx > 0 => world_names[idx - 1].clone(),
      _ => world_names.get(world_names.len() - 1).unwrap().clone(),
    }
  }

  fn get_packed_players(&mut self) -> HashMap<i64, PackedPlayer> {
    let mut output = HashMap::new();

    for (id, player) in self.players.iter() {
      output.insert(*id, player.pack());
    }

    output
  }

  fn send_to_client(&mut self, id: i64, package: Package) {
    if let Some(val) = self.clients.get_mut(&id) {
      val.packages.push(package);
    }
  }

  fn send_to_client_packages(&mut self, id: i64, packages: Vec<Package>) {
    if let Some(val) = self.clients.get_mut(&id) {
      for package in packages.iter() {
        val.packages.push(package.clone());
      }
    }
  }

  fn send_to_all(&mut self, package: Package) {
    for (_, client) in self.clients.iter_mut() {
      client.packages.push(package.clone());
    }
  }
}
