//use crate::easing::*;
use crate::gui::Framework;
use crate::renderer::*;
use glam::Vec2;
use image::DynamicImage;
use log::error;
use pixels::Error;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod easing;
mod gui;
mod renderer;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    sprite: DynamicImage,
    now: Instant,
    time_passed: f32,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Roguelike Engine")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut renderer = Renderer::new(&window);

    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;
    let mut framework = Framework::new(
        &event_loop,
        window_size.width,
        window_size.height,
        scale_factor,
        &renderer.pixels,
    );

    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                renderer.pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }

            // Reset
            if input.key_pressed(VirtualKeyCode::R)
            {
                world.reset();
            }

            // Update internal state and request a redraw
            world.update(&mut input);
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world
                world.draw(&mut renderer);

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result =
                    renderer
                        .pixels
                        .render_with(|encoder, render_target, context| {
                            // Render the world texture
                            context.scaling_renderer.render(encoder, render_target);

                            // Render egui
                            //framework.render(encoder, render_target, context);

                            Ok(())
                        });

                // Basic error handling
                if render_result
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            sprite: image::open("src/assets/slime_idle_anim_f0.png").unwrap(),
            now: Instant::now(),
            time_passed: 0.0,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, input: &mut WinitInputHelper) {
        let dt = self.now.elapsed().as_secs_f32();
        self.time_passed += dt;
        self.now = Instant::now();
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_frame([0x00,0x00, 0x00, 0x00]);
        renderer.set_offset(Vec2::ZERO);
        renderer.draw_text(
            Vec2 { x: 32.0, y: 32.0 },
            "Hello World!",
            32.0,
            24.0,
            [0xff, 0xff, 0xff, 0xff],
        );
        renderer.set_offset(Vec2::ONE * 100.0 * (self.time_passed * 3.0).sin());

        renderer.draw_sprite(Vec2 { x: 50.0, y: 50.0 }, &self.sprite, 4);
    }
}
