use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;

use crate::mahjong::tile::{mahjong_tile::MahjongTile, enums::{Wind, Dragon, Suit}};

pub struct Wall {
  tiles: Vec<MahjongTile>
}

impl Wall {
  pub fn new() -> Self {
    let mut tiles = Vec::new();
    for _ in 0..4 {
        Wind::iter().for_each(|wind| tiles.push(MahjongTile::Wind(wind)));
        Dragon::iter().for_each(|dragon| tiles.push(MahjongTile::Dragon(dragon)));
        Suit::iter().for_each(|suit| {
            for value in 1..9 {
                tiles.push(MahjongTile::new_suit(suit, value))
            }
        });
    }
    tiles.shuffle(&mut thread_rng());
    Wall{
      tiles
    }
  }

  pub fn split_dead_wall(&mut self) -> Vec<MahjongTile> {
    let tiles = self.tiles.clone();
    let (wall, deadwall) = tiles.split_at(tiles.len() - 15);
    self.tiles = wall.to_vec();
    deadwall.to_vec()
  }

  pub fn has_tiles(&self) -> bool {
    self.tiles.len() > 0
  }

  pub fn draw(&mut self) -> Option<MahjongTile> {
    self.tiles.pop()
  }
}