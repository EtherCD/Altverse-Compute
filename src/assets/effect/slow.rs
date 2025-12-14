use std::{cell::RefCell, rc::Rc};

use crate::units::{effect::Effect, entity::Entity, player::Player, structures::distance};

#[derive(Clone)]
pub struct SlowPlayerEffect<'a> {
  target: Rc<RefCell<&'a mut Player>>,
  caster: Rc<&'a Entity>,
  original_speed: f64,
  aura: Option<f64>,
}

impl<'a> SlowPlayerEffect<'a> {
  pub fn new(
    target: Rc<RefCell<&'a mut Player>>,
    caster: Rc<&'a Entity>,
    aura: Option<f64>,
  ) -> Self {
    let original_speed = target.borrow().speed;
    target.borrow_mut().speed = original_speed / 2.0;
    Self {
      target,
      caster,
      original_speed,
      aura,
    }
  }
}

impl<'a> Effect for SlowPlayerEffect<'a> {
  fn update(&mut self, _: &crate::units::structures::UpdateProps) -> bool {
    if let Some(aura) = self.aura {
      let target = self.target.borrow();
      if distance(
        self.caster.pos.x - target.pos.x,
        self.caster.pos.y - target.pos.y,
      ) > aura
      {
        return true;
      }
    }
    return false;
  }

  fn clear(&mut self) {
    let mut target = self.target.borrow_mut();
    target.speed = self.original_speed;
  }
}
