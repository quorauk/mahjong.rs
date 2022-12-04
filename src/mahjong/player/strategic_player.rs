use crate::mahjong::{player::player::Player, player::player::TurnState, strategy::{strategy::{Strategy}, block_strategy::BlockStrategy}, tile::mahjong_tile::MahjongTile};


pub struct StrategicPlayer {
    strategy: Box<dyn Strategy>
}

impl StrategicPlayer {
    pub fn new() -> Self {
        StrategicPlayer {
            strategy: Box::new(BlockStrategy{})
        }
    }
}

impl Default for StrategicPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for StrategicPlayer {
    fn turn(&self, hand: &Vec<MahjongTile>) -> TurnState {
        if hand.len() < 14 {
            return TurnState::Draw
        }
        if self.strategy.winning(hand) {
            return TurnState::Tsumo
        }
        let mut x = self.strategy.discard(hand);
        x.sort_by(|(_, a), (_, b)| a.cmp(b));
        TurnState::Discard(x.first().unwrap().0)
    }
}