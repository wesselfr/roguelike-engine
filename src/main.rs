use crate::easing::*;
use crate::gui::Framework;
use crate::renderer::*;
use core::time;
use glam::Vec2;
use image::{flat, DynamicImage};
use log::error;
use pixels::Error;
use std::time::{Duration, Instant};
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
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    player: Entity,
    slime: Entity,
    now: Instant,
    time_passed: f32,
    death_time: f32,

    slime_killed: bool,
    player_killed: bool,
}

struct Entity {
    pos: Vec2,
    vel: Vec2,
    sprite: DynamicImage,
}

impl Entity {
    fn new(pos: Vec2, vel: Vec2, sprite: DynamicImage) -> Self {
        Self { pos, vel, sprite }
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_sprite(self.pos, &self.sprite, 4);
    }
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

// 1D Check
fn collision_check(a: &Entity, b: &Entity) -> bool {
    if a.pos.x + BOX_SIZE as f32 >= b.pos.x && a.pos.x <= b.pos.x + BOX_SIZE as f32
    // && a.pos.y >= b.pos.y
    // && a.pos.y <= b.pos.y + BOX_SIZE as f32
    {
        true
    } else {
        false
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            // Player entity
            player: Entity::new(
                Vec2 {
                    x: 24.0,
                    y: HEIGHT as f32 / 2.0,
                },
                Vec2::ZERO,
                image::open("src/assets/weapon_sword_1.png").unwrap(),
            ),
            // Slime entity
            slime: Entity::new(
                Vec2 {
                    x: WIDTH as f32,
                    y: HEIGHT as f32 / 2.0,
                },
                Vec2 { x: -1.0, y: 0.0 },
                image::open("src/assets/slime_idle_anim_f0.png").unwrap(),
            ),
            now: Instant::now(),
            time_passed: 0.0,
            death_time: 0.0,

            slime_killed: false,
            player_killed: false,
        }
    }

    fn reset(&mut self) {
        self.death_time = 0.0;
        self.player_killed = false;
        self.slime_killed = false;
        self.player.pos = Vec2 {
            x: 24.0,
            y: HEIGHT as f32 / 2.0,
        };
        self.slime.pos = Vec2 {
            x: WIDTH as f32,
            y: HEIGHT as f32 / 2.0,
        };
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, input: &mut WinitInputHelper) {
        let dt = self.now.elapsed().as_secs_f32();
        self.time_passed += dt;
        self.now = Instant::now();

        // Friction
        self.player.vel *= 0.8;

        // Player Input
        if input.key_held(VirtualKeyCode::A) {
            self.player.vel.x += -100.0;
        }
        if input.key_held(VirtualKeyCode::D) {
            self.player.vel.x += 100.0;
        }

        // Bound check
        if self.player.pos.x <= 0.0 || self.player.pos.x as u32 + BOX_SIZE as u32 > WIDTH {
            self.player.vel.x *= -1.0;
        }
        if self.player.pos.y <= 0.0 || self.player.pos.y as u32 + BOX_SIZE as u32 > HEIGHT {
            self.player.vel.y *= -1.0;
        }

        self.slime.vel.x = ((self.time_passed * 2.5).sin() * 80.0) - 40.0;
        self.slime.vel.y = (self.time_passed * 1.5).cos() * 20.0;

        if collision_check(&self.player, &self.slime) && !self.slime_killed && !self.player_killed {
            println!("COLLISION");

            if self.slime.vel.x < 0.0 {
                self.player_killed = true;
                self.death_time = self.time_passed;
            } else {
                self.slime_killed = true;
            }
        }

        if self.player_killed && self.time_passed - self.death_time > 3.0 {
            self.reset();
        }

        self.player.update(dt);
        self.slime.update(dt);
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_frame([0x48, 0xb2, 0xe8, 0xff]);

        if !self.player_killed {
            self.player.draw(renderer);
        }

        if !self.slime_killed {
            self.slime.draw(renderer);
        }
        // Hide enemy
        // if self.time_passed as u32 % 4 == 0{
        //     self.slime.draw(renderer);
        // }
    }
}
