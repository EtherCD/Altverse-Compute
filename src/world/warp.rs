use std::collections::HashMap;

use crate::{CONFIG, units::player::Player, world::world::World};

pub enum Change {
  NextArea,
  PrevArea,
  // NextWorld,
  // PrevWorld,
}

pub struct Warp {}

impl Warp {
  pub fn update(
    worlds: &HashMap<String, World>,
    players: &HashMap<i64, Player>,
  ) -> HashMap<i64, Change> {
    let mut changes: HashMap<i64, Change> = HashMap::new();

    for (id, player) in players.iter() {
      if let Some(world) = worlds.get(&player.world) {
        if let Some(area) = world.areas.get(player.area as usize) {
          if player.pos.x + player.radius > area.w + 8.0 * 32.0
            && Warp::get_next_area(worlds, player)
          {
            changes.insert(*id, Change::NextArea);
            // area.leave(*id as usize);
            // player.pos.x = -8.0 * 32.0 + player.radius;
            // player.area = player.area + 1;
            // next_area.join(player);
          }

          if player.area > 0
            && player.pos.x - player.radius < -8.0 * 32.0
            && Warp::get_prev_area(worlds, player)
          {
            changes.insert(*id, Change::PrevArea);
            // area.leave(*id as usize);
            // player.pos.x = -8.0 * 32.0 + player.radius;
            // player.area = player.area - 1;
            // prev_area.join(player);
          }
          // if player.area == 0
          //   && player.pos.x - player.radius < 0.0
          //   && player.pos.y - player.radius < 2.0 * 32.0
          //   && Warp::get_next_world(player)
          // {
          //   changes.insert(*id, Change::NextWorld);
          //   // area.leave(*id as usize);
          //   // player.pos.x = -8.0 * 32.0 + player.radius;
          //   // player.area = player.area - 1;
          //   // prev_area.join(player);
          // }

          // if player.area == 0
          //   && player.pos.x - player.radius < 0.0
          //   && player.pos.y - player.radius > area.h - 2.0 * 32.0
          //   && Warp::get_prev_world(player)
          // {
          //   changes.insert(*id, Change::PrevWorld);
          //   // area.leave(*id as usize);
          //   // player.pos.x = -8.0 * 32.0 + player.radius;
          //   // player.area = player.area - 1;
          //   // prev_area.join(player);
          // }
        }
      }
    }

    changes
  }

  // pub fn can_warp_next_world(&mut self, player: &Player) -> bool {}
  //
  // pub fn can_warp_prev_world(&mut self, player: &Player) -> bool {}

  pub fn get_next_world(player: &Player) -> bool {
    let config = CONFIG.lock().unwrap();
    let world_position = config
      .worlds
      .iter()
      .position(|s| s == &player.world)
      .unwrap();
    if world_position + 1 < config.worlds.len() {
      return true;
    }
    false
  }

  pub fn get_prev_world(player: &Player) -> bool {
    let config = CONFIG.lock().unwrap();
    let world_position = config
      .worlds
      .iter()
      .position(|s| s == &player.world)
      .unwrap();
    if world_position - 1 > 0 {
      return true;
    }
    false
  }

  pub fn get_next_area(worlds: &HashMap<String, World>, player: &Player) -> bool {
    if let Some(world) = worlds.get(&player.world) {
      if let Some(_) = world.areas.get(player.area as usize + 1) {
        return true;
      }
    }
    false
  }

  pub fn get_prev_area(worlds: &HashMap<String, World>, player: &Player) -> bool {
    if let Some(world) = worlds.get(&player.world) {
      if let Some(_) = world.areas.get(player.area as usize - 1) {
        return true;
      }
    }
    false
  }
}
