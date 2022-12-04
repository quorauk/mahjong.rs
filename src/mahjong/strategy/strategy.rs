use crate::mahjong::tile::mahjong_tile::MahjongTile;

pub trait Strategy {
  fn winning(&self, hand: &Vec<MahjongTile>) -> bool;
  fn discard(&self, hand: &Vec<MahjongTile>) -> Vec<(MahjongTile, i64)>;
}