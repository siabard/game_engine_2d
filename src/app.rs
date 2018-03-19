//! # Main application
//!
//! 이 모듈은 SDL의 초기화와 ASSET의 설정을 담당한다.
extern crate sdl2;
extern crate std;

use sdl2::pixels::Color;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl_engine::*;
use ecs::*;
use components::*;

/// App
pub struct App {
    title: &'static str,
    xpos: i32,
    ypos: i32,
    width: u32,
    height: u32,
    fullscreen: bool,
}

impl App {
    /// creator

    pub fn new(
        title: &'static str,
        xpos: i32,
        ypos: i32,
        width: u32,
        height: u32,
        fullscreen: bool,
    ) -> Self {
        let app = App {
            title: title,
            xpos: xpos,
            ypos: ypos,
            width: width,
            height: height,
            fullscreen: fullscreen,
        };

        app
    }

    /// run program
    pub fn run(&mut self) {
        let context = sdl2::init().expect("Cannot Initialize SDL2");
        let video_subsystem = context.video().unwrap();
        let timer = context.timer().unwrap();
        let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

        let window = if self.fullscreen {
            video_subsystem
                .window(self.title, self.width, self.height)
                .position(self.xpos, self.ypos)
                .fullscreen()
                .build()
                .unwrap()
        } else {
            video_subsystem
                .window(self.title, self.width, self.height)
                .position(self.xpos, self.ypos)
                .build()
                .unwrap()
        };

        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let event_pump = context.event_pump().unwrap();
        let texture_creator = canvas.texture_creator();

        let mut engine: SdlEngine = SdlEngine::new(canvas, event_pump, &texture_creator, timer);

        //engine.add_sprite("char", "assets/char.png", 0, 0);
        //engine.add_sprite("enemy", "assets/enemy.png", 50, 50);

        let mut player1 = Entity::new("player1".to_owned());
        let mut player2 = Entity::new("enemy".to_owned());

        player1.add_component::<PositionComponent>(PositionComponent { xpos: 50, ypos: 50 });
        player1.add_component::<SpriteComponent>(SpriteComponent {
            sprite_id: "char",
            sprite_path: "assets/char.png",
        });

        player2.add_component::<PositionComponent>(PositionComponent {
            xpos: 100,
            ypos: 100,
        });
        player2.add_component::<SpriteComponent>(SpriteComponent {
            sprite_id: "enemy",
            sprite_path: "assets/enemy.png",
        });
        engine.add_entity("char", player1);
        engine.add_entity("enemy", player2);

        engine.init_sprite();

        engine.game_loop();
    }
}
