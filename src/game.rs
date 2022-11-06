use bitflags::bitflags;
use winit::event::VirtualKeyCode;
use crate::renderer::Renderer;
use crate::sprite::Sprite;
use glam::Vec2;
use winit_input_helper::WinitInputHelper;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 5;

const GRID_OFFSET: Vec2 = Vec2 { x: 100.0, y: 100.0 };

const TILE_COLOUR_A: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const TILE_COLOUR_B: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

bitflags! {
    struct CellType: u8
    {
        const PLAYER = 1 << 0;
        const ENEMY = 1 << 1;
    }
}

pub struct Game {
    player_sprite: Sprite,
    grid: [u8; GRID_WIDTH * GRID_HEIGHT],
    new_grid: [u8; GRID_WIDTH * GRID_HEIGHT],
    time_passed: f32,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            player_sprite: Sprite::from_image("assets/weapon_sword_1.png", Some(2.0)),
            grid: [0; 25],
            new_grid: [0; 25],
            time_passed: 0.0,
        }
    }

    pub(crate) fn reset(&mut self)
    {
        self.grid[2] = CellType::PLAYER.bits;
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

                if self.grid[index] == CellType::PLAYER.bits {
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
                            self.new_grid[index] = 0;
                            self.new_grid[new_index] = CellType::PLAYER.bits;
                        }
                    } else {
                        self.new_grid[index] = CellType::PLAYER.bits;
                    }
                }
                else if self.grid[index] == CellType::ENEMY.bits
                {
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
        renderer.clear_frame([0x00,0x00, 0x00, 0x00]);
        renderer.set_offset(Vec2::ZERO);
        renderer.draw_text(
            Vec2 { x: 32.0, y: 32.0 },
            "Hello World!",
            32.0,
            24.0,
            [0xff, 0xff, 0xff, 0xff],
        );
        renderer.set_offset(Vec2::ONE * 5.0 * (self.time_passed * 3.0).sin());

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let index = y * GRID_WIDTH + x;

                if index % 2 == 0 {
                    renderer.draw_square(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        Vec2::ONE * 32.0,
                        TILE_COLOUR_A,
                    );
                } else {
                    renderer.draw_square(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        Vec2::ONE * 32.0,
                        TILE_COLOUR_B,
                    );
                }

                if self.grid[index] == CellType::PLAYER.bits {
                    renderer.draw_sprite(
                        GRID_OFFSET
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        &self.player_sprite
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
