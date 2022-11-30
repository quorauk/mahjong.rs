use crate::mahjong::tile::Tile;
use crate::mahjong::tile_values::TileValue;
use crate::mahjong::tile_values::Suit::*;
use crate::mahjong::meld::Meld;
use crate::mahjong::hand::Hand;


impl Hand {
  pub fn winning(&self) -> Option<Vec<Meld>> {
    for tile in &self.closed_tiles {
      for meld in tile.possible_melds() {
        if let Some((hand, meld)) = self.split_meld(&meld) {
          if let Some(melds) = hand.winning() {
            let mut new_melds = melds.clone();
            new_melds.push(meld);
            return Some(new_melds)
          }
        }
      }
    }
    if self.closed_tiles.iter().count() == 2 {
      if self.closed_tiles[0] == self.closed_tiles[1] {
        let mut new_melds = Vec::new();
        new_melds.push(Meld::Pair(self.closed_tiles[0].clone()));
        return Some(new_melds)
      }
    }
    None
  }

  fn split_meld(&self, meld: &Meld) -> Option<(Hand, Meld)> {
    if self.has_meld(meld) {
      let mut new_hand = self.clone();
      match meld {
        Meld::Chow(a, b, c) => {
          if self.has_meld(meld) {
            new_hand.remove_tile(a);
            new_hand.remove_tile(b);
            new_hand.remove_tile(c);
          }
        },
        Meld::Pair(a) => {
          for _ in 0..2 {
            new_hand.remove_tile(a);
          }
        }
        Meld::Pung(a) => {
          for _ in 0..3 {
            new_hand.remove_tile(a);
          }
        }
      }
      return Some((new_hand, meld.clone()))
    }
    return None
  }

  fn has_meld(&self, meld: &Meld) -> bool {
    match meld {
      Meld::Pair(a) => self.has_tile(a) >= 2,
      Meld::Pung(a) => self.has_tile(a) >= 3,
      Meld::Chow(a, b, c) => self.has_tile(a) >= 1 && self.has_tile(b) >= 1 && self.has_tile(c) >= 1,
    }
  }

  fn has_tile(&self, tile: &TileValue) -> usize {
    self.closed_tiles.iter().filter(|a| *a == tile).count()
  }

  fn remove_tile(&mut self, tile: &TileValue) -> bool {
    if let Some(position) = self.closed_tiles.iter().position(|a| a == tile) {
      self.closed_tiles.remove(position);
      return true
    };
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pair_hand_wins() {
    let mut hand = Hand::new();
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    assert!(hand.winning().is_some())
  }

  #[test]
  fn chii_hand_wins() {
    let mut hand = Hand::new();
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Pin, 1));
    hand.closed_tiles.push(TileValue::Suit(Pin, 2));
    hand.closed_tiles.push(TileValue::Suit(Pin, 3));
    assert!(hand.winning().is_some())
  }

  #[test]
  fn pon_hand_wins() {
    let mut hand = Hand::new();
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Man, 2));
    hand.closed_tiles.push(TileValue::Suit(Man, 2));
    hand.closed_tiles.push(TileValue::Suit(Man, 2));
    assert!(hand.winning().is_some())
  }

  #[test]
  fn real_hand_wins() {
    let mut hand = Hand::new();
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Sou, 2));
    hand.closed_tiles.push(TileValue::Suit(Sou, 3));
    hand.closed_tiles.push(TileValue::Suit(Sou, 9));
    hand.closed_tiles.push(TileValue::Suit(Sou, 9));
    hand.closed_tiles.push(TileValue::Suit(Sou, 9));
    hand.closed_tiles.push(TileValue::Suit(Man, 3));
    hand.closed_tiles.push(TileValue::Suit(Man, 4));
    hand.closed_tiles.push(TileValue::Suit(Man, 5));
    hand.closed_tiles.push(TileValue::Suit(Pin, 7));
    hand.closed_tiles.push(TileValue::Suit(Pin, 8));
    hand.closed_tiles.push(TileValue::Suit(Pin, 9));
    hand.closed_tiles.push(TileValue::Suit(Man, 2));
    hand.closed_tiles.push(TileValue::Suit(Man, 2));
    assert!(hand.winning().is_some())
  }

  #[test]
  fn real_hand_loses() {
    let mut hand = Hand::new();
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Sou, 2));
    hand.closed_tiles.push(TileValue::Suit(Sou, 3));
    hand.closed_tiles.push(TileValue::Suit(Sou, 9));
    hand.closed_tiles.push(TileValue::Suit(Sou, 9));
    hand.closed_tiles.push(TileValue::Suit(Sou, 9));
    hand.closed_tiles.push(TileValue::Suit(Man, 3));
    hand.closed_tiles.push(TileValue::Suit(Man, 4));
    hand.closed_tiles.push(TileValue::Suit(Man, 5));
    hand.closed_tiles.push(TileValue::Suit(Pin, 7));
    hand.closed_tiles.push(TileValue::Suit(Pin, 8));
    hand.closed_tiles.push(TileValue::Suit(Pin, 9));
    hand.closed_tiles.push(TileValue::Suit(Man, 2));
    hand.closed_tiles.push(TileValue::Suit(Man, 1));
    assert!(hand.winning().is_none())
  }

  #[test]
  fn random_hand_loses() {
    let mut hand = Hand::new();
    hand.closed_tiles.push(TileValue::Suit(Sou, 1));
    hand.closed_tiles.push(TileValue::Suit(Sou, 3));
    hand.closed_tiles.push(TileValue::Suit(Sou, 5));
    hand.closed_tiles.push(TileValue::Suit(Sou, 7));
    hand.closed_tiles.push(TileValue::Suit(Sou, 9));
    hand.closed_tiles.push(TileValue::Suit(Man, 1));
    hand.closed_tiles.push(TileValue::Suit(Man, 3));
    hand.closed_tiles.push(TileValue::Suit(Man, 5));
    hand.closed_tiles.push(TileValue::Suit(Man, 7));
    hand.closed_tiles.push(TileValue::Suit(Man, 9));
    hand.closed_tiles.push(TileValue::Suit(Pin, 5));
    hand.closed_tiles.push(TileValue::Suit(Pin, 7));
    hand.closed_tiles.push(TileValue::Suit(Pin, 9));
    assert!(hand.winning().is_none())
  }
}