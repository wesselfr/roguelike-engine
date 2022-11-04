use fontdue::{self, Font};
use glam::Vec2;
use image::{DynamicImage, GenericImageView};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub(crate) struct Renderer {
    pub pixels: Pixels,
    width: u32,
    _height: u32,
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
            font: {
                // Read the font data.
                let font = include_bytes!("assets/kenpixel_mini_square.ttf") as &[u8];
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

    pub(crate) fn draw_square(&mut self, pos: Vec2, size: Vec2) {
        for (i, pixel) in self.pixels.get_frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;

            let inside_the_box = x >= pos.x as i16
                && x < pos.x as i16 + size.x as i16
                && y >= pos.y as i16
                && y < pos.y as i16 + size.y as i16;

            if inside_the_box {
                let rgba = [0x5e, 0x48, 0xe8, 0xff];
                pixel.copy_from_slice(&rgba);
            }
        }
    }

    pub(crate) fn draw_sprite(&mut self, pos: Vec2, image: &DynamicImage, scale: u32) {
        let (size_x, size_y) = image.dimensions();
        for (i, pixel) in self.pixels.get_frame_mut().chunks_exact_mut(4).enumerate() {
            let x = i as u32 % self.width - pos.x as u32;
            let y = i as u32 / self.width - pos.y as u32;

            if x > 0 && x < size_x * scale && y > 0 && y < size_y * scale {
                let data = &image.get_pixel(x / scale, y / scale).0;
                if data[3] > 0 {
                    pixel.copy_from_slice(data);
                }
            }
        }
    }

    pub(crate) fn draw_char(&mut self, pos: Vec2, char: char, size: f32, color: [u8; 4]) {
        let (metrics, bitmap) = self.font.rasterize(char, size);

        let size_x = metrics.width as u32;
        let size_y = metrics.height as u32;
        for (i, pixel) in self.pixels.get_frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i as i32 % self.width as i32 - pos.x as i32) as u32;
            let y = (i as i32 / self.width as i32 - pos.y as i32) as u32;

            if x < size_x && y < size_y {
                let data_raw = bitmap[(y * size_x + x) as usize];
                if data_raw == 255 {
                    pixel.copy_from_slice(&color);
                }
            }
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
