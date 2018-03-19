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
use vector_2d::*;
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

        // Adding some resource...
        engine.add_texture("char", "assets/char.png");
        engine.add_texture("enemy", "assets/enemy.png");

        // Initiate Entities
        let mut player1 = Entity::new("player1".to_owned());
        let mut enemy = Entity::new("enemy".to_owned());

        // Entity Player 1 Init
        player1.add_component::<TransformComponent>(TransformComponent {
            entity_id: "player1".to_owned(),
            position: Vector2D { x: 50.0, y: 50.0 },
        });
        player1.add_component::<SpriteComponent>(SpriteComponent {
            entity_id: "player1".to_owned(),
            texture_id: "char".to_owned(),
            source_rect: sdl2::rect::Rect::new(0, 0, 32, 32),
            dest_rect: sdl2::rect::Rect::new(0, 0, 64, 64),
        });

        // Entity Player 2 Init
        enemy.add_component::<TransformComponent>(TransformComponent {
            entity_id: "enemy".to_owned(),
            position: Vector2D { x: 100.0, y: 100.0 },
        });
        enemy.add_component::<SpriteComponent>(SpriteComponent {
            entity_id: "enemy".to_owned(),
            texture_id: "enemy".to_owned(),
            source_rect: sdl2::rect::Rect::new(0, 0, 32, 32),
            dest_rect: sdl2::rect::Rect::new(0, 0, 64, 64),
        });

        // Add entities to engine
        engine.add_entity("player1", player1);
        engine.add_entity("enemy", enemy);

        engine.game_loop();
    }
}
