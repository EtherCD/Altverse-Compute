pub mod area;
pub mod world;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RawWorld {
  pub name: String,
  pub areas: Vec<RawArea>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawArea {
  pub enemies: Vec<RawEntity>,
  pub w: f64,
  pub h: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawEntity {
  pub types: Vec<String>,
  pub radius: f64,
  pub speed: f64,
  pub count: u32,
}
