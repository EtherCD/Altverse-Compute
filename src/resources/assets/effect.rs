use crate::resources::assets::effects::slow::PlayerSlow;
use crate::resources::assets::effects::PlayerEffectLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::effect::PlayerEffect;
use crate::resources::EffectUpdateProps;
use napi::{Error, Status};
use crate::resources::assets::effects::draining::PlayerDraining;
use crate::resources::assets::effects::slipped::PlayerSlipped;

macro_rules! effect_dispatch {
  ($self:expr, $method:ident($($arg:expr),*)) => {
    match $self {
      PlayerEffectWrapper::Slow(v) => v.$method($($arg),*),
      PlayerEffectWrapper::Draining(v) => v.$method($($arg),*),
      PlayerEffectWrapper::Slipped(v) => v.$method($($arg),*)
    }
  };
}

#[derive(Clone, Debug)]
pub enum PlayerEffectWrapper {
  Slow(PlayerSlow),
  Draining(PlayerDraining),
  Slipped(PlayerSlipped),
}

impl PlayerEffectWrapper {
  pub fn new(name: u64, hero: &HeroWrapper, caster_id: u64) -> Result<Self, Error> {
    match name {
      0 => Ok(PlayerEffectWrapper::Slow(PlayerSlow::new(hero, caster_id))),
      1 => Ok(PlayerEffectWrapper::Draining(PlayerDraining::new(hero, caster_id))),
      2 => Ok(PlayerEffectWrapper::Slipped(PlayerSlipped::new(hero, caster_id))),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Unknown effect type: ".to_string() + name.to_string().as_str(),
      )),
    }
  }

  pub fn update(&mut self, props: &mut EffectUpdateProps) {
    effect_dispatch!(self, update(props));
  }

  pub fn disable(&mut self, player: &mut HeroWrapper) {
    effect_dispatch!(self, disable(player));
  }

  pub fn enable(&mut self, player: &mut HeroWrapper) {
    effect_dispatch!(self, enable(player));
  }

  pub fn effect_id(&self) -> u64 {
    effect_dispatch!(self, effect_id())
  }

  pub fn effect(&self) -> &PlayerEffect {
    effect_dispatch!(self, effect())
  }

  pub fn effect_mut(&mut self) -> &mut PlayerEffect {
    effect_dispatch!(self, effect_mut())
  }
}
