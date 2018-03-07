extern crate game_engine_2d;
extern crate sdl2;

use game_engine_2d::game::*;
use game_engine_2d::app::*;
use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
use std::path::Path;
fn main() {
    let mut app = App::new("SDL 테스트", 100, 100, 800, 600, false);

    // 이미지를 등록한다.
    let png = "assets/char.png";

    app.add_asset(png);
    app.run_loop();
}
