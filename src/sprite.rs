//!# Sprite
//!
//! 게임내 Sprite 노출처리

extern crate sdl2;

use std::path::Path;
use sdl2::image::LoadTexture;

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

pub struct Sprite<'a> {
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    texture: Option<sdl2::render::Texture<'a>>,
    xpos: i32,
    ypos: i32,
    src_rect: sdl2::rect::Rect,
    dest_rect: sdl2::rect::Rect,
}

impl<'a> Sprite<'a> {
    /// initializer
    pub fn new(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        xpos: i32,
        ypos: i32,
    ) -> Self {
        Sprite {
            texture_creator: texture_creator,
            texture: None,
            xpos: xpos,
            ypos: ypos,
            src_rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            dest_rect: sdl2::rect::Rect::new(0, 0, 0, 0),
        }
    }

    /// set_xpos
    pub fn set_xpos(&mut self, xpos: i32) {
        self.xpos = xpos;
    }

    /// set_ypos
    pub fn set_ypos(&mut self, ypos: i32) {
        self.ypos = ypos;
    }

    /// set source rectangle
    pub fn set_src_rect(&mut self, src_rect: &sdl2::rect::Rect) {
        self.src_rect.x = src_rect.x;
        self.src_rect.y = src_rect.y;
        self.src_rect.set_width(src_rect.width());
        self.src_rect.set_height(src_rect.height());
    }

    /// set source rectangle
    pub fn set_dest_rect(&mut self, dest_rect: &sdl2::rect::Rect) {
        self.dest_rect.x = dest_rect.x;
        self.dest_rect.y = dest_rect.y;
        self.dest_rect.set_width(dest_rect.width());
        self.dest_rect.set_height(dest_rect.height());
    }

    /// update
    pub fn update(&mut self) {
        self.xpos = self.xpos + 1;
        self.ypos = self.ypos + 1;
        self.src_rect.set_width(32);
        self.src_rect.set_height(32);
        self.src_rect.x = 0;
        self.src_rect.y = 0;

        self.dest_rect.x = self.xpos;
        self.dest_rect.y = self.ypos;
        self.dest_rect.set_width(self.src_rect.width() * 2);
        self.dest_rect.set_height(self.src_rect.height() * 2);
    }

    /// render
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let texture = self.texture.as_ref();
        match texture {
            Some(t) => canvas
                .copy(&t, self.src_rect, self.dest_rect)
                .expect("render fail"),
            None => {}
        }
    }

    /// texture 추가
    pub fn set_texture(&mut self, path: &'static str) {
        let img: &'a Path = Path::new(path);

        let texture_creator = self.texture_creator;
        let texture: sdl2::render::Texture<'a> = (*texture_creator).load_texture(img).unwrap();
        self.texture = Some(texture);
    }
}
