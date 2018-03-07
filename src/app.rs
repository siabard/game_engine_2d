//! Main application
extern crate sdl2;

use sdl2::pixels::Color;
use game::*;
use std::env;
use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};

/// App
pub struct App {
    title: &'static str,
    xpos: i32,
    ypos: i32,
    width: u32,
    height: u32,
    fullscreen: bool,
    assets: Vec<&'static str>,
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
        let mut app = App {
            title: title,
            xpos: xpos,
            ypos: ypos,
            width: width,
            height: height,
            fullscreen: fullscreen,
            assets: Vec::new(),
        };

        app
    }

    /// Add Asset
    pub fn add_asset(&mut self, path: &'static str) {
        self.assets.push(path);
    }

    /// run_loop
    pub fn run_loop(&mut self) {
        let context = sdl2::init().expect("Cannot Initialize SDL2");
        let video_subsystem = context.video().unwrap();
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

        let mut event_pump = context.event_pump().unwrap();
        let mut texture_creator = canvas.texture_creator();

        let mut game = Game::new(&texture_creator);

        for a in &self.assets {
            game.add_texture(*a);
        }

        let mut is_running = true;

        while is_running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        is_running = false;
                    }
                    _ => {}
                }
            }

            canvas.clear();

            for texture in &game.textures {
                canvas.copy(texture, None, None).expect("render fail");
            }
            canvas.present();
        }
    }
}
