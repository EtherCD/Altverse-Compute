use napi::{Error, Status};
use napi_derive::napi;
use std::collections::HashMap;

use crate::{
  config::Config,
  world::{world::World, RawWorld},
};

#[napi]
pub struct EngineProps {
  config: String,
  worlds: Vec<String>,
}

#[napi]
impl EngineProps {
  #[napi(constructor)]
  pub fn new(config: String, worlds: Vec<String>) -> Self {
    Self { config, worlds }
  }

  pub fn load_config(&self) -> Result<Config, Error> {
    match serde_json::from_str::<Config>(&self.config) {
      Ok(config) => Ok(config),
      Err(e) => Err(Error::new(
        Status::InvalidArg,
        "Attempt to read invalid json config ".to_string() + e.to_string().as_str(),
      )),
    }
  }

  pub fn load_worlds(&self) -> Result<HashMap<String, World>, Error> {
    let mut result: HashMap<String, World> = HashMap::new();
    for world in self.worlds.iter() {
      return match serde_json::from_str::<RawWorld>(&world) {
        Ok(e) => {
          let raw = &e;
          result.insert(raw.name.clone(), World::new(raw.clone()));
        }
        Err(e) => Err(Error::new(
          Status::InvalidArg,
          "Attempt to read invalid json world".to_string() + e.to_string().as_str(),
        )),
      };
    }
    Ok(result)
  }
}
