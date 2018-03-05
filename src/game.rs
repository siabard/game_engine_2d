//! Game Engine structure
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Main Structure
///
/// xpos, ypos is integer value, width, height is always positive
///
/// ```rust,no_run
/// let mut game = Game::new("SDL 테스트", 100, 100, 800, 600, false);
///
/// while game.running() {
///     game.handle_event();
///     game.render();
/// }
/// ```
pub struct Game {
    is_running: bool,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
}

impl Game {
    /// initializer
    pub fn new(
        title: &'static str,
        xpos: i32,
        ypos: i32,
        width: u32,
        height: u32,
        fullscreen: bool,
    ) -> Game {
        let context = sdl2::init().expect("Cannot Initialize SDL2");
        let video_subsystem = context.video().unwrap();

        let window = if fullscreen {
            video_subsystem
                .window(title, width, height)
                .position(xpos, ypos)
                .fullscreen()
                .build()
                .unwrap()
        } else {
            video_subsystem
                .window(title, width, height)
                .position(xpos, ypos)
                .build()
                .unwrap()
        };

        let canvas = window.into_canvas().accelerated().build().unwrap();
        let event_pump = context.event_pump().unwrap();
        Game {
            canvas: canvas,
            is_running: true,
            event_pump: event_pump,
        }
    }

    /// update Game objects
    ///
    /// Game logic is located here.
    pub fn update(&mut self) {}

    /// Render Screen
    pub fn render(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    /// Clear all object
    pub fn clean(&self) {}

    /// Event Hadler
    pub fn handle_event(&mut self) {
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

    /// predicate about game is running
    pub fn running(&self) -> bool {
        self.is_running
    }
}
