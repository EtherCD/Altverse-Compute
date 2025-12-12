use napi_derive::napi;

use crate::units::vector::Vector;

pub struct PlayerProps {
  pub name: String,
  pub id: i64,
  pub world: String,
  pub area: u32,
}

pub struct EntityProps {
  pub type_id: u64,
  pub radius: f64,
  pub speed: f64,
  pub boundary: Boundary,
}

#[napi]
pub struct JoinProps {
  pub name: String,
  pub id: i64,
}

#[napi]
impl JoinProps {
  #[napi(constructor)]
  pub fn new(name: String, id: i64) -> JoinProps {
    JoinProps { name, id }
  }
}

#[derive(Clone)]
pub struct UpdateProps {
  pub delta: i64,
  pub time_fix: f64,
}

pub struct InputProps {
  pub left: Option<bool>,
  pub right: Option<bool>,
  pub up: Option<bool>,
  pub down: Option<bool>,
  pub shift: Option<bool>,
  pub mouse_enable: Option<bool>,
  pub mouse_pos: Option<Vector>,
}

pub fn distance(a: f64, b: f64) -> f64 {
  (a * a + b * b).sqrt()
}

pub struct Boundary {
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64,
}

pub struct GlobalUpdatePackage {
  pub clients: Vec<(i64, String)>,
}
