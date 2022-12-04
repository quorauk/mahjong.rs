mod mahjong;

use mahjong::game::Game;

fn main() {
    let mut game = Game::new();
    game.run()
}
