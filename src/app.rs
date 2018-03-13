//! Main application
extern crate sdl2;
extern crate std;

use sdl2::pixels::Color;
use sprite::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_JPG, INIT_PNG};
use std::collections::HashMap;

const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;

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
        let mut app = App {
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

    /// run_loop
    pub fn run_loop(&mut self) {
        let context = sdl2::init().expect("Cannot Initialize SDL2");
        let video_subsystem = context.video().unwrap();
        let mut timer = context.timer().unwrap();
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
        let texture_creator = canvas.texture_creator();

        let mut sprites = HashMap::new();

        let mut pos: i32 = 0;
        for (id, path) in &self.assets {
            let mut sprite = Sprite::new(&texture_creator, pos, pos);
            sprite.set_texture(path);
            sprites.insert(id, sprite);

            pos = pos + 32;
        }

        let mut is_running = true;

        while is_running {
            let start_tick = timer.ticks();
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

            for (_id, sprite) in &mut sprites {
                sprite.update();
                sprite.render(&mut canvas);
            }

            canvas.present();

            let tick_span = timer.ticks() - start_tick;

            if tick_span < FRAME_DELAY {
                std::thread::sleep(std::time::Duration::from_millis(
                    (FRAME_DELAY - tick_span) as u64,
                ));
            }
        }
    }
}
