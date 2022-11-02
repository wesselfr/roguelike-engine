use glam::Vec2;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::window::Window;
use winit::window::WindowBuilder;

pub(crate) struct Renderer {
    pub pixels: Pixels,
    width: u32,
    height: u32,
}

impl Renderer {
    pub(crate) fn new(window: &Window) -> Self {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(window_size.width, window_size.height, surface_texture)
            .expect("Error while creating buffer");

        Self { pixels, width: window_size.width as u32, height: window_size.height as u32 }
    }

    pub(crate) fn clear_frame(&mut self, color: [u8; 4]) {
        for (i, pixel) in self.pixels.get_frame_mut().chunks_exact_mut(4).enumerate() {
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
    
            if inside_the_box
            {
                let rgba = [0x5e, 0x48, 0xe8, 0xff];
                pixel.copy_from_slice(&rgba);
            }
        }
    }
}
