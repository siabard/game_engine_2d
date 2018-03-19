//! Components Definition
extern crate sdl2;

use ecs::*;
use std::path::Path;
use sdl2::image::LoadTexture;
use sprite::*;

/// Position Component
pub struct PositionComponent {
    /// xpos
    pub xpos: i32,
    /// ypos
    pub ypos: i32,
}

impl PositionComponent {
    /// get_xpos
    pub fn get_xpos(&self) -> i32 {
        self.xpos
    }

    /// get_ypos
    pub fn get_ypos(&self) -> i32 {
        self.ypos
    }

    /// set xpos
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.xpos = x;
        self.ypos = y;
    }
}

impl Component for PositionComponent {
    /// init
    fn init(&mut self) {
        self.xpos = 0;
        self.ypos = 0;
    }

    /// draw
    fn draw(&self) {}

    /// update
    fn update(&mut self) {
        self.xpos += 1;
        self.ypos += 1;
        println!("xpos = {}, ypos = {}", self.xpos, self.ypos);
    }
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
}
impl Component for HealthComponent {
    /// init
    fn init(&mut self) {
        self.hp = 0;
    }
    /// draw
    fn draw(&self) {}

    /// update
    fn update(&mut self) {}
}

/// Sprite Component
pub struct SpriteComponent {
    /// sprite_id
    pub sprite_id: String,
}

impl SpriteComponent {
    /// new
    pub fn new(sprite_id: &'static str) -> Self {
        SpriteComponent {
            sprite_id: sprite_id.to_owned(),
        }
    }
}

impl Component for SpriteComponent {
    /// init
    fn init(&mut self) {}

    /// draw
    fn draw(&self) {}

    /// update
    fn update(&mut self) {}
}
