use crate::mahjong_v2::calculation::ProvisionalHand;
use crate::mahjong_v2::game::Tile;

pub trait MahjongPlayer {
    fn turn(&mut self, tile: Tile) -> TurnState;
    fn offer_discard(&mut self, tile: Tile) -> DiscardState;
}

pub enum DiscardState {
    Ron,
    None,
}

pub enum TurnState {
    Tsumo,
    Discard(Tile),
    None,
}

pub struct Player {
    hand: ClosedHand,
}

#[derive(Debug, Clone)]
pub struct ClosedHand {
    pub tiles: Vec<Tile>,
}

impl ClosedHand {
    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn needs_discard(&self) -> bool {
        self.tiles.len() > 13
    }

    pub fn discard_tile(&mut self, tile: &Tile) -> Option<Tile> {
        if let Some(position) = self.tiles.iter().position(|a| *a == *tile) {
            Some(self.tiles.remove(position))
        } else {
            None
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            hand: ClosedHand { tiles: Vec::new() },
        }
    }
}

impl MahjongPlayer for Player {
    fn turn(&mut self, tile: Tile) -> TurnState {
        self.hand.add_tile(tile);
        if self.hand.needs_discard() {
            let provisional: ProvisionalHand = ProvisionalHand::new(&self.hand.tiles);
            if provisional.winning() {
                return TurnState::Tsumo;
            }
            if let Some(tile) = provisional.to_discard() {
                self.hand.discard_tile(&tile);
                return TurnState::Discard(tile);
            }
        }
        return TurnState::None;
    }

    fn offer_discard(&mut self, tile: Tile) -> DiscardState {
        let mut prospective_hand = self.hand.clone();
        prospective_hand.add_tile(tile);
        let provisional: ProvisionalHand = ProvisionalHand::new(&prospective_hand.tiles);
        if provisional.winning() {
            self.hand.add_tile(tile);
            return DiscardState::Ron;
        }
        return DiscardState::None;
    }
}
