//! Components Definition

use ecs::*;

/// Position Component
pub struct PositionComponent {
    /// xpos
    pub xpos: i32,
    /// ypos
    pub ypos: i32,
    /// is_active
    pub is_active: bool,
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
    /// get name
    fn get_name(&self) -> &'static str {
        "position_component"
    }

    /// init
    fn init(&mut self) {
        self.xpos = 0;
        self.ypos = 0;
        self.is_active = true;
    }

    /// draw
    fn draw(&self) {}

    /// is_active
    fn is_active(&self) -> bool {
        true
    }

    /// destroy
    fn destroy(&mut self) {
        self.is_active = false;
    }

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
    active: bool,
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
    /// get_name
    fn get_name(&self) -> &'static str {
        "health_component"
    }
    /// draw
    fn draw(&self) {}

    /// return active
    fn is_active(&self) -> bool {
        self.active
    }

    /// destroy
    fn destroy(&mut self) {
        self.active = false;
    }

    /// update
    fn update(&mut self) {}
}
