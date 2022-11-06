use crate::sprite::Sprite;
use fontdue::{self, Font};
use glam::Vec2;
use image::{DynamicImage, GenericImageView};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub(crate) struct Renderer {
    pub pixels: Pixels,
    width: u32,
    _height: u32,
    offset: Vec2,
    font: Font,
}

impl Renderer {
    pub(crate) fn new(window: &Window) -> Self {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(window_size.width, window_size.height, surface_texture)
            .expect("Error while creating buffer");

        Self {
            pixels,
            width: window_size.width as u32,
            _height: window_size.height as u32,
            offset: Vec2::ZERO,
            font: {
                // Read the font data.
                let font = include_bytes!("../assets/kenpixel_mini_square.ttf") as &[u8];
                // Parse it into the font type.
                fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap()
            },
        }
    }

    pub(crate) fn clear_frame(&mut self, color: [u8; 4]) {
        for pixel in self.pixels.get_frame_mut().chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }

    pub(crate) fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }

    pub(crate) fn draw_square(&mut self, pos: Vec2, size: Vec2, color: [u8; 4]) {
        let pos = pos + self.offset;
        for (i, pixel) in self.pixels.get_frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;

            let inside_the_box = x >= pos.x as i16
                && x < pos.x as i16 + size.x as i16
                && y >= pos.y as i16
                && y < pos.y as i16 + size.y as i16;

            if inside_the_box {
                pixel.copy_from_slice(&color);
            }
        }
    }

    pub(crate) fn draw_sprite(&mut self, pos: Vec2, sprite: &Sprite) {
        let pos = pos + self.offset;

        let mut s = 0;
        for y in 0..sprite.height as usize * sprite.scale as usize {
            let i = pos.x as usize * 4
                + pos.y as usize * self.width as usize * 4
                + y * self.width as usize * 4;

            for (sprite_index, chunk) in self.pixels.get_frame_mut()
                [i..i + sprite.width as usize * 4 * sprite.scale as usize]
                .chunks_mut(4)
                .enumerate()
            {
                let x = sprite_index as u32 / sprite.scale as u32;
                let data = &sprite.image.get_pixel(x, y as u32 / sprite.scale as u32).0;

                if data[3] > 0 {
                    for (col, pixel) in chunk.iter_mut().enumerate() {
                        *pixel = data[col];
                    }
                }
            }

            s += sprite.width * 4;
        }
    }

    pub(crate) fn draw_sprite_animated(&mut self, pos: Vec2, sprite: &Sprite, frame: u32) {
        let size_x = sprite.width as u32;
        let size_y = sprite.height as u32;
        let pos = pos + self.offset;

        let size_x = size_x / sprite.frame_num;

        let mut s = 0;
        for y in 0..size_y as usize * sprite.scale as usize {
            let i = pos.x as usize * 4
                + pos.y as usize * self.width as usize * 4
                + y * self.width as usize * 4;

            for (sprite_index, chunk) in self.pixels.get_frame_mut()
                [i..i + size_x as usize * 4 * sprite.scale as usize]
                .chunks_mut(4)
                .enumerate()
            {
                let frame_offset = size_x * frame;
                let x = frame_offset + sprite_index as u32 / sprite.scale as u32;
                let data = &sprite.image.get_pixel(x, y as u32 / sprite.scale as u32).0;

                if data[3] > 0 {
                    for (col, pixel) in chunk.iter_mut().enumerate() {
                        *pixel = data[col];
                    }
                }
            }

            s += size_x * 4;
        }
    }

    pub(crate) fn draw_char(&mut self, pos: Vec2, char: char, size: f32, color: [u8; 4]) {
        let (metrics, bitmap) = self.font.rasterize(char, size);
        let pos = pos + self.offset;

        let mut s = 0;
        for y in 0..metrics.height {
            let i = pos.x as usize * 4
                + pos.y as usize * self.width as usize * 4
                + y * self.width as usize * 4;

            for (bitmap_index, chunk) in self.pixels.get_frame_mut()[i..i + metrics.width * 4]
                .chunks_mut(4)
                .enumerate()
            {
                let bitmap_index = bitmap_index + y * metrics.width;
                if bitmap[bitmap_index] > 0 {
                    for (col, pixel) in chunk.iter_mut().enumerate() {
                        *pixel = color[col];
                    }
                }
            }

            s += metrics.width * 4;
        }
    }

    pub(crate) fn draw_text(
        &mut self,
        pos: Vec2,
        text: &str,
        size: f32,
        spacing: f32,
        color: [u8; 4],
    ) {
        for (i, char) in text.chars().enumerate() {
            self.draw_char(
                Vec2 {
                    x: pos.x + spacing * i as f32,
                    y: pos.y,
                },
                char,
                size,
                color,
            );
        }
    }
}
