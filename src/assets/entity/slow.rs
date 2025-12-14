use crate::{
  assets::{effect::EffectWrapper, enemy::Enemy, entity::EnemyWrapper},
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps, distance},
  },
};

#[derive(Clone)]
pub struct SlowEntity {
  entity: Entity,
}

impl SlowEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.aura = 150.0;
    entity.state = 1;
    entity.state_metadata = 150.0;
    Self { entity }
  }
}

impl Enemy for SlowEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
  }

  fn interact<'a>(&mut self, player: &mut crate::units::player::Player<'a>) {
    if !self.entity.harmless
      && player.pos.x > -player.radius
      && player.pos.x - player.radius < self.entity.boundary.w
    {
      if !player.immortal && !player.downed {
        if distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        ) <= self.entity.radius + player.radius
        {
          player.knock();
        }
        if distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        ) <= self.entity.aura + player.radius
          && !player.has_effect(1)
        {
          let effect =
            EffectWrapper::new("slow", player, &self.entity, Some(self.entity.aura)).unwrap();
          player.add_effect(1, effect);
        }
      }
    }
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
