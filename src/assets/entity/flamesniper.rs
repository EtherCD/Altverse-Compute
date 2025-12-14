use crate::{
  assets::{
    enemy::Enemy,
    entity::{EnemyWrapper, flamebullet::FlameBulletEntity},
  },
  units::{
    entity::Entity,
    player::Player,
    random,
    structures::{AdditionalEntityProps, EntityProps, distance},
  },
};

#[derive(Clone)]
pub struct FlameSniperEntity {
  entity: Entity,
  timer: f64,
}

impl FlameSniperEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
      timer: random(3000.0, 6000.0),
    }
  }
}

impl Enemy for FlameSniperEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;

    if self.timer > 6000.0 {
      let mut target: Option<&&Player> = None;
      let mut last_distance = 20.0 * 32.0;
      for player in props.players.iter() {
        if player.pos.x > -player.radius
          && player.pos.x - player.radius < self.entity.boundary.w
          && !player.downed
        {
          let dist = distance(
            player.pos.x - self.entity.pos.x,
            player.pos.y - self.entity.pos.y,
          );
          if dist <= 20.0 * 32.0 && dist < last_distance {
            last_distance = dist;
            target = Some(player);
          }
        }

        if let Some(target) = target {
          let angl = (target.pos.y - self.entity.pos.y).atan2(target.pos.x - self.entity.pos.x);

          let mut bullet = FlameBulletEntity::new(
            EntityProps {
              type_id: 3,
              radius: self.entity.radius / 2.0,
              speed: 10.0,
              boundary: self.entity.boundary,
            },
            AdditionalEntityProps {
              count: 0,
              num: 0,
              inverse: false,
            },
          );
          bullet.entity.vel.x = angl.cos() * 10.0;
          bullet.entity.vel.y = angl.sin() * 10.0;
          bullet.entity.pos.x = self.entity.pos.x;
          bullet.entity.pos.y = self.entity.pos.y;

          self
            .entity
            .nested_entities
            .push(EnemyWrapper::FlameBullet(bullet));

          self.timer = 0.0;
        }
      }
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
