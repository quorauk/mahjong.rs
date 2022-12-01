use crate::mahjong_v2::player::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::player::TurnState;

#[derive(EnumIter, Clone, Copy, PartialEq, Debug)]
pub enum Wind {
    North,
    East,
    South,
    West,
}

#[derive(EnumIter, Clone, Copy, PartialEq, Debug)]
pub enum Dragon {
    Green,
    White,
    Red,
}

#[derive(EnumIter, Clone, Copy, PartialEq, Debug)]
pub enum Suit {
    Pin,
    Man,
    Sou,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Wind(Wind),
    Dragon(Dragon),
    Suit(Suit, i8),
}

pub struct Game {
    tile_pile: TilePile,
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Self {
        let mut tiles = Vec::new();
        for _ in 0..4 {
            Wind::iter().for_each(|wind| tiles.push(Tile::Wind(wind)));
            Dragon::iter().for_each(|dragon| tiles.push(Tile::Dragon(dragon)));
            Suit::iter().for_each(|suit| {
                for val in 1..9 {
                    tiles.push(Tile::Suit(suit, val))
                }
            });
        }
        tiles.shuffle(&mut thread_rng());
        let players = vec![Player::new(), Player::new(), Player::new(), Player::new()];
        Game {
            tile_pile: TilePile {
                discards: Vec::new(),
                tiles: tiles,
            },
            players: players,
        }
    }

    pub fn run(&mut self) {
        while self.tile_pile.has_tiles() {
            for player in self.players.iter_mut() {
                if let Some(tile) = self.tile_pile.draw() {
                    match player.turn(tile) {
                        TurnState::Tsumo => {
                            println!("PLAYER WINS");
                            return;
                        }
                        TurnState::Discard(tile) => self.tile_pile.discard(tile),
                        TurnState::None => (),
                    }
                }
            }
        }
    }
}

pub struct TilePile {
    discards: Vec<Tile>,
    tiles: Vec<Tile>,
}

impl TilePile {
    pub fn draw(&mut self) -> Option<Tile> {
        self.tiles.pop()
    }

    pub fn discard(&mut self, tile: Tile) {
        self.discards.push(tile);
    }

    pub fn has_tiles(&self) -> bool {
        !self.tiles.is_empty()
    }
}
