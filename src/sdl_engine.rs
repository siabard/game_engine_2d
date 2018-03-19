//! # GameObject
//!
//! SDL 게임내부 메커니즘 담당
//! (render, event 등 처리)

extern crate sdl2;
extern crate std;

use std::collections::HashMap;
use sprite::*;
use map::*;
use ecs::*;
use components::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;
/// NotFound
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NoSprite {
    /// Not Exist
    NotExist,
    /// Cannot Found
    CannotFound,
}

/// Sprite Result
pub type ResultSprite<T> = Result<T, NoSprite>;

/// SdlEngine
pub struct SdlEngine<'a> {
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
    /// Map
    pub map: Map<'a>,

    is_running: bool,
    /// Entities
    entities: HashMap<&'static str, Entity>,
}

impl<'a> SdlEngine<'a> {
    /// Constructor
    pub fn new(
        window: sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: sdl2::EventPump,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        timer: sdl2::TimerSubsystem,
    ) -> SdlEngine<'a> {
        let map = Map::new(&texture_creator);

        SdlEngine {
            window: window,
            event_pump: event_pump,
            texture_creator: Some(texture_creator),
            sprites: HashMap::new(),
            timer: timer,
            is_running: false,
            map: map,
            entities: HashMap::new(),
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

    /// insert entity
    pub fn add_entity(&mut self, entity_id: &'static str, entity: Entity) {
        self.entities.insert(entity_id, entity);
    }

    /// remove_entity
    pub fn remove_entity(&mut self, entity_id: &'static str) {
        self.entities.remove(entity_id);
    }

    /// init_sprite
    ///
    /// SpriteComponent 를 가지고 있는 Entity가 있으면 대응하는 Sprite를 생성한다.
    pub fn init_sprite(&mut self) {
        for (id, entity) in &self.entities {
            match entity.get_component::<SpriteComponent>() {
                Ok(c) => {
                    let mut sprite = Sprite::new(&self.texture_creator.unwrap(), 0, 0);
                    sprite.set_texture(c.sprite_path);
                    self.sprites.insert(id, sprite);
                }
                Err(_) => {}
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

            // Rendering Screen
            self.map.draw_map(&mut self.window);

            // ECS를 이용한 렌더랑을 걸어보자!!!
            for (id, entity) in &mut self.entities {
                // PositionComponent를 갖는 경우 일정 틱마다 위치를 변경한다.
                let pos = match entity.get_component_mut::<PositionComponent>() {
                    Ok(mut p) => {
                        p.update();
                        p
                    }
                    Err(_) => &PositionComponent { xpos: 0, ypos: 0 },
                };

                // 등록된 Sprite는 해당 ID에 따라서 지정 위치에 노출한다.
                match self.sprites.get_mut(id.to_owned()) {
                    Some(s) => {
                        s.set_xpos(pos.xpos);
                        s.set_ypos(pos.ypos);
                        s.update();
                        s.render(&mut self.window);
                    }
                    _ => {}
                }
            }
            self.window.present();

            // FPS
            let tick_span = self.timer.ticks() - start_tick;

            if tick_span < FRAME_DELAY {
                std::thread::sleep(std::time::Duration::from_millis(
                    (FRAME_DELAY - tick_span) as u64,
                ));
            }
        }
    }
}
