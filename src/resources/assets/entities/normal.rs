use crate::proto::PackedEntity;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::entity::Entity;
use crate::resources::player::Player;
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Normal {
  entity: Entity,
}

impl Normal {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 0;
    Self { entity }
  }
}

impl EntityLogic for Normal {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
  }

  fn interact(&mut self, player: &mut Player) {
    self.entity.interact(player);
  }

  fn pack(&self) -> PackedEntity {
    self.entity.pack()
  }

  fn is_to_remove(&self) -> bool {
    self.entity.to_remove
  }
}
