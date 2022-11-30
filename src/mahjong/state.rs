use crate::mahjong::player::Player;
use crate::mahjong::hand::{Hand};
use crate::mahjong::tile_values::{TileValue, Dragon};
use crate::mahjong::tile_values::Suit::*;
use crate::mahjong::enums::*;
use strum::IntoEnumIterator;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub trait Game {
  fn new() -> Self;
  fn deal_to_players(&mut self);
  fn round_complete(&self) -> bool;
  fn next_player(&mut self);
  fn player_round(&mut self);
}

pub struct GameState {
  pub wall: Vec<TileValue>,
  pub dead_wall: Vec<TileValue>,
  pub players: Vec<Player>,
  pub current_step: Wind
}

impl GameState {
  fn build_wall(&mut self) {
    for number in 1..9 {
        for _ in 0..4 {
          self.wall.push(TileValue::Suit(Man, number));
          self.wall.push(TileValue::Suit(Pin, number));
          self.wall.push(TileValue::Suit(Sou, number))
        }
    }
    for _ in 0..4 {
        for wind in Wind::iter() {
          self.wall.push(TileValue::Wind(wind))
        }
    }
    for _ in 0..4 {
        for dragon in Dragon::iter() {
          self.wall.push(TileValue::Dragon(dragon))
        }
    }
    self.wall.shuffle(&mut thread_rng());
  }

  fn generate_players(&mut self) {
    for wind in Wind::iter() {
      self.players.push(Player::new(wind))
    }
  }

  fn split_dead_wall(&mut self) {
    self.dead_wall = self.wall.drain(0..14).collect::<Vec<TileValue>>();
  }
}

pub enum TurnResult {
  Tsumo,
  None
}

impl Game for GameState {
  fn new() -> GameState {
    let mut state = GameState {
        wall: Vec::new(),
        dead_wall: Vec::new(),
        players: Vec::new(),
        current_step: Wind::East
    };

    state.build_wall();
    state.split_dead_wall();
    state.generate_players();
    state
  }


  fn deal_to_players(&mut self) {
    for player in &mut self.players {
      while !self.wall.is_empty() {
        player.draw(&mut self.wall);
      }
    }
  }

  fn round_complete(&self) -> bool {
    self.wall.is_empty()
  }

  fn player_round(&mut self) {
    let player = self.players.iter_mut().find( |player| player.wind == self.current_step );
    match player {
      Some(player) => {
        player.draw(&mut self.wall);
        ()
      },
      _ => ()
    };
  }

  fn next_player(&mut self) {
    self.current_step = self.current_step.next()
  }
}