//! Main application
extern crate sdl2;
extern crate std;

use sdl2::pixels::Color;
use game::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_JPG, INIT_PNG};

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

        let mut game = Game::new(&texture_creator);
        let _src_r = sdl2::rect::Rect::new(0, 0, 32, 32);
        let mut dest_r = sdl2::rect::Rect::new(0, 0, 64, 64);

        for a in &self.assets {
            game.add_texture(*a);
        }

        let mut is_running = true;
        let mut cnt = 0;

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
            dest_r.set_x(cnt);
            for texture in &game.textures {
                canvas.copy(texture, None, dest_r).expect("render fail");
            }
            canvas.present();

            let tick_span = timer.ticks() - start_tick;

            if tick_span < FRAME_DELAY {
                cnt = cnt + 1;

                std::thread::sleep(std::time::Duration::from_millis(
                    (FRAME_DELAY - tick_span) as u64,
                ));
            }
        }
    }
}
