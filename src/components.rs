//! Components Definition
extern crate sdl2;

use vector_2d::*;

/// Position Component
pub struct TransformComponent {
    /// entity_id
    pub entity_id: String,
    /// position
    pub position: Vector2D,
}

impl TransformComponent {
    /// get_xpos
    pub fn get_xpos(&self) -> f32 {
        self.position.x
    }

    /// get_ypos
    pub fn get_ypos(&self) -> f32 {
        self.position.y
    }

    /// set xpos
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.position = Vector2D { x: x, y: y };
    }

    /// init
    pub fn init(&mut self) {
        self.position = Vector2D { x: 0.0, y: 0.0 };
    }

    /// draw
    pub fn draw(&self) {}

    /// update
    pub fn update(&mut self) {}
}
/// Health Compoent
pub struct HealthComponent {
    hp: u32,
}

impl HealthComponent {
    /// get_hp
    pub fn get_hp(&self) -> u32 {
        self.hp
    }

    /// set hp
    pub fn set_hp(&mut self, hp: u32) {
        self.hp = hp;
    }

    /// init
    pub fn init(&mut self) {
        self.hp = 0;
    }
    /// draw
    pub fn draw(&self) {}

    /// update
    pub fn update(&mut self) {}
}

/// Sprite Component
pub struct SpriteComponent {
    /// sprite_id
    pub entity_id: String,
    /// texture_id
    pub texture_id: String,
    /// Source_rect
    pub source_rect: sdl2::rect::Rect,
    /// target_rect
    pub dest_rect: sdl2::rect::Rect,
}

impl SpriteComponent {
    /// new
    pub fn new(sprite_id: &'static str, texture_id: &'static str) -> Self {
        SpriteComponent {
            entity_id: sprite_id.to_owned(),
            texture_id: texture_id.to_owned(),
            source_rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            dest_rect: sdl2::rect::Rect::new(0, 0, 0, 0),
        }
    }

    /// init
    pub fn init(&mut self) {}

    /// draw
    pub fn draw(&self) {}

    /// update
    pub fn update(&mut self) {}
}
