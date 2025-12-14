use crate::{
  assets::{enemy::Enemy, entity::EnemyWrapper},
  units::{
    entity::Entity,
    structures::{AdditionalEntityProps, EntityProps, distance},
  },
};

#[derive(Clone)]
pub struct PullEntity {
  entity: Entity,
  time_fix: f64,
}

impl PullEntity {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.aura = 150.0;
    entity.state = 1;
    entity.state_metadata = 150.0;
    Self {
      entity: entity.clone(),
      time_fix: 0.0,
    }
  }
}

impl Enemy for PullEntity {
  fn update(&mut self, props: &crate::units::structures::EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
    self.time_fix = props.time_fix;
  }

  fn interact(&mut self, player: &mut crate::units::player::Player) {
    self.entity.interact(player);
    if player.pos.x - player.radius > 0.0 && player.pos.x + player.radius < self.entity.boundary.w {
      let dist = distance(
        player.pos.x - self.entity.pos.x,
        player.pos.y - self.entity.pos.y,
      );

      if dist <= self.entity.aura && !player.downed {
        let dx = player.pos.x - self.entity.pos.x;
        let dy = player.pos.y - self.entity.pos.y;
        let attract_amplitude = (-(dist / 120.0)).powf(2.0);
        let move_dist = 2.5 * attract_amplitude;
        let angle = dy.atan2(dx);

        player.pos.x -= move_dist * angle.cos() * self.time_fix;
        player.pos.y -= move_dist * angle.sin() * self.time_fix;
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
