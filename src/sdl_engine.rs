//! # GameObject
//!
//! SDL 게임내부 메커니즘 담당
//! (render, event 등 처리)

extern crate sdl2;
extern crate std;

use components::*;
use ecs::*;
use map::*;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::path::Path;
use vector_2d::*;

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

/// Sprite Result{}
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
    /// Textures
    pub textures: HashMap<String, sdl2::render::Texture<'a>>,
    /// Map
    pub map: Map<'a>,

    is_running: bool,
    /// Entities
    entities: HashMap<String, Entity>,
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
            textures: HashMap::new(),
            timer: timer,
            is_running: false,
            map: map,
            entities: HashMap::new(),
        }
    }

    /// add sprite
    pub fn add_texture(&mut self, texture_id: &'static str, path: &'static str) {
        let img: &'a Path = Path::new(path);
        let texture = (*self.texture_creator.unwrap()).load_texture(img).unwrap();

        self.textures.insert(texture_id.to_owned(), texture);
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
        self.entities.insert(entity_id.to_owned(), entity);
    }

    /// remove_entity
    pub fn remove_entity(&mut self, entity_id: &'static str) {
        self.entities.remove(&entity_id.to_owned());
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

            let mut positions = HashMap::new();

            // ECS를 이용한 렌더링을 걸어보자!!!
            for (id, entity) in &mut self.entities {
                // TransformComponent 갖는 경우 일정 틱마다 위치를 변경한다.
                match entity.get_component_mut::<TransformComponent>() {
                    Ok(p) => {
                        //p.position += Vector2D { x: 5.0, y: 0.0 };
                        p.update();
                        positions.insert(
                            id.clone(),
                            Vector2D {
                                x: p.position.x,
                                y: p.position.y,
                            },
                        );
                    }
                    Err(_) => {}
                };
            }
            for (id, entity) in &mut self.entities {
                // SpriteComponent를 갖는 경우 해당 위치에 Sprite를 노출한다.
                match entity.get_component_mut::<SpriteComponent>() {
                    Ok(mut c) => {
                        let position = positions.get(&id.clone()).unwrap();
                        c.dest_rect.x = (*position).x as i32;
                        c.dest_rect.y = (*position).y as i32;

                        if c.dest_rect.x > 300 {
                            c.texture_id = "enemy".to_owned();
                        }

                        let texture_id = c.texture_id.clone();

                        let texture = self.textures.get(&texture_id);

                        let t = texture.as_ref().unwrap();
                        self.window
                            .copy(t, c.source_rect, c.dest_rect)
                            .expect("render fail");
                    }
                    Err(_) => {}
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
