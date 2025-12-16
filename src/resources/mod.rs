use crate::bus::EventBus;
use crate::resources::player::Player;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;

mod area;
pub mod assets;
pub mod entity;
pub mod player;
pub mod utils;
pub mod world;

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(rand::rng());
}

// Structures

#[derive(Debug, Clone, Copy)]
pub struct Boundary {
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64,
}

#[derive(Clone, Copy)]
pub struct EntityProps {
  pub type_id: u64,
  pub radius: f64,
  pub speed: f64,
  pub boundary: Boundary,
}

pub struct EntityUpdateProps<'a> {
  pub delta: i64,
  pub time_fix: f64,
  pub players: Vec<&'a Player>,
  pub event_bus: &'a mut EventBus,
}

pub struct UpdateProps {
  pub delta: i64,
  pub time_fix: f64,
}

pub struct PlayerUpdateProps<'a> {
  pub delta: i64,
  pub time_fix: f64,
  pub players: Vec<&'a Player>,
}

#[derive(Clone, Copy)]
pub struct AdditionalEntityProps {
  pub count: u64,
  pub num: u64,
  pub inverse: bool,
}

// functions

pub fn distance(a: f64, b: f64) -> f64 {
  (a * a + b * b).sqrt()
}

pub fn random(min: f64, max: f64) -> f64 {
  RNG.with(|rng| {
    let mut r: f64 = rng.borrow_mut().random::<f64>();
    r = r.clamp(0.0, 1.0 - f64::EPSILON * 2.0);

    r * (max - min) + min
  })
}
