use std::collections::HashMap;

use serde_json::Value;

use crate::network::{PackedEntity, PackedPlayer};

impl PackedEntity {
  pub fn diff(&self, new: &PackedEntity) -> HashMap<String, Value> {
    let mut diff: HashMap<String, Value> = HashMap::new();

    if self.x != new.x {
      diff.insert("x".to_string(), Value::from(new.x));
    }
    if self.y != new.y {
      diff.insert("y".to_string(), Value::from(new.y));
    }
    if self.radius != new.radius {
      diff.insert("radius".to_string(), Value::from(new.radius));
    }
    if self.harmless != new.harmless {
      diff.insert("harmless".to_string(), Value::from(new.harmless));
    }
    if self.state != new.state {
      diff.insert("state".to_string(), Value::from(new.state));
    }
    if self.state_metadata != new.state_metadata {
      diff.insert(
        "state_metadata".to_string(),
        Value::from(new.state_metadata),
      );
    }
    if self.alpha != new.alpha {
      diff.insert("alpha".to_string(), Value::from(new.alpha));
    }

    diff
  }
}

impl PackedPlayer {
  pub fn diff(&self, new: &PackedPlayer) -> HashMap<String, Value> {
    let mut diff: HashMap<String, Value> = HashMap::new();

    if self.x != new.x {
      diff.insert("x".to_string(), Value::from(new.x));
    }
    if self.y != new.y {
      diff.insert("y".to_string(), Value::from(new.y));
    }
    if self.radius != new.radius {
      diff.insert("radius".to_string(), Value::from(new.radius));
    }
    if self.speed != new.speed {
      diff.insert("speed".to_string(), Value::from(new.speed));
    }
    if self.energy != new.energy {
      diff.insert("energy".to_string(), Value::from(new.energy));
    }
    if self.max_energy != new.max_energy {
      diff.insert("maxEnergy".to_string(), Value::from(new.max_energy));
    }
    if self.death_timer != new.death_timer {
      diff.insert("deathTimer".to_string(), Value::from(new.death_timer));
    }
    if self.state != new.state {
      diff.insert("state".to_string(), Value::from(new.state));
    }
    if self.state_meta != new.state_meta {
      diff.insert("stateMeta".to_string(), Value::from(new.state_meta));
    }
    if self.area != new.area {
      diff.insert("area".to_string(), Value::from(new.area));
    }
    if self.world != new.world {
      diff.insert("world".to_string(), Value::from(new.world.clone()));
    }
    if self.died != new.died {
      diff.insert("died".to_string(), Value::from(new.died));
    }

    diff
  }
}
