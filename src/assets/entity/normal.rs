use crate::{
  assets::enemy::Enemy,
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps},
  },
};

pub struct NormalEntity {
  entity: Entity,
}

impl NormalEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
    }
  }
}

impl Enemy for NormalEntity {
  fn update(&mut self, props: crate::units::structures::UpdateProps) {
    self.entity.update(props);
    self.entity.collide();
  }

  fn interact(&mut self, player: &mut crate::units::player::Player) {
    self.entity.interact(player);
  }

  fn pack(&self) -> crate::network::PackedEntity {
    self.entity.pack()
  }
}
