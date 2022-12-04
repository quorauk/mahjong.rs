use crate::mahjong::{tile::mahjong_tile::MahjongTile};

// Players handle an AI or Human player
// Players have no access to their own hand state, they are simply given references
// this prevents players from cheating
pub trait Player {
  fn turn(&self, game_state: &Vec<MahjongTile>) -> TurnState;
}

pub enum TurnState {
  Draw,
  Discard(MahjongTile),
  Riichi(MahjongTile),
  Tsumo,
}