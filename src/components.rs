//! Components Definition
extern crate sdl2;

/// Position Component
pub struct PositionComponent {
    /// entity_id
    pub entity_id: String,
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

    /// init
    pub fn init(&mut self) {
        self.xpos = 0;
        self.ypos = 0;
    }

    /// draw
    pub fn draw(&self) {}

    /// update
    pub fn update(&mut self) {
        self.xpos += 1;
        self.ypos += 1;
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
