use crate::mahjong::enums::Wind;
use crate::mahjong::hand::Hand;
use crate::mahjong::tile_values::TileValue;
use crate::mahjong::state::TurnResult;

pub struct Player {
    pub hand: Hand,
    pub score: i32,
    pub wind: Wind
}

trait GamePlayer {
}

impl Player {
    pub fn new(wind: Wind) -> Player {
        Player {
            wind: wind,
            score: 25000,
            hand: Hand::new()
        }
    }
    pub fn draw(&mut self, tiles: &mut Vec<TileValue>) -> TurnResult {
        self.hand.draw(&self.wind, tiles)
    }
}