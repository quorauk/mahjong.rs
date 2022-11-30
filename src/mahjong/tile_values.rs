use crate::mahjong::enums::Wind;
use crate::mahjong::tile::Tile;
use crate::mahjong::meld::Meld;
use strum_macros::EnumIter;
use std::cmp::min;

#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum Dragon {
  White,
  Green,
  Red
}

#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
  Pin,
  Man,
  Sou
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum TileValue {
  Wind(Wind),
  Dragon(Dragon),
  Suit(Suit, i8),
}

impl Tile for TileValue {
  fn is_simple(self) -> bool {
    !self.is_honor()
  }

  fn is_honor(self) -> bool {
    match self {
      TileValue::Dragon(_) => true,
      TileValue::Wind(_) => true,
      _ => false
    }
  }

  fn value(&self) -> String {
    match self {
      TileValue::Wind(wind) => {
        String::from(match wind {
          Wind::South => "South",
          Wind::West => "West",
          Wind::North => "North",
          Wind::East => "East"
        })
      }
      TileValue::Dragon(dragon) => {
        String::from(match dragon {
          Dragon::Green => "Green",
          Dragon::Red => "Red",
          Dragon::White => "White",
        })
      }
      TileValue::Suit(x, i) => format!("{} {:?}", i, x),
    }
  }
}

impl TileValue {
  pub fn possible_melds(&self) -> Vec<Meld> {
    let mut possible = Vec::new();

    match self {
      TileValue::Suit(suit, val) => {
        for value in min(*val-2, 1)..*val {
          possible.push(
            Meld::Chow(TileValue::Suit(*suit, value), TileValue::Suit(*suit, value + 1), TileValue::Suit(*suit, value + 2))
          );
        }
      },
      _ => ()
    }

    possible.push(Meld::Pung(self.clone()));

    possible
  }


}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tile_values_work() {
    let x = TileValue::Wind(Wind::South);
    println!("{:?}", x);
  }
}