use crate::{
  network::PackedPlayer,
  units::{structures::*, vector::Vector},
};

#[derive(Clone)]
pub struct Player {
  pub name: String,
  pub id: i64,
  pub pos: Vector,
  pub radius: f64,
  pub vel: Vector,
  acc: Vector,
  slide: Vector,
  pub speed: f64,
  pub energy: f64,
  pub max_energy: i16,
  pub downed: bool,
  pub regeneration: f32,
  pub world: String,
  pub area: u32,
  angle: f64,
  death_timer: f64,

  pub immortal: bool,
  state: u32,
  state_meta: f64,
}

impl Player {
  pub fn new(props: PlayerProps) -> Player {
    Player {
      name: props.name,
      id: props.id,
      pos: Vector::new(None, None),
      radius: 15.0,
      vel: Vector::new(None, None),
      acc: Vector::new(None, None),
      slide: Vector::new(None, None),
      speed: 17.0,
      energy: 30.0,
      max_energy: 30,
      downed: false,
      regeneration: 1.0,
      angle: 0.0,
      death_timer: 60.0,
      immortal: false,
      state: 0,
      state_meta: 0.0,
      world: props.world,
      area: props.area,
    }
  }

  pub fn update(&mut self, props: &UpdateProps) {
    let time_fix = props.time_fix;

    let mut slide = [self.slide.x, self.slide.y];

    let dim = 1.0 - 0.75;

    slide[0] *= 1.0 - (1.0 - dim) * time_fix;
    slide[1] *= 1.0 - (1.0 - dim) * time_fix;

    self.acc.x *= time_fix;
    self.acc.y *= time_fix;

    self.acc.x += slide[0];
    self.acc.y += slide[1];

    self.vel.x = self.acc.x;
    self.vel.y = self.acc.y;

    if self.downed {
      self.vel.x = 0.0;
      self.vel.y = 0.0;
    }

    self.pos.x += self.vel.x * time_fix;
    self.pos.y += self.vel.y * time_fix;

    self.pos.x = (self.pos.x * 100.0).round() / 100.0;
    self.pos.y = (self.pos.y * 100.0).round() / 100.0;

    self.slide.x = self.acc.x;
    self.slide.y = self.acc.y;
    self.acc = Vector::new(None, None);
  }

  pub fn input(&mut self, input: InputProps) {
    let shift: f64 = if input.shift.unwrap() { 0.5 } else { 1.0 };

    if input.left.unwrap() {
      self.acc.x = -self.speed * shift;
    }
    if input.right.unwrap() {
      self.acc.x = self.speed * shift;
    }
    if input.up.unwrap() {
      self.acc.y = -self.speed * shift;
    }
    if input.down.unwrap() {
      self.acc.y = -self.speed * shift;
    }
    if input.mouse_enable.unwrap() {
      match input.mouse_pos {
        Some(e) => {
          let dist = distance(e.x, e.y);
          let mut speed_x = e.x;
          let mut speed_y = e.y;

          if dist > 150.0 {
            speed_x = e.x * (150.0 / dist);
            speed_y = e.y * (150.0 / dist);
          }

          self.angle = speed_y.atan2(speed_x);

          let mouse_dist = (e.x.powf(2.0) - e.y.powf(2.0)).sqrt().min(150.0);

          let mut dist_movement = self.speed * shift;
          dist_movement *= mouse_dist / 150.0;

          self.acc.x = dist_movement * self.angle.cos();
          self.acc.y = dist_movement * self.angle.sin();
        }
        None => {}
      }
    }
  }

  pub fn knock(&mut self) {
    self.downed = true;
    self.death_timer = 60.0;
  }

  pub fn res(&mut self) {
    self.downed = false;
  }

  pub fn collide(&mut self, boundary: Boundary) {
    if self.pos.x - self.radius < boundary.x {
      self.pos.x = self.radius - boundary.x;
    }
    if self.pos.y - self.radius < boundary.y {
      self.pos.y = self.radius - boundary.y;
    }
    if self.pos.x + self.radius > boundary.x + boundary.w {
      self.pos.x = self.radius + boundary.x;
    }
    if self.pos.y + self.radius > boundary.y + boundary.h {
      self.pos.y = self.radius + boundary.y;
    }
  }

  pub fn pack(&self) -> PackedPlayer {
    PackedPlayer {
      id: self.id,
      name: self.name.clone(),
      x: self.pos.x,
      y: self.pos.y,
      radius: self.radius,
      speed: self.speed,
      energy: self.energy,
      max_energy: self.max_energy,
      death_timer: self.death_timer,
      state: self.state,
      area: self.area,
      world: self.world.clone(),
      died: self.downed,
      state_meta: self.state_meta,
    }
  }
}
