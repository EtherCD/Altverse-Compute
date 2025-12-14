use crate::{
  assets::{enemy::Enemy, entity::EnemyWrapper},
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps},
  },
};

#[derive(Clone)]
pub struct TrailEntity {
  pub entity: Entity,
  timer: f64,
  pub owner_speed: f64,
}

impl TrailEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
      timer: 0.0,
      owner_speed: 0.0,
    }
  }
}

impl Enemy for TrailEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;
    self.entity.alpha = 1.0 - self.timer / (5000.0 / self.owner_speed);
    if self.timer >= 5000.0 / self.owner_speed {
      self.entity.to_remove = true;
    }
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
