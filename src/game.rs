use crate::renderer::Renderer;
use crate::sprite::Sprite;
use glam::Vec2;
use winit_input_helper::WinitInputHelper;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

pub struct Game {
    slime: Sprite,
    time_passed: f32,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            slime: Sprite::from_image_animated("assets/slime_idle_spritesheet.png", 6, None),
            time_passed: 0.0,
        }
    }

    pub(crate) fn update(&mut self, input: &mut WinitInputHelper, dt: f32) {
        self.time_passed += dt;
    }

    pub(crate) fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_text(
            Vec2 { x: 32.0, y: 32.0 },
            "Hello World!",
            32.0,
            24.0,
            [0xff, 0xff, 0xff, 0xff],
        );

        renderer.draw_sprite_animated(
            Vec2 { x: 50.0, y: 50.0 },
            &self.slime,
            ((self.time_passed * 8.0) % 5.0).round() as u32,
        );

        renderer.draw_sprite(Vec2 { x: 100.0, y: 150.0 }, &Sprite::from_image("assets/goblin_idle_anim_f0.png", Some(8.0)))
    }
}
