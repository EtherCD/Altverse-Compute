use crate::proto::PackedPlayer;
use crate::resources::assets::heroes::Hero;
use crate::resources::player::Player;
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::{distance, Boundary, PlayerUpdateProps};

#[derive(Clone)]
pub struct Maven {
  player: Player,
  first_ability_active: bool,
}

impl Maven {
  pub fn new(props: JoinProps) -> Self {
    Self {
      player: Player::new(props),
      first_ability_active: false,
    }
  }
}

impl Hero for Maven {
  fn update(&mut self, props: &PlayerUpdateProps) {
    self.player.update(props);

    if self.first_ability_active {
      for player in props.players.iter() {
        if distance(
          player.pos.x - self.player.pos.x,
          player.pos.y - self.player.pos.y,
        ) <= 180.0 + player.radius
        {
          // player.res();
        }
      }
    }
  }

  fn input(&mut self, input: &mut Input) {
    self.player.input(input);
    if input.first_ability {
      self.first_ability_active = !self.first_ability_active;
      if self.first_ability_active {
        self.player.state = 1;
        self.player.state_meta = 180.0;
      } else {
        self.player.state = 0;
      }
    }
    input.first_ability = false;
  }

  fn knock(&mut self) {
    self.player.knock();
  }

  fn res(&mut self) {
    self.player.res();
  }

  fn collide(&mut self, boundary: Boundary) {
    self.player.collide(boundary);
  }

  fn pack(&self) -> PackedPlayer {
    self.player.pack()
  }

  fn player(&self) -> &Player {
    &self.player
  }

  fn player_mut(&mut self) -> &mut Player {
    &mut self.player
  }
}
