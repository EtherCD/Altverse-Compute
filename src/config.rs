use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub spawn: Spawn,
}

#[derive(Serialize, Deserialize)]
pub struct Spawn {
  pub radius: f64,
  pub speed: f64,
  pub max_speed: f64,
  pub regeneration: f64,
  pub energy: f64,
  pub max_energy: f64,
  pub world: String,
  pub area: i64,

  pub sx: f64,
  pub sy: f64,
  pub ex: f64,
  pub ey: f64,

  pub died_timer: f64,
}
