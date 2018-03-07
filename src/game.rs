//! Game Engine structure
extern crate sdl2;

use std::path::Path;
use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};

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

pub struct Game<'a> {
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    /// textures
    pub textures: Vec<sdl2::render::Texture<'a>>,
}

impl<'a> Game<'a> {
    /// initializer
    pub fn new(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Self {
        Game {
            texture_creator: texture_creator,
            textures: Vec::new(),
        }
    }

    /// texture 추가
    pub fn add_texture(&mut self, path: &'static str) {
        let img: &'a Path = Path::new(path);

        let texture_creator = self.texture_creator;
        let texture: sdl2::render::Texture<'a> = (*texture_creator).load_texture(img).unwrap();
        self.textures.push(texture);
    }
}
