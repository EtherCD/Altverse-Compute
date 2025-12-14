use napi::{Error, Status};

use crate::{
  assets::{
    enemy::Enemy,
    entity::{
      bee::BeeEntity, bullet::BulletEntity, changer::ChangerEntity, drop::DropEntity,
      fade::FadeEntity, flame::FlameEntity, flamebullet::FlameBulletEntity,
      flamesniper::FlameSniperEntity, flametrail::TrailEntity, homing::HomingEntity,
      immune::ImmuneEntity, normal::NormalEntity, pull::PullEntity, sniper::SniperEntity,
      vortex::VortexEntity, wall::WallEntity,
    },
  },
  network::PackedEntity,
  units::{
    player::Player,
    structures::{AdditionalEntityProps, EntityProps, EntityUpdateProps},
  },
};

macro_rules! enemy_dispatch {
  ($self:expr, $method:ident($($arg:expr),*)) => {
    match $self {
      EnemyWrapper::Normal(v) => v.$method($($arg),*),
      EnemyWrapper::Wall(v) => v.$method($($arg),*),
      EnemyWrapper::Immune(v) => v.$method($($arg),*),
      EnemyWrapper::Changer(v) => v.$method($($arg),*),
      EnemyWrapper::Homing(v) => v.$method($($arg),*),
      EnemyWrapper::Vortex(v) => v.$method($($arg),*),
      EnemyWrapper::Pull(v) => v.$method($($arg),*),
      EnemyWrapper::Sniper(v) => v.$method($($arg),*),
      EnemyWrapper::Bullet(v) => v.$method($($arg),*),
      EnemyWrapper::Flame(v) => v.$method($($arg),*),
      EnemyWrapper::FlameTrail(v) => v.$method($($arg),*),
      EnemyWrapper::FlameSniper(v) => v.$method($($arg),*),
      EnemyWrapper::FlameBullet(v) => v.$method($($arg),*),
      EnemyWrapper::Drop(v) => v.$method($($arg),*),
      EnemyWrapper::Fade(v) => v.$method($($arg),*),
      EnemyWrapper::Bee(v) => v.$method($($arg),*),
      // EnemyWrapper::Slower(v) => v.$method($($arg),*),
    }
  };
}

pub mod bee;
pub mod bullet;
pub mod changer;
pub mod drop;
pub mod fade;
pub mod flame;
pub mod flamebullet;
pub mod flamesniper;
pub mod flametrail;
pub mod homing;
pub mod immune;
pub mod normal;
pub mod pull;
// pub mod slow;
pub mod sniper;
pub mod vortex;
pub mod wall;

#[derive(Clone)]
pub enum EnemyWrapper {
  Normal(NormalEntity),
  Wall(WallEntity),
  Immune(ImmuneEntity),
  Changer(ChangerEntity),
  Homing(HomingEntity),
  Vortex(VortexEntity),
  Pull(PullEntity),
  Sniper(SniperEntity),
  Bullet(BulletEntity),
  Flame(FlameEntity),
  FlameTrail(TrailEntity),
  FlameSniper(FlameSniperEntity),
  FlameBullet(FlameBulletEntity),
  Drop(DropEntity),
  Fade(FadeEntity),
  Bee(BeeEntity),
  // Slower(SlowEntity),
}

impl EnemyWrapper {
  pub fn new(
    name: &str,
    props: &mut EntityProps,
    additional: AdditionalEntityProps,
  ) -> Result<Self, Error> {
    match name {
      "normal" => {
        props.type_id = 0;
        Ok(EnemyWrapper::Normal(NormalEntity::new(*props, additional)))
      }
      "wall" => {
        props.type_id = 1;
        Ok(EnemyWrapper::Wall(WallEntity::new(*props, additional)))
      }
      "immune" => {
        props.type_id = 2;
        Ok(EnemyWrapper::Immune(ImmuneEntity::new(*props, additional)))
      }
      "changer" => {
        props.type_id = 5;
        Ok(EnemyWrapper::Changer(ChangerEntity::new(
          *props, additional,
        )))
      }
      "homing" => {
        props.type_id = 9;
        Ok(EnemyWrapper::Homing(HomingEntity::new(*props, additional)))
      }
      "vortex" => {
        props.type_id = 10;
        Ok(EnemyWrapper::Vortex(VortexEntity::new(*props, additional)))
      }
      "pull" => {
        props.type_id = 22;
        Ok(EnemyWrapper::Pull(PullEntity::new(*props, additional)))
      }
      "sniper" => {
        props.type_id = 3;
        Ok(EnemyWrapper::Sniper(SniperEntity::new(*props, additional)))
      }
      "flame" => {
        props.type_id = 18;
        Ok(EnemyWrapper::Flame(FlameEntity::new(*props, additional)))
      }
      "flame_sniper" => {
        props.type_id = 20;
        Ok(EnemyWrapper::FlameSniper(FlameSniperEntity::new(
          *props, additional,
        )))
      }
      "drop" => {
        props.type_id = 7;
        Ok(EnemyWrapper::Drop(DropEntity::new(*props, additional)))
      }
      "fade" => {
        props.type_id = 23;
        Ok(EnemyWrapper::Fade(FadeEntity::new(*props, additional)))
      }
      "bee" => {
        props.type_id = 15;
        Ok(EnemyWrapper::Bee(BeeEntity::new(*props, additional)))
      }
      // "slower" => {
      //   props.type_id = 11;
      //   Ok(EnemyWrapper::Slower(SlowEntity::new(*props, additional)))
      // }
      _ => Err(Error::new(
        Status::InvalidArg,
        "Unknown enemy type: ".to_string() + name,
      )),
    }
  }

  pub fn update(&mut self, props: &EntityUpdateProps) {
    enemy_dispatch!(self, update(props));
  }

  pub fn interact(&mut self, player: &mut Player) {
    enemy_dispatch!(self, interact(player));
  }

  pub fn pack(&self) -> PackedEntity {
    enemy_dispatch!(self, pack())
  }

  pub fn is_to_remove(&self) -> bool {
    enemy_dispatch!(self, is_to_remove())
  }

  pub fn get_nested_entities(&self) -> Vec<EnemyWrapper> {
    enemy_dispatch!(self, get_nested_entities())
  }

  pub fn clear_nested_entities(&mut self) {
    enemy_dispatch!(self, clear_nested_entities())
  }
}
