extern crate game_engine_2d;
extern crate sdl2;

use game_engine_2d::app::*;
fn main() {
    let mut app = App::new("SDL 테스트", 100, 100, 800, 600, false);

    app.run();
}
