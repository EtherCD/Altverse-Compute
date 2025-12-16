use crate::proto::{PackedEntity, PackedPlayer, PartialEntity, PartialPlayer};

impl PackedEntity {
  pub fn diff(&self, new: &PackedEntity) -> PartialEntity {
    PartialEntity {
      type_id: if self.type_id != new.type_id {
        Some(new.type_id)
      } else {
        None
      },
      x: if self.x != new.x { Some(new.x) } else { None },
      y: if self.y != new.y { Some(new.y) } else { None },
      radius: if self.radius != new.radius {
        Some(new.radius)
      } else {
        None
      },
      harmless: if self.harmless != new.harmless {
        Some(new.harmless)
      } else {
        None
      },
      aura: if self.aura != new.aura {
        Some(new.aura)
      } else {
        None
      },
      state: if self.state != new.state {
        Some(new.state)
      } else {
        None
      },
      state_metadata: if self.state_metadata != new.state_metadata {
        Some(new.state_metadata)
      } else {
        None
      },
      alpha: if self.alpha != new.alpha {
        Some(new.alpha)
      } else {
        None
      },
    }
  }
}

impl PackedPlayer {
  pub fn diff(&self, new: &PackedPlayer) -> PartialPlayer {
    PartialPlayer {
      name: if self.name != new.name {
        Some(new.name.clone())
      } else {
        None
      },
      id: if self.id != new.id {
        Some(new.id)
      } else {
        None
      },
      x: if self.x != new.x { Some(new.x) } else { None },
      y: if self.y != new.y { Some(new.y) } else { None },
      radius: if self.radius != new.radius {
        Some(new.radius)
      } else {
        None
      },
      speed: if self.speed != new.speed {
        Some(new.speed)
      } else {
        None
      },
      energy: if self.energy != new.energy {
        Some(new.energy)
      } else {
        None
      },
      max_energy: if self.max_energy != new.max_energy {
        Some(new.max_energy)
      } else {
        None
      },
      death_timer: if self.death_timer != new.death_timer {
        Some(new.death_timer)
      } else {
        None
      },
      state: if self.state != new.state {
        Some(new.state)
      } else {
        None
      },
      state_meta: if self.state_meta != new.state_meta {
        Some(new.state_meta)
      } else {
        None
      },
      area: if self.area != new.area {
        Some(new.area)
      } else {
        None
      },
      world: if self.world != new.world {
        Some(new.world.clone())
      } else {
        None
      },
      died: if self.died != new.died {
        Some(new.died)
      } else {
        None
      },
    }
  }
}
