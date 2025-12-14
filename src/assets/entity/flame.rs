use crate::{
  assets::{
    enemy::Enemy,
    entity::{EnemyWrapper, flametrail::TrailEntity},
  },
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps},
  },
};

#[derive(Clone)]
pub struct FlameEntity {
  entity: Entity,
  timer: f64,
}

impl FlameEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
      timer: 0.0,
    }
  }
}

impl Enemy for FlameEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;
    if self.timer >= 32.0 * ((self.entity.radius * 2.0) / self.entity.speed) {
      let mut trail = TrailEntity::new(
        EntityProps {
          type_id: 19,
          radius: self.entity.radius,
          speed: 0.0,
          boundary: self.entity.boundary,
        },
        AdditionalEntityProps {
          count: 0,
          num: 0,
          inverse: false,
        },
      );
      trail.entity.pos = self.entity.pos.clone();

      trail.owner_speed = self.entity.speed;
      self.timer = 0.0;
      self
        .entity
        .nested_entities
        .push(EnemyWrapper::FlameTrail(trail));
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
