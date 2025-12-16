use crate::proto::PackedEntity;
use crate::resources::assets::entities::flame::{Flame, FlameTrail};
use crate::resources::assets::entities::normal::Normal;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::player::Player;
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};
use napi::{Error, Status};

macro_rules! entity_dispatch {
  ($self:expr, $method:ident($($arg:expr),*)) => {
    match $self {
      EntityWrapper::Normal(v) => v.$method($($arg),*),
      EntityWrapper::Flame(v) => v.$method($($arg),*),
      EntityWrapper::FlameTrail(v) => v.$method($($arg),*),
    }
  };
}

#[derive(Clone)]
pub enum EntityWrapper {
  Normal(Normal),
  Flame(Flame),
  FlameTrail(FlameTrail),
}

impl EntityWrapper {
  pub fn new(
    name: &str,
    props: &mut EntityProps,
    additional: AdditionalEntityProps,
  ) -> Result<Self, Error> {
    match name {
      "normal" => Ok(EntityWrapper::Normal(Normal::new(*props, additional))),
      "flame" => Ok(EntityWrapper::Flame(Flame::new(*props, additional))),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Unknown enemy type: ".to_string() + name,
      )),
    }
  }

  pub fn update(&mut self, props: &mut EntityUpdateProps) {
    entity_dispatch!(self, update(props));
  }

  pub fn interact(&mut self, player: &mut Player) {
    entity_dispatch!(self, interact(player));
  }

  pub fn pack(&self) -> PackedEntity {
    entity_dispatch!(self, pack())
  }

  pub fn is_to_remove(&self) -> bool {
    entity_dispatch!(self, is_to_remove())
  }
}
