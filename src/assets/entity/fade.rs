use crate::{
  assets::{enemy::Enemy, entity::EnemyWrapper},
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps, EntityUpdateProps},
  },
};

const MAX_TIME: f64 = 7500.0;
const START_TIME: f64 = 7500.0 / 2.0;

#[derive(Clone)]
pub struct FadeEntity {
  entity: Entity,
  timer: f64,
}

impl FadeEntity {
  pub fn new(props: EntityProps, additional: AdditionalEntityProps) -> Self {
    let mut timer = 0.0;
    if additional.num > additional.count / 2 {
      timer = START_TIME;
    }
    Self {
      entity: Entity::new(props.clone()),
      timer,
    }
  }
}

impl Enemy for FadeEntity {
  fn update(&mut self, props: &EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;

    let period = MAX_TIME;

    let phase = (self.timer / period) * std::f64::consts::TAU;
    self.entity.alpha = (phase.cos() + 1.0) * 0.5;
    self.entity.harmless = self.entity.alpha < 0.5;
  }

  fn interact(&mut self, player: &mut crate::units::player::Player) {
    self.entity.interact(player);
  }

  fn pack(&self) -> crate::network::PackedEntity {
    self.entity.pack()
  }

  fn is_to_remove(&self) -> bool {
    self.entity.to_remove
  }

  fn get_nested_entities(&self) -> Vec<EnemyWrapper> {
    return self.entity.nested_entities.clone();
  }

  fn clear_nested_entities(&mut self) {
    self.entity.nested_entities.clear()
  }
}
