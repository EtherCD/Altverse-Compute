use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
  pub spawn: Spawn,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Spawn {
  pub radius: f64,
  pub speed: f64,
  pub max_speed: f64,
  pub regeneration: f64,
  pub energy: f64,
  pub max_energy: i64,
  pub world: String,
  pub area: i64,

  pub sx: f64,
  pub sy: f64,
  pub ex: f64,
  pub ey: f64,

  pub died_timer: f64,
}

impl Config {
  pub fn new() -> Self {
    Self {
      spawn: Spawn {
        radius: 15.0,
        speed: 17.0,
        max_speed: 17.0,
        regeneration: 7.0,
        energy: 30.0,
        max_energy: 30,
        world: "".to_string(),
        area: 0,
        sx: -(10.0 * 32.0 - 155.0),
        sy: 25.0 * 32.0 + 15.0,
        ex: -15.0,
        ey: 15.0 * 32.0 - 15.0 - 2.0 * 32.0,
        died_timer: 60.0,
      },
    }
  }
}
