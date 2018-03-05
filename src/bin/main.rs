extern crate game_engine_2d;

use game_engine_2d::game::*;

fn main() {
    let mut game = Game::new("SDL 테스트", 100, 100, 800, 600, false);

    while game.running() {
        // Process Event
        game.handle_event();

        // Move position of sprite

        // Update display.
        game.render();
    }
}
