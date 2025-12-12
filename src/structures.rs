use std::collections::HashMap;

use napi_derive::napi;
use serde_json::Error;

use crate::{
  config::Config,
  world::{RawWorld, world::World},
};

#[napi]
pub struct GameProps {
  config: String,
  worlds: Vec<String>,
}

#[napi]
impl GameProps {
  #[napi(constructor)]
  pub fn new(config: String, worlds: Vec<String>) -> Self {
    Self { config, worlds }
  }

  pub fn load_config(&self) -> Result<Config, Error> {
    serde_json::from_str::<Config>(&self.config)
  }

  pub fn load_worlds(&self) -> Result<HashMap<String, World>, Error> {
    let mut result: HashMap<String, World> = HashMap::new();
    for world in self.worlds.iter() {
      match serde_json::from_str::<RawWorld>(&world) {
        Ok(e) => {
          let raw = &e;
          result.insert(raw.name.clone(), World::new(raw.clone()));
        }
        Err(e) => return Err(e),
      }
    }
    Ok(result)
  }
}
