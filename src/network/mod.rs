use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

pub struct NetworkClient {
  pub packages: Vec<Package>,
}

#[derive(SerdeDiff, Serialize, Deserialize, Debug, Clone)]
pub struct PackedPlayer {
  pub id: i64,
  pub name: String,
  pub x: f64,
  pub y: f64,
  pub radius: f64,
  pub speed: f64,
  pub energy: f64,
  pub max_energy: i16,
  pub death_timer: f64,
  pub state: u32,
  pub state_meta: f64,
  pub area: u32,
  pub world: String,
  pub died: bool,
}

#[derive(SerdeDiff, Serialize, Deserialize, Debug, Clone)]
pub struct PackedEntity {
  pub type_id: u64,
  pub x: f64,
  pub y: f64,
  pub radius: f64,
  pub harmless: bool,
  pub state: u64,
  pub state_metadata: f64,
  pub alpha: f64,
}

#[derive(SerdeDiff, Serialize, Deserialize, Debug, Clone)]
pub struct PackedArea {
  pub w: f64,
  pub h: f64,
  pub area: u32,
  pub world: String,
  pub entities: HashMap<i64, PackedEntity>,
}

#[derive(SerdeDiff, Serialize, Deserialize, Clone)]
pub enum Package {
  NewPlayer(PackedPlayer),
  ClosePlayer(i64),
  UpdatePlayers(HashMap<i64, PackedPlayer>),
  UpdateEntities(HashMap<i64, PackedEntity>),
  AreaInit(PackedArea),
}
