use crate::mahjong::tile_values::TileValue;
use crate::mahjong::enums::Wind;

use super::state::TurnResult;

pub mod utils;

#[derive(Clone)]
pub struct Hand {
  pub closed_tiles: Vec<TileValue>
}

impl Hand {
  pub fn new() -> Hand {
    Hand {
      closed_tiles: Vec::new(),
    }
  }

  pub fn draw(&mut self, wind: &Wind, tile_pile: &mut Vec<TileValue>) -> TurnResult {
    let drawn_tile = tile_pile.pop();
    match drawn_tile {
      Some(tile) =>  {
        if self.closed_tiles.len() == 13 {
          if let Some(res) = self.winning() {
            println!("{:?}: Tsumo'ed with {:?}", wind, res);
            TurnResult::Tsumo
          } else {
            println!("{:?}: Drew {:?} discarded {:?}", wind, tile, tile);
            TurnResult::None
          }
        } else {
          self.closed_tiles.push(tile);
          TurnResult::None
        }
      }
      _ => TurnResult::None
    }
  }
}
