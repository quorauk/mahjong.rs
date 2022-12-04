use crate::mahjong::{tile::{enums::Wind, mahjong_tile::MahjongTile}, player::{strategic_player::StrategicPlayer, player::{TurnState, Player}}};

pub struct PlayerState {
  wind: Wind,
  discards: Vec<MahjongTile>,
  player: StrategicPlayer,
  hand: Vec<MahjongTile>
}

impl PlayerState {
    pub fn new(wind: Wind) -> Self {
        PlayerState { player: StrategicPlayer::new(), hand: Vec::new(), wind, discards: Vec::new() }
    }

    pub fn wind(&self) -> Wind {
      self.wind
    }

    pub fn turn(&mut self, tile: MahjongTile) -> TurnState {
      self.hand.push(tile.clone());
      self.player.turn(&self.hand)
    }

    pub fn discard(&mut self, tile: MahjongTile) -> Result<TurnState, &str> {
      let tile_position = self.hand.iter().position(|t| *t == tile);
      if let Some(tile_position) = tile_position {
        self.hand.remove(tile_position);
        Ok(TurnState::Discard(tile))
      } else {
        Err("Missing tile")
      }
    }
}