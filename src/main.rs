use crate::gui::Framework;
use crate::renderer::*;
use glam::Vec2;
use log::error;
use pixels::Error;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;
mod renderer;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_pos: Vec2,
    box_vel: Vec2,
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

            if input.key_pressed(VirtualKeyCode::R) {
                world.box_vel.x = 0.0;
                world.box_vel.y = 0.0;
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
                            framework.render(encoder, render_target, context);

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
            box_pos: Vec2 { x: 24.0, y: 16.0 },
            box_vel: Vec2 { x: 1.0, y: 1.0 },
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, input: &mut WinitInputHelper) {
        // Friction
        self.box_vel *= 0.8;

        // Player Input
        if input.key_held(VirtualKeyCode::W) {
            self.box_vel.y += -1.0;
        }
        if input.key_held(VirtualKeyCode::S) {
            self.box_vel.y += 1.0;
        }
        if input.key_held(VirtualKeyCode::A) {
            self.box_vel.x += -1.0;
        }
        if input.key_held(VirtualKeyCode::D) {
            self.box_vel.x += 1.0;
        }

        // Bound check
        if self.box_pos.x <= 0.0 || self.box_pos.x + BOX_SIZE as f32 > WIDTH as f32 {
            self.box_vel.x *= -1.0;
        }
        if self.box_pos.y <= 0.0 || self.box_pos.y + BOX_SIZE as f32 > HEIGHT as f32 {
            self.box_vel.y *= -1.0;
        }

        self.box_pos += self.box_vel;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_frame([0x48, 0xb2, 0xe8, 0xff]);

        renderer.draw_square(
            self.box_pos,
            Vec2 {
                x: BOX_SIZE as f32,
                y: BOX_SIZE as f32,
            },
        );
    }
}
