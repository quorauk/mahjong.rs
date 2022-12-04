use std::collections::HashMap;

use crate::mahjong::tile::mahjong_tile::MahjongTile;

use super::strategy::Strategy;

pub struct BlockStrategy {}

impl Strategy for BlockStrategy {
    fn winning(&self, hand: &Vec<MahjongTile>) -> bool {
      get_all_blocks(hand).iter().any(|b| b.winning())
    }

    fn discard(&self, hand: &Vec<MahjongTile>) -> Vec<(MahjongTile, i64)> {
      let mut blocks = get_all_blocks(hand);
      blocks.sort_by(|b, c| b.floating_tiles.len().cmp(&c.floating_tiles.len()));
      if let Some(block) = blocks.first() {
        if block.floating_tiles.len() > 0 {
          return block.floating_tiles.iter().map(|t| (*t, 1)).collect()
        } else if block.protoruns.len() > 0 {
          return block.protoruns.iter().map(|p| (p.tiles, p.possible_melds().len() as i64))
            .flat_map(|(tiles, p)| tiles.map(|t| (t, p))).collect()
        } else {
          return hand.iter().map(|t| (*t, 1)).collect()
          // block.protoruns.iter().flat_map(|proto| proto.tiles).map(|t| (*t, 1)).collect()
        };
      }
      vec![(*hand.first().unwrap(), 1)]
    }
}

impl BlockStrategy {
}

fn get_all_blocks(tiles: &Vec<MahjongTile>) -> Vec<Block> {
  let mut blocks = Vec::new();

  let mut tile_count = HashMap::new();
  for tile in tiles.clone() {
    tile_count
      .entry(tile)
      .and_modify(|count| *count += 1)
      .or_insert(1);
  }
  for (tile, count) in tile_count {
    if count >= 2 {
      blocks.push(find_blocks_assuming_pair(tiles.clone(), tile))
    }
  }

  blocks
}

fn find_blocks_assuming_pair(tiles: Vec<MahjongTile>, tile: MahjongTile) -> Block {
  let mut tiles_without_pair = tiles.clone();
  for _ in 0..2 {
    let position = tiles_without_pair.iter().position(|x| *x == tile);
    if let Some(position) = position {
      tiles_without_pair.remove(position);
    }
  }

  let (melds, floating_tiles) = find_melds(tiles_without_pair);
  let (protoruns, floating_tiles) = find_protoruns(floating_tiles);

  Block { pair: (tile, tile), melds, protoruns, floating_tiles}
}

fn find_protoruns(floating_tiles: Vec<MahjongTile>) -> (Vec<Protorun>, Vec<MahjongTile>) {
  let mut new_tiles = floating_tiles.clone();
  for tile in floating_tiles {
    for protorun in tile.possible_protoruns() {
      if protorun.included(&new_tiles) {
        protorun.remove(&mut new_tiles);
        let (mut results, remaining) = find_protoruns(new_tiles);
        results.push(protorun);
        return (results, remaining)
      }
    }
  }
  (Vec::new(), new_tiles)
}

fn find_melds(tiles: Vec<MahjongTile>) -> (Vec<Meld>, Vec<MahjongTile>) {
  let mut new_tiles = tiles.clone();
  for tile in tiles {
    for meld in tile.possible_melds() {
      if meld.included(&new_tiles) {
        meld.remove(&mut new_tiles);
        let (mut results, remaining) = find_melds(new_tiles);
        results.push(meld);
        return (results, remaining)
      }
    }
  }
  (Vec::new(), new_tiles)
}

#[derive(Debug)]
struct Block {
  pair: (MahjongTile, MahjongTile),
  melds: Vec<Meld>,
  protoruns: Vec<Protorun>,
  floating_tiles: Vec<MahjongTile>
}

impl Block {
  pub fn winning(&self) -> bool {
    self.melds.len() == 4
  }
}

#[derive(Debug, PartialEq)]
pub struct Chow {
  tiles: [MahjongTile; 3]
}

#[derive(Debug, PartialEq)]
pub struct Pung {
  tiles: [MahjongTile; 3]
}

#[derive(Debug, PartialEq)]
pub enum Meld {
  Chow(Chow),
  Pung(Pung)
}

impl Pung {
  pub fn new(tile: MahjongTile) -> Self {
    Pung {
      tiles: [tile; 3]
    }
  }
}

impl Chow {
  pub fn new(a: MahjongTile, b: MahjongTile, c: MahjongTile) -> Self {
    Chow {
      tiles: [a, b, c]
    }
  }
}

trait Removeable {
  fn remove(&self, tiles: &mut Vec<MahjongTile>);
  fn included(&self, tiles: &Vec<MahjongTile>) -> bool;
}

impl Removeable for Meld {
    fn remove(&self, tiles: &mut Vec<MahjongTile>) {
       match self {
          Meld::Chow(chow) => chow.remove(tiles),
          Meld::Pung(pung) => pung.remove(tiles),
       };
    }

    fn included(&self, tiles: &Vec<MahjongTile>) -> bool {
       match self {
          Meld::Chow(chow) => chow.included(tiles),
          Meld::Pung(pung) => pung.included(tiles),
       }
    }
}

impl Removeable for Pung {
    fn remove(&self, tiles: &mut Vec<MahjongTile>) {
      for tile in self.tiles {
        let position = tiles.iter().position(|x| *x == tile);
        if let Some(position) = position {
          tiles.remove(position);
        }
      }
    }

    fn included(&self, tiles: &Vec<MahjongTile>) -> bool {
      tiles.iter().filter(|t| **t == self.tiles[0]).count() > 2
    }
}

impl Removeable for Chow {
    fn remove(&self, tiles: &mut Vec<MahjongTile>) {
      for tile in self.tiles {
        let position = tiles.iter().position(|t| *t == tile);
        match position {
          Some(position) => { tiles.remove(position); }
          None => { println!("PROBLEM") }
        }
      };
    }

    fn included(&self, tiles: &Vec<MahjongTile>) -> bool {
      self.tiles.iter().all(|self_tile| tiles.contains(self_tile))
    }
}

#[derive(Debug, PartialEq)]
pub struct Protorun {
  pub tiles: [MahjongTile; 2]
}

impl Protorun {
  fn possible_melds(&self) -> Vec<Meld> {
    let [a, b] = self.tiles;
    let a_melds = a.possible_melds();
    let b_melds = b.possible_melds();
    let mut out = Vec::new();
    for meld in a_melds {
      if b_melds.contains(&meld) {
        out.push(meld)
      }
    }
    out
  }
}

impl Removeable for Protorun {
    fn remove(&self, tiles: &mut Vec<MahjongTile>) {
      if !self.included(tiles) {
        println!("WTF?!");
        return
      }
      for tile in self.tiles {
        let position = tiles.iter().position(|t| *t == tile);
        match position {
          Some(position) => { tiles.remove(position); }
          None => { println!("PROBLEM") }
        }
      };
    }

    fn included(&self, tiles: &Vec<MahjongTile>) -> bool {
      let [a, b] = self.tiles;
      if a == b {
        tiles.iter().filter(|t| **t == self.tiles[0]).count() > 1
      } else {
        self.tiles.iter().all(|self_tile| tiles.contains(self_tile))
      }
    }
}


#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::mahjong::tile::{mahjong_tile::MahjongTile, enums::{Wind, Suit, Dragon}};

  use super::*;

  fn gen_chii(suit: Suit, start: i8) -> Vec<MahjongTile> {
    (start..start+3).into_iter().map(|v| MahjongTile::new_suit(suit, v)).collect()
  }

  fn gen_pung(tile: MahjongTile) -> Vec<MahjongTile> {
    (0..3).into_iter().map(|_| tile.clone() ).collect()
  }

  fn gen_x_copies(tile: MahjongTile, x: i32) -> Vec<MahjongTile> {
    (0..x).into_iter().map(|_| tile.clone() ).collect()
  }

  #[test]
  fn it_works() {
    let strategy = BlockStrategy{};
    let mut hand = Vec::new();
    hand.extend(gen_chii(Suit::Man, 1));
    hand.extend(gen_chii(Suit::Man, 6));
    hand.extend(gen_chii(Suit::Sou, 3));
    hand.extend(gen_chii(Suit::Pin, 4));
    hand.extend(gen_x_copies(MahjongTile::Dragon(Dragon::Red), 2));

    assert_eq!(
      strategy.winning(&hand),
      true
    );
    let strategy = BlockStrategy{};
    let mut hand = Vec::new();
    hand.extend(gen_chii(Suit::Man, 1));
    hand.extend(gen_chii(Suit::Man, 6));
    hand.extend(gen_chii(Suit::Sou, 3));
    hand.extend(gen_x_copies(MahjongTile::Dragon(Dragon::Red), 2));
    hand.push(MahjongTile::new_suit(Suit::Pin, 4));
    hand.push(MahjongTile::new_suit(Suit::Pin, 6));
    hand.push(MahjongTile::new_suit(Suit::Pin, 7));
    assert_eq!(
      strategy.winning(&hand),
      false
    )
  }

  #[test]
  fn chow_included() {
    let tiles = vec![
      MahjongTile::new_suit(Suit::Sou, 1),
      MahjongTile::new_suit(Suit::Sou, 2),
      MahjongTile::new_suit(Suit::Sou, 3),
      MahjongTile::new_suit(Suit::Man, 1),
      MahjongTile::new_suit(Suit::Man, 2),
      MahjongTile::new_suit(Suit::Man, 3),
    ];
    assert_eq!(Chow::new(
        MahjongTile::new_suit(Suit::Man, 1),
        MahjongTile::new_suit(Suit::Man, 2),
        MahjongTile::new_suit(Suit::Man, 3),
      ).included(&tiles), true
    );
    assert_eq!(Chow::new(
        MahjongTile::new_suit(Suit::Man, 2),
        MahjongTile::new_suit(Suit::Man, 3),
        MahjongTile::new_suit(Suit::Man, 4),
      ).included(&tiles), false
    )
  }

  #[test]
  fn proto_included() {
    let tiles = vec![
      MahjongTile::new_suit(Suit::Sou, 1),
      MahjongTile::new_suit(Suit::Sou, 2),
      MahjongTile::new_suit(Suit::Sou, 3),
      MahjongTile::new_suit(Suit::Man, 1),
      MahjongTile::new_suit(Suit::Man, 2),
      MahjongTile::new_suit(Suit::Man, 3),
    ];
    assert_eq!(Protorun{
      tiles: [
        MahjongTile::new_suit(Suit::Man, 1),
        MahjongTile::new_suit(Suit::Man, 3),
      ]}.included(&tiles), true
    );
    assert_eq!(Protorun{
      tiles: [
        MahjongTile::new_suit(Suit::Man, 3),
        MahjongTile::new_suit(Suit::Man, 4)
      ]}.included(&tiles), false
    )
  }
}