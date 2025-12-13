use napi::{Error, Status};

use crate::{
  assets::{
    enemy::Enemy,
    entity::{
      changer::ChangerEntity, immune::ImmuneEntity, normal::NormalEntity, wall::WallEntity,
    },
  },
  network::PackedEntity,
  units::{
    player::Player,
    structures::{AdditionalEntityProps, EntityProps, UpdateProps},
  },
};

macro_rules! enemy_dispatch {
  ($self:expr, $method:ident($($arg:expr),*)) => {
    match $self {
      Enemies::Normal(v) => v.$method($($arg),*),
      Enemies::Wall(v) => v.$method($($arg),*),
      Enemies::Immune(v) => v.$method($($arg),*),
      Enemies::Changer(v) => v.$method($($arg),*),
    }
  };
}

pub mod changer;
pub mod immune;
pub mod normal;
pub mod wall;

pub enum Enemies {
  Normal(NormalEntity),
  Wall(WallEntity),
  Immune(ImmuneEntity),
  Changer(ChangerEntity),
}

impl Enemies {
  pub fn new(
    name: &str,
    props: &mut EntityProps,
    additional: AdditionalEntityProps,
  ) -> Result<Self, Error> {
    match name {
      "normal" => {
        props.type_id = 0;
        Ok(Enemies::Normal(NormalEntity::new(*props, additional)))
      }
      "wall" => {
        props.type_id = 1;
        Ok(Enemies::Wall(WallEntity::new(*props, additional)))
      }
      "immune" => {
        props.type_id = 2;
        Ok(Enemies::Immune(ImmuneEntity::new(*props, additional)))
      }
      "changer" => {
        props.type_id = 5;
        Ok(Enemies::Changer(ChangerEntity::new(*props, additional)))
      }
      _ => Err(Error::new(
        Status::InvalidArg,
        "Unknown enemy type: ".to_string() + name,
      )),
    }
  }

  pub fn update(&mut self, props: UpdateProps) {
    enemy_dispatch!(self, update(props));
  }

  pub fn interact(&mut self, player: &mut Player) {
    enemy_dispatch!(self, interact(player));
  }

  pub fn pack(&self) -> PackedEntity {
    enemy_dispatch!(self, pack())
  }
}
