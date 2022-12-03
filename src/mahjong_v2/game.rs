
use std::hash::Hash;

use crate::mahjong_v2::player::{MahjongPlayer, Player};
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::mahjong_v2::player::{DiscardState, TurnState};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    // players: HashMap<Wind, Player>,
    players: Vec<PlayerState>,
    current_round: Wind,
}

struct PlayerState {
    player: Player,
    wind: Wind,
    discards: Vec<Tile>
}

impl PlayerState {
    fn new(wind: Wind) -> Self {
        PlayerState { player: Player::new(), wind: wind, discards: Vec::new() }
    }
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
        let players : Vec<PlayerState> = Wind::iter().map ( |w| PlayerState::new(w) ).collect();
        Game {
            tile_pile: TilePile {
                discards: Vec::new(),
                tiles: tiles,
            },
            players,
            current_round: Wind::East,
        }
    }

    pub fn run(&mut self) {
        while self.tile_pile.has_tiles() {
            match self.turn() {
                GameTurnState::Win(wind) => {
                    println!("{:?} wins", wind);
                    return;
                }
                _ => (),
            }
        }
        println!("Exhaustive Draw");
    }

    fn turn(&mut self) -> GameTurnState {
        let player_state = self.players.iter_mut().find(|x| x.wind == self.current_round).unwrap();
        if let Some(tile) = self.tile_pile.draw() {
            match player_state.player.turn(tile) {
                TurnState::Tsumo => {
                    println!("We got a tsumo from {:?}", self.current_round);
                    return GameTurnState::Win(self.current_round);
                }
                TurnState::Discard(tile) => {
                    if let GameTurnState::Win(wind) = self.handle_discard(tile) {
                        return GameTurnState::Win(wind);
                    }
                }
                TurnState::None => (),
            }
        }

        self.progress_round();
        return GameTurnState::None;
    }

    fn handle_discard(&mut self, tile: Tile) -> GameTurnState {
        for player_state in &mut self.players {
            if player_state.wind != self.current_round {
                match player_state.player.offer_discard(tile) {
                    DiscardState::None => (),
                    DiscardState::Ron => {
                        println!("We got a ron from {:?}", player_state.wind);
                        return GameTurnState::Win(player_state.wind);
                    }
                }
            }
        }
        let current_player = self.players.iter_mut().find(|x| x.wind == self.current_round).unwrap();
        current_player.discards.push(tile);
        GameTurnState::None
    }

    fn progress_round(&mut self) {
        self.current_round = match self.current_round {
            Wind::East => Wind::North,
            Wind::North => Wind::West,
            Wind::West => Wind::South,
            Wind::South => Wind::East,
        };
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

enum GameTurnState {
    None,
    Win(Wind),
}
