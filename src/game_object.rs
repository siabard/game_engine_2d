//! SDL 오브젝트 관리

extern crate sdl2;
extern crate std;

use std::collections::HashMap;
use sprite::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;

/// GameObject
pub struct GameObject<'a> {
    /// window
    pub window: sdl2::render::Canvas<sdl2::video::Window>,
    /// event_pump
    pub event_pump: sdl2::EventPump,
    /// texture_creator
    pub texture_creator: Option<&'a sdl2::render::TextureCreator<sdl2::video::WindowContext>>,
    /// timer
    pub timer: sdl2::TimerSubsystem,
    /// Sprites
    pub sprites: HashMap<&'static str, Sprite<'a>>,
    is_running: bool,
}

impl<'a> GameObject<'a> {
    /// Constructor
    pub fn new(
        window: sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: sdl2::EventPump,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        timer: sdl2::TimerSubsystem,
    ) -> GameObject<'a> {
        GameObject {
            window: window,
            event_pump: event_pump,
            texture_creator: Some(texture_creator),
            sprites: HashMap::new(),
            timer: timer,
            is_running: false,
        }
    }

    /// add sprite
    pub fn add_sprite(&mut self, id: &'static str, path: &'static str, xpos: i32, ypos: i32) {
        let mut sprite = Sprite::new(&self.texture_creator.unwrap(), xpos, ypos);
        sprite.set_texture(path);
        self.sprites.insert(id, sprite);
    }

    /// event manipulation
    pub fn process_event(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_running = false;
                }
                _ => {}
            }
        }
    }

    /// game loop
    pub fn game_loop(&mut self) {
        self.is_running = true;
        while self.is_running {
            let start_tick = self.timer.ticks();

            self.process_event();

            self.window.clear();

            for (_id, sprite) in &mut self.sprites {
                sprite.update();
                sprite.render(&mut self.window);
            }

            self.window.present();

            let tick_span = self.timer.ticks() - start_tick;

            if tick_span < FRAME_DELAY {
                std::thread::sleep(std::time::Duration::from_millis(
                    (FRAME_DELAY - tick_span) as u64,
                ));
            }
        }
    }
}
