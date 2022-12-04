use crate::mahjong::tile::{mahjong_tile::MahjongTile};

use super::strategy::Strategy;


pub struct DumbStrategy;

impl Strategy for DumbStrategy {
    fn discard(&self, tiles: &Vec<MahjongTile>) -> Vec<(MahjongTile, i64)> {
      vec![(*tiles.last().unwrap(), 1)]
    }

    fn winning(&self, _hand: &Vec<MahjongTile>) -> bool {
      false
    }
}