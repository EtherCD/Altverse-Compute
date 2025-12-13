use napi_derive::napi;

pub struct PlayerProps {
  pub name: String,
  pub id: i64,
  pub world: String,
  pub area: u64,
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

#[derive(Clone, Debug)]
#[napi]
pub struct InputProps {
  pub left: bool,
  pub right: bool,
  pub up: bool,
  pub down: bool,
  pub shift: bool,
  pub mouse_enable: bool,
  pub mouse_pos_x: f64,
  pub mouse_pos_y: f64,
}

#[napi]
impl InputProps {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      left: false,
      right: false,
      up: false,
      down: false,
      shift: false,
      mouse_enable: false,
      mouse_pos_x: 0.0,
      mouse_pos_y: 0.0,
    }
  }
  #[napi]
  pub fn set_left(&mut self, val: bool) {
    self.left = val;
  }

  #[napi]
  pub fn set_right(&mut self, val: bool) {
    self.right = val;
  }

  #[napi]
  pub fn set_up(&mut self, val: bool) {
    self.up = val;
  }

  #[napi]
  pub fn set_down(&mut self, val: bool) {
    self.down = val;
  }

  #[napi]
  pub fn set_shift(&mut self, val: bool) {
    self.shift = val;
  }

  #[napi]
  pub fn set_mouse_enable(&mut self, val: bool) {
    self.mouse_enable = val;
  }

  #[napi]
  pub fn set_mouse_pos_x(&mut self, val: f64) {
    self.mouse_pos_x = val;
  }

  #[napi]
  pub fn set_mouse_pos_y(&mut self, val: f64) {
    self.mouse_pos_y = val;
  }
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
