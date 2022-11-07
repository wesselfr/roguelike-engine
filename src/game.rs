use std::cell::Cell;

use crate::easing::*;
use crate::renderer::Renderer;
use crate::sprite::Sprite;
use bitflags::bitflags;
use glam::Vec2;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

const GRID_OFFSET: Vec2 = Vec2 { x: 100.0, y: 100.0 };

const TILE_COLOUR_A: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const TILE_COLOUR_B: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

bitflags! {
    struct CellType: u8
    {
        const PLAYER_MIDDLE = 1 << 2;
        const PLAYER_FRONT = 1 << 3;
        const ENEMY = 1 << 4;
    }
}

pub struct Game {
    player_sprite: Sprite,
    test_tile: Sprite,
    track_tile_normal: Sprite,
    track_tile_flipped: Sprite,
    grid: [u8; GRID_WIDTH * GRID_HEIGHT],
    new_grid: [u8; GRID_WIDTH * GRID_HEIGHT],
    time_passed: f32,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            player_sprite: Sprite::from_image("assets/weapon_sword_1.png", Some(2.0)),
            test_tile: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                4,
                0,
                49,
                22,
                Some(2.0),
            ),
            track_tile_normal: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                0,
                5,
                49,
                22,
                Some(2.0),
            ),
            track_tile_flipped: {
                let mut sprite = Sprite::from_grid(
                    "assets/monochrome-transparent_packed.png",
                    0,
                    5,
                    49,
                    22,
                    Some(2.0),
                );
                sprite.image = sprite.image.rotate90();
                sprite
            },
            grid: [0; GRID_WIDTH * GRID_HEIGHT],
            new_grid: [0; GRID_WIDTH * GRID_HEIGHT],
            time_passed: 0.0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.grid[2] = CellType::PLAYER_FRONT.bits;
        self.grid[8] = CellType::ENEMY.bits;
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub(crate) fn update(&mut self, input: &mut WinitInputHelper, dt: f32) {
        // Todo: Remove this way of initialization.
        if self.time_passed == 0.0 {
            self.reset()
        };
        self.time_passed += dt;

        let mut player_dir_x: i32 = 0;
        let mut player_dir_y: i32 = 0;

        if input.key_pressed(VirtualKeyCode::W) {
            player_dir_y = -1;
        }
        if input.key_pressed(VirtualKeyCode::S) {
            player_dir_y = 1;
        }
        if input.key_pressed(VirtualKeyCode::A) {
            player_dir_x = -1;
        }
        if input.key_pressed(VirtualKeyCode::D) {
            player_dir_x = 1;
        }

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let index = y * GRID_WIDTH + x;

                if self.grid[index] == CellType::PLAYER_FRONT.bits {
                    let new_index = ((y as i32 + player_dir_y) * GRID_WIDTH as i32
                        + x as i32
                        + player_dir_x) as usize;

                    if new_index != index {
                        let pos_x = x as i32;
                        let pos_y = y as i32;
                        if pos_x + player_dir_x >= 0
                            && pos_x + player_dir_x as i32 <= GRID_WIDTH as i32 - 1
                            && pos_y + player_dir_y >= 0
                            && pos_y + player_dir_y as i32 <= GRID_WIDTH as i32 - 1
                        {
                            self.new_grid[index] = CellType::PLAYER_MIDDLE.bits;
                            self.new_grid[new_index] = CellType::PLAYER_FRONT.bits;
                        }
                    } else {
                        self.new_grid[index] = CellType::PLAYER_FRONT.bits;
                    }
                } else if self.grid[index] > 0 && self.grid[index] <= CellType::PLAYER_MIDDLE.bits {
                    if player_dir_x == 0 && player_dir_y == 0 {
                        self.new_grid[index] = self.grid[index];
                    } else {
                        self.new_grid[index] = self.grid[index] - 1;
                    }
                } else if self.grid[index] == CellType::ENEMY.bits {
                    self.new_grid[index] = CellType::ENEMY.bits;
                }
            }
        }

        for i in 0..GRID_WIDTH * GRID_HEIGHT {
            self.grid[i] = self.new_grid[i];
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub(crate) fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_frame([0x00, 0x00, 0x00, 0x00]);
        renderer.set_offset(Vec2::ZERO);

        let text_animated_time = (self.time_passed * 0.45).min(1.0);
        renderer.draw_text(
            Vec2 { x: 32.0, y: 32.0 },
            "Nuclear Train",
            32.0 * ease_out_back(text_animated_time),
            24.0 * ease_out_back(text_animated_time),
            [0x18, 0x7d, 0x0f, 0xff],
        );

        renderer.draw_sprite_color(
            Vec2 { x: 20.0, y: 20.0 },
            &self.test_tile,
            [0x00, 0xff, 0x00, 0xff],
        );

        renderer.draw_sprite(Vec2 { x: 90.0, y: 20.0 }, &self.track_tile_normal);
        renderer.draw_sprite(Vec2 { x: 150.0, y: 20.0 }, &self.track_tile_flipped);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let index = y * GRID_WIDTH + x;

                if index % 2 == 0 {
                    renderer.draw_sprite_color(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        &self.test_tile,
                        TILE_COLOUR_A,
                    );
                } else {
                    renderer.draw_sprite_color(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        &self.test_tile,
                        TILE_COLOUR_B,
                    );
                }

                if self.grid[index] == CellType::PLAYER_FRONT.bits {
                    renderer.draw_square(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        Vec2::ONE * 32.0,
                        [0x00, 0xff, 0x00, 0xff],
                    );
                }

                if self.grid[index] > 0 && self.grid[index] <= CellType::PLAYER_MIDDLE.bits {
                    renderer.draw_char(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        'M',
                        32.0,
                        [0x55, 0x55, 0x55, 0xff],
                    );
                }

                if self.grid[index] == CellType::ENEMY.bits {
                    renderer.draw_char(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0 + 8.0,
                                y: y as f32 * 32.0 + 8.0,
                            },
                        'E',
                        32.0,
                        [0xea, 0x10, 0x26, 0xff],
                    );
                }
            }
        }
    }
}
