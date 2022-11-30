use crate::mahjong_v2::calculation::ProvisionalHand;
use crate::mahjong_v2::game::{Tile, TilePile};

pub struct Player {
    hand: ClosedHand,
}

#[derive(Debug)]
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

pub enum TurnState {
    Tsumo,
    Discard(Tile),
    None,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hand: ClosedHand { tiles: Vec::new() },
        }
    }

    pub fn turn(&mut self, tile: Tile) -> TurnState {
        self.hand.add_tile(tile);
        if self.hand.needs_discard() {
            let provisional: ProvisionalHand = ProvisionalHand::new(&self.hand.tiles);
            if provisional.winning() {
                println!("{:?}", provisional);
                return TurnState::Tsumo;
            }
            if let Some(tile) = provisional.to_discard() {
                self.hand.discard_tile(&tile);
                return TurnState::Discard(tile);
            }
        }
        return TurnState::None;
    }
}

struct HandCalculator {}

impl HandCalculator {
    pub fn tile_values(hand: ClosedHand) -> Vec<(Tile, i32)> {
        let mut dedup_tiles = hand.tiles.clone();
        dedup_tiles.dedup_by(|a, b| *a == *b);
        dedup_tiles
            .iter()
            .map(|tile| match tile {
                Tile::Suit(_, _) => (*tile, 5),
                Tile::Dragon(_) => (*tile, 0),
                Tile::Wind(_) => (*tile, 0),
            })
            .collect()
    }
}
