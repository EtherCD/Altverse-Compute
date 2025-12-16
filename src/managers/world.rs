use crate::bus::NetworkBus;
use crate::props::EngineProps;
use crate::proto::PackedEntity;
use crate::resources::player::Player;
use crate::resources::world::World;
use crate::resources::{distance, EntityUpdateProps, UpdateProps};
use serde_json::Value;
use std::collections::HashMap;

pub struct WorldsManager {
  pub worlds: HashMap<String, World>,
  pub new_entities: HashMap<u64, PackedEntity>,
  pub old_entities: HashMap<u64, PackedEntity>,
  pub entities_diff: HashMap<u64, HashMap<String, Value>>,
}

pub enum Change {
  NextArea,
  PrevArea,
  NextWorld,
  PrevWorld,
}

impl WorldsManager {
  pub fn new(props: &EngineProps) -> Self {
    Self {
      worlds: props.load_worlds().unwrap(),
      new_entities: HashMap::new(),
      old_entities: HashMap::new(),
      entities_diff: HashMap::new(),
    }
  }

  pub fn update(
    &mut self,
    props: &UpdateProps,
    players: &mut HashMap<i64, Player>,
    network_bus: &mut NetworkBus,
  ) {
    for world in self.worlds.values_mut() {
      for area in world.areas.iter_mut() {
        self.old_entities = area.get_packed_entities();
        self.entities_diff.clear();

        let entity_update = EntityUpdateProps {
          delta: props.delta,
          time_fix: props.time_fix,
          players: area.get_players_vec(&players),
        };

        for (_, entity) in area.entities.iter_mut() {
          entity.update(&entity_update);
          entity.collide();
        }

        for (_, entity) in area.entities.iter_mut() {
          for id in &area.players_id {
            if let Some(player) = players.get_mut(&id) {
              entity.interact(player);
            }
          }
        }

        for first_id in &area.players_id {
          for second_id in &area.players_id {
            if first_id == second_id {
              continue;
            }

            let can_rescue = {
              let first_player = players.get(&first_id).unwrap();
              let second_player = players.get(&second_id).unwrap();
              second_player.downed
                && distance(
                  second_player.pos.x - first_player.pos.x,
                  second_player.pos.y - first_player.pos.y,
                ) <= first_player.radius + second_player.radius
            };

            if can_rescue {
              if let Some(second_player) = players.get_mut(&second_id) {
                second_player.res();
              }
            }
          }
        }

        self.new_entities = area.get_packed_entities();

        // for (id, entity) in self.new_entities.iter() {
        //   if let Some(old_entity) = self.old_entities.get(&id) {
        //     let diff = old_entity.diff(&entity);
        //     if diff.len() > 0 {
        //       self.entities_diff.insert(*id, diff);
        //     }
        //   }
        // }

        // if !self.entities_diff.is_empty() {
        //   let package = Package::from(OneOfkind::self.entities_diff.clone());
        //   for id in area.players_id.iter() {
        //     network_bus.add_direct_package(*id, package);
        //   }
        // }
      }
    }
  }

  pub fn prepare_warps<'a>(&mut self, players: &HashMap<i64, Player>) -> HashMap<i64, Change> {
    let mut changes: HashMap<i64, Change> = HashMap::new();

    for (id, player) in players.iter() {
      if let Some(world) = self.worlds.get(&player.world) {
        if let Some(area) = world.areas.get(player.area as usize) {
          if player.pos.x + player.radius > area.raw_area.w + 8.0 * 32.0
            && self.get_next_area(player)
          {
            changes.insert(*id, Change::NextArea);
          }

          if player.area > 0
            && player.pos.x - player.radius < -8.0 * 32.0
            && self.get_prev_area(player)
          {
            changes.insert(*id, Change::PrevArea);
          }

          if player.area == 0
            && player.pos.x - player.radius < 0.0
            && player.pos.y - player.radius < 2.0 * 32.0
          {
            changes.insert(*id, Change::NextWorld);
          }

          if player.area == 0
            && player.pos.x - player.radius < 0.0
            && player.pos.y + player.radius > area.raw_area.h - 2.0 * 32.0
          {
            changes.insert(*id, Change::PrevWorld);
          }
        }
      }
    }

    changes
  }

  // pub fn process_warps<'a>(
  //   &'a mut self,
  //   players_manager: &'a mut PlayersManager<'a>,
  //   config: &Config,
  //   network_bus: &'a mut NetworkBus<'a>,
  // ) {
  //   let warps = self.prepare_warps(&players_manager.players);
  //   for (id, change) in &warps {
  //     if let Some(player) = players_manager.players.get_mut(&id) {
  //       match change {
  //         Change::NextArea => {
  //           if let Some(world) = self.worlds.get_mut(&player.world) {
  //             if let Some(area) = world.areas.get_mut(player.area as usize) {
  //               area.leave(player.id);
  //             }
  //             player.area += 1;
  //             player.pos.x = -8.0 * 32.0 + player.radius;
  //             let next_area = world.areas.get_mut(player.area as usize).unwrap();
  //             next_area.join(player.id);
  //             let area_init_package = OneOfkind::area_init(world.pack_area(player.area as usize));
  //             network_bus.add_direct_package(*id, area_init_package);
  //             let players_package = OneOfkind::players(players_manager.pack_players());
  //             network_bus.add_direct_package(*id, players_package);
  //           }
  //         }
  //         Change::PrevArea => {
  //           if let Some(world) = self.worlds.get_mut(&player.world) {
  //             if let Some(area) = world.areas.get_mut(player.area as usize) {
  //               area.leave(player.id);
  //             }
  //             player.area -= 1;
  //             let prev_area = world.areas.get_mut(player.area as usize).unwrap();
  //             prev_area.join(player.id);
  //             player.pos.x = prev_area.raw_area.w + 8.0 * 32.0 - player.radius;
  //             let area_init_package = OneOfkind::area_init(world.pack_area(player.area as usize));
  //             network_bus.add_direct_package(*id, area_init_package);
  //             let players_package = OneOfkind::players(players_manager.pack_players());
  //             network_bus.add_direct_package(*id, players_package);
  //           }
  //         }
  //         Change::NextWorld => {
  //           if let Some(prev_world) = self.worlds.get_mut(&player.world) {
  //             prev_world.leave(player);
  //           }
  //           let next_world_name = WorldsManager::get_next_world(&config.worlds, &player.world);
  //           let next_world = self.worlds.get_mut(&next_world_name).unwrap();
  //           let area = next_world.areas.get_mut(0).unwrap();
  //           player.world = next_world_name;
  //           player.pos.y = area.raw_area.h - player.radius - 2.0 * 32.0;
  //           next_world.join(player);
  //           let area_init_package =
  //             OneOfkind::area_init(next_world.pack_area(player.area as usize));
  //           network_bus.add_direct_package(*id, area_init_package);
  //           let players_package = OneOfkind::players(players_manager.pack_players());
  //           network_bus.add_direct_package(*id, players_package);
  //         }
  //         Change::PrevWorld => {
  //           if let Some(prev_world) = self.worlds.get_mut(&player.world) {
  //             prev_world.leave(player);
  //           }
  //           let prev_world_name = WorldsManager::get_prev_world(&config.worlds, &player.world);
  //           let prev_world = self.worlds.get_mut(&prev_world_name).unwrap();
  //           player.world = prev_world_name;
  //           player.pos.y = player.radius + 2.0 * 32.0;
  //           prev_world.join(player);
  //           let area_init_package =
  //             OneOfkind::area_init(prev_world.pack_area(player.area as usize));
  //           network_bus.add_direct_package(*id, area_init_package);
  //           let players_package = OneOfkind::players(players_manager.pack_players());
  //           network_bus.add_direct_package(*id, players_package);
  //         }
  //       };
  //     }
  //   }
  // }

  fn send_update_packages(&mut self) {}

  fn get_next_world(world_names: &Vec<String>, current_world: &String) -> String {
    let current_index = world_names.iter().position(|name| name == current_world);

    match current_index {
      Some(idx) if idx + 1 < world_names.len() => world_names[idx + 1].clone(),
      _ => world_names.get(0).unwrap().clone(),
    }
  }

  fn get_prev_world(world_names: &Vec<String>, current_world: &String) -> String {
    let current_index = world_names.iter().position(|name| name == current_world);

    match current_index {
      Some(idx) if idx > 0 => world_names[idx - 1].clone(),
      _ => world_names.get(world_names.len() - 1).unwrap().clone(),
    }
  }

  pub fn get_next_area(&self, player: &Player) -> bool {
    if let Some(world) = self.worlds.get(&player.world) {
      if let Some(_) = world.areas.get(player.area as usize + 1) {
        return true;
      }
    }
    false
  }

  pub fn get_prev_area(&self, player: &Player) -> bool {
    if let Some(world) = self.worlds.get(&player.world) {
      if let Some(_) = world.areas.get(player.area as usize - 1) {
        return true;
      }
    }
    false
  }
}
