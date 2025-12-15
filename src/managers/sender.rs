use crate::managers::player::PlayersManager;
use crate::managers::world::WorldsManager;
use crate::packager::{PackedEntity, PackedPlayer};
use std::collections::HashMap;

pub struct Sender {
  old_entities_pack: HashMap<i64, PackedEntity>,
  new_entities_pack: HashMap<i64, PackedEntity>,
  old_players_pack: HashMap<i64, PackedPlayer>,
  new_players_pack: HashMap<i64, PackedPlayer>,
}

impl Sender {
  pub fn new() -> Sender {
    Self {
      old_entities_pack: HashMap::new(),
      new_entities_pack: HashMap::new(),
      old_players_pack: HashMap::new(),
      new_players_pack: HashMap::new(),
    }
  }

  pub fn before_snapshot(
    &mut self,
    players_manager: &mut PlayersManager,
    worlds_manager: &mut WorldsManager,
  ) {
    self.old_players_pack = players_manager.pack_players();
  }

  // pub fn after_snapshot(
  //   &mut self,
  //   players_manager: &mut PlayersManager,
  //   worlds_manager: &mut WorldsManager,
  // ) -> Vec<Pl>{
  //   self.new_players_pack = players_manager.pack_players();
  //
  //     for
  // }
}
