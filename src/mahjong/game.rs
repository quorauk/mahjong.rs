
use strum::IntoEnumIterator;
use crate::mahjong::tile::enums::*;

use super::player::player::TurnState;
use super::player_state::player_state::PlayerState;
use super::tile::mahjong_tile::MahjongTile;
use super::wall::wall::Wall;


pub struct Game {
    wall: Wall,
    dead_wall: Vec<MahjongTile>,
    players: Vec<PlayerState>,
    current_round: Wind,
}

impl Game {
    pub fn new() -> Self {
        let mut wall = Wall::new();
        let dead_wall = wall.split_dead_wall();
        let players : Vec<PlayerState> = Wind::iter().map ( |w| PlayerState::new(w) ).collect();
        Game {
            wall,
            dead_wall,
            players,
            current_round: Wind::East,
        }
    }

    pub fn run(&mut self) {
        while self.wall.has_tiles() {
            match self.turn() {
                GameTurnState::Win(wind) => {
                    println!("{:?} wins", wind);
                    return;
                }
                GameTurnState::Chombo(wind) => {
                    println!("{:?} chombo'ed", wind);
                    return;
                }
                _ => (),
            }
        }
        println!("Exhaustive Draw");
    }

    fn turn(&mut self) -> GameTurnState {
        let player_state = self.players.iter_mut().find(|x| x.wind() == self.current_round).unwrap();
        if let Some(tile) = self.wall.draw() {
            match player_state.turn(tile) {
                TurnState::Tsumo => {
                    println!("We got a tsumo from {:?}", self.current_round);
                    return GameTurnState::Win(self.current_round);
                }
                TurnState::Discard(discarded) => {
                    match player_state.discard(discarded) {
                        Ok(_tile) => (),
                        Err(_) => return GameTurnState::Chombo(player_state.wind()),
                    }
                }
                TurnState::Draw => (),
                TurnState::Riichi(_) => todo!(),
            }
        }

        self.progress_round();
        return GameTurnState::None;
    }

    // fn handle_discard(&mut self, tile: MahjongTile) -> GameTurnState {
    //     for player_state in &mut self.players {
    //         if player_state.wind != self.current_round {
    //             // match player_state.player.offer_discard(tile) {
    //             //     DiscardState::None => (),
    //             //     DiscardState::Ron => {
    //             //         println!("We got a ron from {:?}", player_state.wind);
    //             //         return GameTurnState::Win(player_state.wind);
    //             //     }
    //             // }
    //         }
    //     }
    //     let current_player = self.players.iter_mut().find(|x| x.wind == self.current_round).unwrap();
    //     current_player.discards.push(tile);
    //     GameTurnState::None
    // }

    fn progress_round(&mut self) {
        self.current_round = match self.current_round {
            Wind::East => Wind::North,
            Wind::North => Wind::West,
            Wind::West => Wind::South,
            Wind::South => Wind::East,
        };
    }
}

enum GameTurnState {
    None,
    Chombo(Wind),
    Win(Wind),
}
