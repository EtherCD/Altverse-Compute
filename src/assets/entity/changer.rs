use crate::{
  assets::{enemy::Enemy, entity::Enemies},
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps},
  },
};

#[derive(Clone)]
pub struct ChangerEntity {
  entity: Entity,
  disable: bool,
  timer: f64,
}

impl ChangerEntity {
  pub fn new(props: EntityProps, additional: AdditionalEntityProps) -> Self {
    let mut disable = false;

    if additional.num >= additional.count / 2 {
      disable = true;
    }

    Self {
      entity: Entity::new(props.clone()),
      disable,
      timer: 0.0,
    }
  }
}

impl Enemy for ChangerEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    self.timer += props.delta as f64;

    if self.timer > 5000.0 {
      self.disable = !self.disable;
    }

    self.entity.harmless = self.disable;

    self.timer = self.timer % 5000.0;

    self.entity.update(props);
    self.entity.collide();
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

  fn get_nested_entities(&self) -> Vec<Enemies> {
    return self.entity.nested_entities.clone();
  }

  fn clear_nested_entities(&mut self) {
    self.entity.nested_entities.clear()
  }
}
