#![deny(clippy::all)]

use crate::bus::PackagesBus;
use crate::config::Config;
use crate::managers::player::PlayersManager;
use crate::managers::world::WorldsManager;
use crate::props::EngineProps;
use crate::resources::utils::join::JoinProps;
use crate::resources::UpdateProps;
use lazy_static::lazy_static;
use napi::bindgen_prelude::Object;
use napi::{Env, Error};
use napi_derive::napi;
use std::sync::Mutex;

mod bus;
mod config;
mod managers;
mod packager;
mod props;
mod resources;

lazy_static! {
  pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

pub struct ComputeEngine {
  players_manager: PlayersManager,
  worlds_manager: WorldsManager,
  packages_bus: PackagesBus,

  last_timestamp: i64,
}

impl ComputeEngine {
  pub fn new(props: &EngineProps) -> Result<Self, Error> {
    let worlds = props.load_worlds()?;
    let config = props.load_config()?;

    *CONFIG.lock().unwrap() = config.clone();

    Ok(Self {
      players_manager: PlayersManager::new(),
      worlds_manager: WorldsManager::new(props),
      worlds,
      packages_bus: PackagesBus::new(),
    })
  }

  pub fn join(&mut self, player_props: &JoinProps) -> Result<(), Error> {
    self.players_manager.join(
      player_props.clone(),
      &mut self.worlds_manager.worlds,
      &mut self.packages_bus,
    )
  }

  pub fn leave(&mut self, player_id: i64) {
    self
      .players_manager
      .leave(player_id, &mut self.packages_bus);
  }

  pub fn update(&mut self, env: &Env) -> Result<Object<'_>, Error> {
    let config = CONFIG.lock().unwrap();

    let time = Utc::now().timestamp_millis();
    let delta = time - self.last_timestamp;
    self.last_timestamp = time;
    let time_fix = delta as f64 / (1000.0 / 30.0);

    let update_props = UpdateProps { delta, time_fix };

    let players

    self
      .players_manager
      .update_behavior(&update_props, &mut self.worlds_manager.worlds);
    self
      .worlds_manager
      .update(&update_props, &mut self.players_manager.players);



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
}
