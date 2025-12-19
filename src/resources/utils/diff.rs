use crate::proto::{PackedEntity, PackedPlayer, PartialEntity, PartialPlayer};

#[macro_export]
macro_rules! diff_field {
  ($self:ident, $new:ident, $field:ident) => {{
    if $self.$field != $new.$field {
      Some($new.$field.try_into().unwrap())
    } else {
      None
    }
  }};

  ($self:ident, $new:ident, $field:ident, clone) => {{
    if $self.$field != $new.$field {
      Some($new.$field.clone())
    } else {
      None
    }
  }};
}

#[macro_export]
macro_rules! diff_delta {
  ($self:ident, $new:ident, $field:ident) => {{
    let old_val: i32 = $self.$field;
    let new_val: i32 = $new.$field;
    if old_val != new_val {
      Some((old_val - new_val) as i32)
    } else {
      None
    }
  }};
  ($self:ident, $new:ident, $field:ident, $typ:ident) => {{
    let old_val: $typ = $self.$field;
    let new_val: $typ = $new.$field;
    if old_val != new_val {
      Some((old_val - new_val) as $typ)
    } else {
      None
    }
  }};
}

impl PackedEntity {
  pub fn diff(&self, new: &PackedEntity) -> (PartialEntity, bool) {
    let mut changed = false;

    if self.type_id != new.type_id
      || self.x != new.x
      || self.y != new.y
      || self.radius != new.radius
      || self.harmless != new.harmless
      || self.state != new.state
      || self.state_metadata != new.state_metadata
      || self.alpha != new.alpha
    {
      changed = true;
    }

    (
      PartialEntity {
        x: diff_field!(self, new, x),
        y: diff_field!(self, new, y),
        radius: diff_field!(self, new, radius),
        harmless: diff_field!(self, new, harmless),
        state: diff_field!(self, new, state),
        state_metadata: diff_field!(self, new, state_metadata),
        alpha: diff_field!(self, new, alpha),
      },
      changed,
    )
  }
}

impl PackedPlayer {
  pub fn diff(&self, new: &PackedPlayer) -> (PartialPlayer, bool) {
    let mut changed = false;
    if self.id != new.id
      || self.x != new.x
      || self.y != new.y
      || self.radius != new.radius
      || self.speed != new.speed
      || self.state != new.state
      || self.energy != new.energy
      || self.max_energy != new.max_energy
      || self.died != new.died
      || self.death_timer != new.death_timer
      || self.world != new.world
      || self.area != new.area
      || self.state_meta != new.state_meta
    {
      changed = true;
    }

    (
      PartialPlayer {
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
        world: (self.world != new.world).then(|| new.world.clone()),
        died: (self.died != new.died).then(|| new.died),
      },
      changed,
    )
  }
}
