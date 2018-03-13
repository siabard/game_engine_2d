extern crate game_engine_2d;
extern crate sdl2;

use game_engine_2d::app::*;
fn main() {
    let mut app = App::new("SDL 테스트", 100, 100, 800, 600, false);

    // 이미지를 등록한다.

    app.add_asset("char", "assets/char.png");
    app.add_asset("enemy", "assets/enemy.png");
    app.run_loop();
}
