//! # Main application
//!
//! 이 모듈은 SDL의 초기화와 ASSET의 설정을 담당한다.
extern crate sdl2;
extern crate std;

use sdl2::pixels::Color;
use sdl2::image::{INIT_JPG, INIT_PNG};
use std::collections::HashMap;
use game_object::*;

/// App
pub struct App {
    title: &'static str,
    xpos: i32,
    ypos: i32,
    width: u32,
    height: u32,
    fullscreen: bool,
    assets: HashMap<&'static str, &'static str>,
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
            assets: HashMap::new(),
        };

        app
    }

    /// Add Asset
    pub fn add_asset(&mut self, id: &'static str, path: &'static str) {
        self.assets.insert(id, path);
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

        let mut gobject: GameObject = GameObject::new(canvas, event_pump, &texture_creator, timer);

        gobject.add_sprite("char", "assets/char.png", 0, 0);
        gobject.add_sprite("enemy", "assets/enemy.png", 50, 50);

        gobject.game_loop();
    }
}
