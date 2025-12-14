use std::f32::consts::PI;

use crate::{
  assets::{enemy::Enemy, entity::EnemyWrapper},
  units::{
    entity::Entity,
    player::Player,
    structures::{AdditionalEntityProps, EntityProps, distance},
  },
};

const MAX_DIST: f64 = 5.625 * 32.0;
const ANGLE_INCREMENT: f64 = 0.04;

#[derive(Clone)]
pub struct BeeEntity {
  entity: Entity,
}

impl BeeEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
    }
  }
}

impl Enemy for BeeEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    let mut target: Option<&&Player> = None;
    let mut last_distance = MAX_DIST;
    for player in props.players.iter() {
      if player.pos.x > -player.radius
        && player.pos.x - player.radius < self.entity.boundary.w
        && !player.downed
      {
        let dist = distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        );
        if dist <= MAX_DIST && dist < last_distance {
          last_distance = dist;
          target = Some(player);
        }
      }
    }

    if let Some(target) = target {
      let d_x = target.pos.x - self.entity.pos.x;
      let d_y = target.pos.y - self.entity.pos.y;
      let target_angle = d_y.atan2(d_x);

      self.entity.vel_to_angle();

      let mut angle_diff = target_angle - self.entity.angle;

      if angle_diff > PI as f64 {
        angle_diff -= 2.0 * PI as f64;
      }
      if angle_diff < -PI as f64 {
        angle_diff += 2.0 * PI as f64;
      }

      let max_turn = ANGLE_INCREMENT * (props.delta as f64 / 16.67);
      if angle_diff.abs() < max_turn {
        self.entity.angle = target_angle;
      } else {
        self.entity.angle += angle_diff.sin() * max_turn;
      }

      self.entity.angle_to_vel();
    }

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
