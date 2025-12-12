use std::cell::RefCell;

use rand::{Rng, rngs::ThreadRng};

pub mod entity;
pub mod player;
pub mod structures;
pub mod vector;

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(rand::rng());
}

pub fn random(min: f64, max: f64) -> f64 {
  RNG.with(|rng| rng.borrow_mut().random::<f64>()) * (max - min) + min
}
