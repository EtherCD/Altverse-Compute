use std::{cell::RefCell, rc::Rc};

use napi::{Error, Status};

use crate::{
  assets::effect::slow::SlowPlayerEffect,
  units::{effect::Effect, entity::Entity, player::Player},
};

macro_rules! effect_dispatch {
  ($self:expr, $method:ident($($arg:expr),*)) => {
    match $self {
      EffectWrapper::Slow(v) => v.$method($($arg),*),
    }
  };
}

pub mod slow;

#[derive(Clone)]
pub enum EffectWrapper<'a> {
  Slow(SlowPlayerEffect<'a>),
}

impl<'a> EffectWrapper<'a> {
  pub fn new(
    name: &str,
    target: &'a mut Player,
    caster: &'a Entity,
    aura: Option<f64>,
  ) -> Result<Self, Error> {
    match name {
      "slow" => Ok(EffectWrapper::Slow(SlowPlayerEffect::new(
        Rc::new(RefCell::new(target)),
        Rc::new(caster),
        aura,
      ))),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Unknown effect name: ".to_string() + name,
      )),
    }
  }

  pub fn update(&mut self, props: &crate::units::structures::UpdateProps) -> bool {
    effect_dispatch!(self, update(props))
  }

  pub fn clear(&mut self) {
    effect_dispatch!(self, clear());
  }
}
