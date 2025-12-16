use crate::proto::{PackedEntity, PackedPlayer, PartialEntity, PartialPlayer};

#[macro_export]
macro_rules! diff_field {
  ($self:ident, $new:ident, $field:ident) => {
    if $self.$field != $new.$field {
      Some($new.$field.clone())
    } else {
      None
    }
  };
  ($self:ident, $new:ident, $field:ident, copy) => {
    if $self.$field != $new.$field {
      Some($new.$field)
    } else {
      None
    }
  };
}

impl PackedEntity {
  pub fn diff(&self, new: &PackedEntity) -> PartialEntity {
    PartialEntity {
      type_id: diff_field!(self, new, type_id),
      x: diff_field!(self, new, x),
      y: diff_field!(self, new, y),
      radius: diff_field!(self, new, radius),
      harmless: diff_field!(self, new, harmless),
      aura: diff_field!(self, new, aura),
      state: diff_field!(self, new, state),
      state_metadata: diff_field!(self, new, state_metadata),
      alpha: diff_field!(self, new, alpha),
    }
  }
}

impl PackedPlayer {
  pub fn diff(&self, new: &PackedPlayer) -> PartialPlayer {
    PartialPlayer {
      name: diff_field!(self, new, name),
      id: diff_field!(self, new, id),
      x: diff_field!(self, new, x),
      y: diff_field!(self, new, y),
      radius: diff_field!(self, new, radius),
      speed: diff_field!(self, new, speed),
      energy: diff_field!(self, new, energy),
      max_energy: diff_field!(self, new, max_energy),
      death_timer: diff_field!(self, new, death_timer),
      state: diff_field!(self, new, state),
      state_meta: diff_field!(self, new, state_meta),
      area: diff_field!(self, new, area),
      world: diff_field!(self, new, world),
      died: diff_field!(self, new, died),
    }
  }
}
