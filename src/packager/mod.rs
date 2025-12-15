mod diff;

use crate::resources::utils::input::Input;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct NetworkClient {
  pub packages: Vec<Package>,
  pub input: Input,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PackedPlayer {
  pub id: i64,
  pub name: String,
  pub x: f64,
  pub y: f64,
  pub radius: f64,
  pub speed: f64,
  pub energy: f64,
  pub max_energy: f64,
  pub death_timer: f64,
  pub state: u64,
  pub state_meta: f64,
  pub area: u64,
  pub world: String,
  pub died: bool,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PackedEntity {
  pub type_id: u64,
  pub x: f64,
  pub y: f64,
  pub radius: f64,
  pub harmless: bool,
  pub aura: f64,
  pub state: u64,
  pub state_metadata: f64,
  pub alpha: f64,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PackedArea {
  pub w: f64,
  pub h: f64,
  pub area: u32,
  pub world: String,
  pub entities: HashMap<i64, PackedEntity>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Package {
  NewPlayer(PackedPlayer),
  ClosePlayer(i64),
  Players(HashMap<i64, PackedPlayer>),
  NewEntities(HashMap<i64, PackedEntity>),
  CloseEntities(Vec<i64>),
  UpdatePlayers(HashMap<i64, HashMap<String, serde_json::Value>>),
  UpdateEntities(HashMap<i64, HashMap<String, serde_json::Value>>),
  AreaInit(PackedArea),
  MySelf(PackedPlayer),
}
