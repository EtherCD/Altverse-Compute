// #[deny(clippy::all)]
use crate::bus::{EventBus, NetworkBus};
use crate::config::Config;
use crate::managers::player::PlayersManager;
use crate::managers::world::WorldsManager;
use crate::props::EngineProps;
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::UpdateProps;
use chrono::Utc;
use lazy_static::lazy_static;
use napi::bindgen_prelude::Function;
use napi::bindgen_prelude::Null;
use napi::bindgen_prelude::{JsObjectValue, Object, Uint8ArraySlice};
use napi::{Env, Error};
use napi_derive::napi;
use std::io;
use std::sync::Mutex;

pub mod proto {
  include!(concat!(env!("OUT_DIR"), "/game.rs"));
}

mod bus;
mod config;
mod managers;
mod props;
mod resources;

lazy_static! {
  pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

#[napi]
pub struct ComputeEngine {
  players_manager: PlayersManager,
  worlds_manager: WorldsManager,
  network_bus: NetworkBus,
  event_bus: EventBus,
  proto_buffer: Vec<u8>,

  last_timestamp: i64,
  player_death_callback: Option<Function<'static, i64, Null>>,
}

#[napi]
impl ComputeEngine {
  #[napi(constructor)]
  pub fn new(props: &EngineProps) -> Result<Self, Error> {
    // let worlds = props.load_worlds()?;
    let config = props.load_config()?;

    *CONFIG.lock().unwrap() = config.clone();

    Ok(Self {
      players_manager: PlayersManager::new(),
      worlds_manager: WorldsManager::new(props),
      network_bus: NetworkBus::new(),
      last_timestamp: Utc::now().timestamp_millis(),
      proto_buffer: Vec::with_capacity(1024),
      event_bus: EventBus::new(),
      player_death_callback: None,
    })
  }

  #[napi]
  pub fn join(&mut self, player_props: &JoinProps) -> Result<(), Error> {
    self.network_bus.add_client(player_props.id);
    self.players_manager.join(
      player_props,
      &mut self.worlds_manager.worlds,
      &mut self.network_bus,
    )?;
    Ok(())
  }

  #[napi]
  pub fn leave(&mut self, player_id: i64) {
    self.players_manager.leave(
      player_id,
      &mut self.worlds_manager.worlds,
      &mut self.network_bus,
    );
    self.network_bus.remove_client(player_id);
  }

  #[napi]
  pub fn input(&mut self, id: i64, input: &Input) {
    self.network_bus.accept_input(id, input);
  }

  #[napi]
  pub fn on_player_death(&mut self, callback: Function<'static, i64, Null>) {
    self.player_death_callback = Some(callback);
  }

  #[napi]
  pub fn update(&mut self, env: &Env) -> Result<Object<'_>, Error> {
    let config = CONFIG.lock().unwrap();

    let time = Utc::now().timestamp_millis();
    let delta = time - self.last_timestamp;
    self.last_timestamp = time;
    let time_fix = delta as f64 / (1000.0 / 30.0);

    let update_props = UpdateProps { delta, time_fix };

    self.players_manager.snapshot_start();
    self.worlds_manager.update(
      &update_props,
      &mut self.players_manager,
      &mut self.network_bus,
      &mut self.event_bus,
    );
    self
      .worlds_manager
      .process_warps(&mut self.players_manager, &config, &mut self.network_bus);
    self.players_manager.update_behavior(
      &update_props,
      &mut self.worlds_manager.worlds,
      &mut self.network_bus,
      &mut self.event_bus,
    );
    self.players_manager.snapshot_end(&mut self.network_bus);

    if let Some(callback) = self.player_death_callback {
      for id in self.players_manager.check_players_to_remove() {
        self.leave(id as i64);
        let _ = callback.call(id as i64);
      }
    }

    self.packages_as_napi(env)
  }

  fn packages_as_napi(&mut self, env: &Env) -> Result<Object<'_>, Error> {
    let mut object = Object::new(env)?;

    for (index, client) in self.network_bus.clients.iter_mut() {
      if client.packages.items.len() != 0 {
        let key = env.create_string(index.to_string())?;
        self.proto_buffer.clear();
        if let Ok(_) = prost::Message::encode(&client.packages, &mut self.proto_buffer) {
          let slice = self.proto_buffer.as_slice();
          // if let Ok(buffer) = encoder.compress_vec(slice) {
          let uint8 = Uint8ArraySlice::from_data(env, slice)?;
          object.set_property(key, uint8)?;
          // }
        }
      }
    }

    self.network_bus.clear_packages();

    Ok(object)
  }
}
