use std::collections::HashMap;

use crate::{
  network::PackedArea,
  units::player::Player,
  world::{RawWorld, area::Area},
};

pub struct World {
  pub areas: Vec<Area>,
  name: String,
}

impl World {
  pub fn new(props: RawWorld) -> Self {
    let mut areas = Vec::new();
    for a in props.areas {
      areas.push(Area::new(a));
    }
    Self {
      areas,
      name: props.name,
    }
  }

  pub fn join(&mut self, player: &Player) {
    if let Some(area) = self.areas.get_mut(player.area as usize) {
      area.join(player);
    }
  }

  pub fn leave(&mut self, player: &Player) {
    if let Some(area) = self.areas.get_mut(player.area as usize) {
      area.leave(player);
    }
  }

  pub fn interact(&mut self, players: &mut HashMap<i64, Player>) {
    for area in self.areas.iter_mut() {
      for (_, entity) in area.entities.iter_mut() {
        for id in area.players.iter() {
          if let Some(player) = players.get_mut(&id) {
            entity.interact(player);
          }
        }
      }

      // for fid in area.players.iter() {
      //   if let Some(first) = players.get_mut(&fid) {
      //     for sid in area.players.iter() {
      //       if (fid != sid) {
      //         for sid in area.players.iter() {
      //           if let Some(second) = players.get_mut(&sid) {
      //             if (!first.downed
      //               && second.downed
      //               && distance(first.pos.x - second.pos.x, first.pos.y - second.pos.y)
      //                 <= first.radius + second.radius)
      //             {
      //               second.res();
      //             }
      //           }
      //         }
      //       }
      //     }
      //   }
      // }

      // let players = &mut area.get_players(players);
      //   for (_, player) in players.iter_mut() {
      //     entity.interact(player);
      //   }

      // for (fid, first) in players.iter() {
      //   for (sid, second) in players {
      //
      //   }
      // }
    }
  }

  pub fn pack_area(&mut self, area_id: usize) -> PackedArea {
    let area = self.areas.get(area_id).unwrap();
    return PackedArea {
      w: area.w,
      h: area.h,
      area: area_id as u32,
      world: self.name.clone(),
      entities: area.get_entities(),
    };
  }
}
