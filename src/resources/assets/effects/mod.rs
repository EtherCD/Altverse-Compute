use crate::resources::assets::hero::HeroWrapper;
use crate::resources::effect::PlayerEffect;
use crate::resources::EffectUpdateProps;

pub mod slow;

pub trait PlayerEffectLogic {
  fn enable(&mut self, player: &mut HeroWrapper);
  fn disable(&self, player: &mut HeroWrapper);
  fn update(&mut self, props: &EffectUpdateProps);
  fn effect(&self) -> &PlayerEffect;
  fn effect_mut(&mut self) -> &mut PlayerEffect;
  fn effect_id(&self) -> u64;
}
