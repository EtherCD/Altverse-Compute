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
      area.leave(player.id);
    }
  }

  pub fn pack_area(&mut self, area_id: usize) -> PackedArea {
    let area = self.areas.get(area_id).unwrap();
    return PackedArea {
      w: area.w,
      h: area.h,
      area: area_id as u32,
      world: self.name.clone(),
      entities: area.get_enemies(),
    };
  }
}
