use crate::mahjong::tile_values::TileValue;

#[derive(Clone, Copy, Debug)]
pub enum Meld {
  Pung(TileValue),
  Chow(TileValue, TileValue, TileValue),
  Pair(TileValue)
}