#![feature(slice_group_by)]

// mod mahjong;

// use mahjong::state::{Game, GameState};

// fn main() {
//     let mut state = GameState::new();
//     state.deal_to_players();

//     while !state.round_complete() {
//         state.player_round();
//         state.next_player();
//     }
// }

mod mahjong_v2;

use mahjong_v2::game::Game;

fn main() {
    for _ in 1..20 {
        let mut game = Game::new();
        game.run()
    }
}
