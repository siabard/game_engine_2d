//! Map
extern crate sdl2;
extern crate std;

use std::path::Path;
use sdl2::image::LoadTexture;

/// Map Structure
pub struct Map<'a> {
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    dirt: Option<sdl2::render::Texture<'a>>,
    grass: Option<sdl2::render::Texture<'a>>,
    water: Option<sdl2::render::Texture<'a>>,
    src_rect: sdl2::rect::Rect,
    dest_rect: sdl2::rect::Rect,
    map: [[i32; 25]; 20],
}

impl<'a> Map<'a> {
    /// 생성자
    pub fn new(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Self {
        let dirt: sdl2::render::Texture<'a> = (*texture_creator)
            .load_texture(Path::new("assets/dirt.png"))
            .unwrap();
        let grass: sdl2::render::Texture<'a> = (*texture_creator)
            .load_texture(Path::new("assets/grass.png"))
            .unwrap();
        let water: sdl2::render::Texture<'a> = (*texture_creator)
            .load_texture(Path::new("assets/water.png"))
            .unwrap();

        let lvl1: [[i32; 25]; 20] = [
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ],
        ];
        Map {
            texture_creator: texture_creator,
            dirt: Some(dirt),
            grass: Some(grass),
            water: Some(water),
            src_rect: sdl2::rect::Rect::new(0, 0, 32, 32),
            dest_rect: sdl2::rect::Rect::new(0, 0, 32, 32),
            map: lvl1,
        }
    }
    /// Map 로딩
    pub fn load_map(&mut self, map: [[i32; 25]; 20]) {
        for y in 0..20 {
            for x in 0..25 {
                self.map[y][x] = map[y][x];
            }
        }
    }

    /// Map 그리기
    pub fn draw_map(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for y in 0..20 {
            for x in 0..25 {
                let texture = match self.map[y][x] {
                    0 => self.dirt.as_ref(),
                    1 => self.grass.as_ref(),
                    2 => self.water.as_ref(),
                    _ => None,
                };

                self.dest_rect.x = x as i32 * 32;
                self.dest_rect.y = y as i32 * 32;

                match texture {
                    Some(t) => canvas
                        .copy(&t, self.src_rect, self.dest_rect)
                        .expect("render fail"),
                    None => {}
                }
            }
        }
    }
}
