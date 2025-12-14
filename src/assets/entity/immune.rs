use crate::{
  assets::{enemy::Enemy, entity::EnemyWrapper},
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps},
  },
};

#[derive(Clone)]
pub struct ImmuneEntity {
  entity: Entity,
}

impl ImmuneEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
    }
  }
}

impl Enemy for ImmuneEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
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

  fn get_nested_entities(&self) -> Vec<EnemyWrapper> {
    return self.entity.nested_entities.clone();
  }

  fn clear_nested_entities(&mut self) {
    self.entity.nested_entities.clear()
  }
}
