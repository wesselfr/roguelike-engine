use crate::renderer::Renderer;
use crate::sprite::Sprite;
use crate::{easing::*, sprite};
use bitflags::bitflags;
use glam::Vec2;
use rand::rngs::ThreadRng;
use rand::Rng;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

const TILESHEET: &str = "assets/monochrome-transparent_packed.png";
const TILESHEET_MAX_X: u32 = 49;
const TILESHEET_MAX_Y: u32 = 22;

const CELL_SIZE: f32 = 16.0 * 2.0;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = ((HEIGHT / CELL_SIZE as u32) - 1) as usize;

const CELL_TRACK_COLOR: [u8; 4] = [0x4c, 0x4c, 0x4c, 0xff];
const TILE_COLOUR_A: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const TILE_COLOUR_B: [u8; 4] = [0x00, 0x00, 0xff, 0xff]; // generates a float between 0 and 1

bitflags! {
    struct Directions: u8
    {
        const UP = 1 << 0;
        const RIGHT = 1 << 1;
        const DOWN = 1 << 2;
        const LEFT = 1 << 3;
    }
}

bitflags! {
    struct CellType: u16
    {
        const EMPTY = 0 << 0;
        const PLAYER_MIDDLE = 1 << 2;
        const PLAYER_FRONT = 1 << 3;
        const ENEMY = 1 << 4;
        const TRACK = 1 << 5;
        const TRACK_DIR_MAX = 1 << 9;
    }
}

struct Entity {
    pos: Vec2,
    vel: Vec2,
    sprite: Sprite,
}

impl Entity {
    fn new(pos: Vec2, sprite: Sprite) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            sprite,
        }
    }
}

pub struct Game {
    rng: ThreadRng,
    initialized: bool,
    grid_offset: Vec2,
    player_sprite_right: Sprite,
    player_sprite_up: Sprite,
    player_sprite_down: Sprite,
    test_tile: Sprite,
    track_tile_normal: Sprite,
    track_tile_flipped: Sprite,
    track_tile_corner_t_r: Sprite,
    track_tile_corner_t_l: Sprite,
    track_tile_corner_b_r: Sprite,
    track_tile_corner_b_l: Sprite,
    track_tile_t_junction_normal: Sprite,
    track_tile_t_junction_flipped: Sprite,
    track_tile_crossing: Sprite,
    grass_sprite: Sprite,
    grid: [u16; GRID_WIDTH * GRID_HEIGHT],
    new_grid: [u16; GRID_WIDTH * GRID_HEIGHT],
    time_passed: f32,
    move_timer: f32,
    move_interval: f32,

    last_dir_x: i32,
    last_dir_y: i32,
    train_test: Vec<Entity>,
}

fn get_index(x: u32, y: u32) -> u32 {
    y * GRID_WIDTH as u32 + x
}

fn index_in_grid(index: u32) -> bool {
    index > 0 && index < (GRID_WIDTH * GRID_HEIGHT) as u32
}

fn get_position(index: u32) -> (u32, u32) {
    (index % GRID_WIDTH as u32, index / GRID_WIDTH as u32)
}

fn get_grid_cell_pos(x: u32, y: u32, grid: &[u16; GRID_WIDTH * GRID_HEIGHT]) -> Option<u16> {
    let index = get_index(x, y);
    if index_in_grid(index) {
        Some(grid[index as usize])
    } else {
        None
    }
}

fn gen_track(
    x: u32,
    y: u32,
    grid: &mut [u16; GRID_WIDTH * GRID_HEIGHT],
    rng: &mut ThreadRng,
    make_corner: bool,
) {
    let lenght = rng.gen_range(3..12);

    let mut dir = 1;
    if make_corner {
        let down = rng.gen_bool(0.5);
        dir = if down { 1 } else { -1 };
    }

    let mut last_index = 0;
    let mut should_gen = true;
    for i in 0..lenght {
        if !should_gen {
            return;
        }
        if make_corner {
            let y_pos = y as i32 + dir * i;

            if y_pos <= 0 || y_pos >= (GRID_HEIGHT as i32) - 1 {
                println!("TRIGGERED");
                should_gen = false;
                break;
            }

            let index = get_index(x, y_pos as u32) as usize;
            if index_in_grid(index as u32) {
                grid[index] = grid[index] | CellType::TRACK.bits;
                last_index = index;
            } else {
                should_gen = false;
                break;
            }
        } else {
            let index = get_index(x + i as u32, y) as usize;
            if index_in_grid(index as u32) {
                grid[index] = grid[index] | CellType::TRACK.bits;
                last_index = index;
            } else {
                should_gen = false;
                break;
            }
        }
    }

    let (new_x, new_y) = get_position(last_index as u32);

    if new_x < 45 && new_y > 0 && new_y < GRID_HEIGHT as u32 - 1 {
        return gen_track(new_x, new_y, grid, rng, !make_corner);
    }
}

fn evaluate_track_dir(index: u32, grid: &mut [u16; GRID_WIDTH * GRID_HEIGHT]) {
    // Only evaluate for track
    if (grid[index as usize] & CellType::TRACK.bits) > 0 {
        let (pos_x, pos_y) = get_position(index);
        let mut dir = 0;

        let up = get_grid_cell_pos(pos_x, pos_y - 1, &grid);
        let right = get_grid_cell_pos(pos_x + 1, pos_y, &grid);
        let down = get_grid_cell_pos(pos_x, pos_y + 1, &grid);
        let left = get_grid_cell_pos(pos_x - 1, pos_y, &grid);

        if up.is_some() {
            dir = if up.unwrap() & CellType::TRACK.bits > 0 {
                dir | Directions::UP.bits
            } else {
                dir
            };
        }
        if right.is_some() {
            dir = if right.unwrap() & CellType::TRACK.bits > 0 {
                dir | Directions::RIGHT.bits
            } else {
                dir
            };
        }
        if down.is_some() {
            dir = if down.unwrap() & CellType::TRACK.bits > 0 {
                dir | Directions::DOWN.bits
            } else {
                dir
            };
        }
        if left.is_some() {
            dir = if left.unwrap() & CellType::TRACK.bits > 0 {
                dir | Directions::LEFT.bits
            } else {
                dir
            };
        }

        grid[index as usize] = grid[index as usize] | (dir as u16) << 6;
    }
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            initialized: false,
            grid_offset: Vec2 { x: 100.0, y: 100.0 },
            player_sprite_right: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                11,
                20,
                49,
                22,
                Some(2.0),
            ),
            player_sprite_up: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                10,
                20,
                49,
                22,
                Some(2.0),
            ),
            player_sprite_down: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                9,
                20,
                49,
                22,
                Some(2.0),
            ),
            test_tile: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                38,
                12,
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
            track_tile_corner_t_r: {
                let mut sprite = Sprite::from_grid(
                    "assets/monochrome-transparent_packed.png",
                    1,
                    5,
                    49,
                    22,
                    Some(2.0),
                );
                sprite.flip_vertical();
                sprite
            },
            track_tile_corner_t_l: {
                let mut sprite = Sprite::from_grid(
                    "assets/monochrome-transparent_packed.png",
                    1,
                    5,
                    49,
                    22,
                    Some(2.0),
                );
                sprite.image = sprite.image.rotate180();
                sprite
            },
            track_tile_corner_b_r: {
                let mut sprite = Sprite::from_grid(
                    "assets/monochrome-transparent_packed.png",
                    1,
                    5,
                    49,
                    22,
                    Some(2.0),
                );
                sprite
            },
            track_tile_corner_b_l: {
                let mut sprite = Sprite::from_grid(
                    "assets/monochrome-transparent_packed.png",
                    1,
                    5,
                    49,
                    22,
                    Some(2.0),
                );
                sprite.flip_horizontal();
                sprite
            },
            track_tile_t_junction_normal: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                2,
                5,
                49,
                22,
                Some(2.0),
            ),
            track_tile_t_junction_flipped: {
                let mut sprite = Sprite::from_grid(
                    "assets/monochrome-transparent_packed.png",
                    2,
                    5,
                    49,
                    22,
                    Some(2.0),
                );
                sprite.image = sprite.image.rotate90();
                sprite
            },
            track_tile_crossing: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                3,
                5,
                49,
                22,
                Some(2.0),
            ),
            grass_sprite: Sprite::from_grid(
                "assets/monochrome-transparent_packed.png",
                5,
                0,
                49,
                22,
                Some(2.0),
            ),
            grid: [0; GRID_WIDTH * GRID_HEIGHT],
            new_grid: [0; GRID_WIDTH * GRID_HEIGHT],
            time_passed: 0.0,
            move_timer: 0.0,
            move_interval: 2.0,
            last_dir_x: 1,
            last_dir_y: 0,
            train_test: {
                vec![
                    Entity::new(
                        Vec2 { x: 200.0, y: 200.0 },
                        Sprite::from_grid(
                            TILESHEET,
                            9,
                            21,
                            TILESHEET_MAX_X,
                            TILESHEET_MAX_Y,
                            Some(2.0),
                        ),
                    ),
                    Entity::new(
                        Vec2 { x: 200.0, y: 232.0 },
                        Sprite::from_grid(
                            TILESHEET,
                            9,
                            21,
                            TILESHEET_MAX_X,
                            TILESHEET_MAX_Y,
                            Some(2.0),
                        ),
                    ),
                    Entity::new(
                        Vec2 { x: 200.0, y: 264.0 },
                        Sprite::from_grid(
                            TILESHEET,
                            9,
                            21,
                            TILESHEET_MAX_X,
                            TILESHEET_MAX_Y,
                            Some(2.0),
                        ),
                    ),
                ]
            },
        }
    }

    pub(crate) fn reset(&mut self) {
        //let (px, py) = get_position(20);

        self.grid[get_index(0, 5) as usize] = CellType::PLAYER_FRONT.bits;

        gen_track(0, 5, &mut self.grid, &mut self.rng, false);

        // for i in 20..30 {
        //     self.grid[i] = self.grid[i] | CellType::TRACK.bits;
        // }

        // self.grid[24] = self.grid[24] | CellType::TRACK.bits;
        // self.grid[34] = self.grid[34] | CellType::TRACK.bits;
        // self.grid[44] = self.grid[44] | CellType::TRACK.bits;

        // self.grid[27] = self.grid[27] | CellType::TRACK.bits;
        // self.grid[37] = self.grid[37] | CellType::TRACK.bits;
        // self.grid[47] = self.grid[47] | CellType::TRACK.bits;
        // for i in 45..47 {
        //     self.grid[i] = self.grid[i] | CellType::TRACK.bits;
        // }

        // Evaluate all tracks
        for i in 0..GRID_WIDTH * GRID_HEIGHT {
            evaluate_track_dir(i as u32, &mut self.grid)
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub(crate) fn update(&mut self, input: &mut WinitInputHelper, dt: f32) {
        // Todo: Remove this way of initialization.
        if !self.initialized {
            self.reset();
            self.initialized = true;
        };
        self.time_passed += dt;
        self.move_timer += dt;

        let mut player_dir_x: i32 = 0;
        let mut player_dir_y: i32 = 0;

        // Auto move
        if self.move_timer > self.move_interval {
            player_dir_x = self.last_dir_x;
            player_dir_y = self.last_dir_y;
            self.move_timer = 0.0;
        }

        if input.key_pressed(VirtualKeyCode::S) {
            self.move_interval += 0.2;
            self.move_interval = self.move_interval.min(3.0);
        }
        if input.key_pressed(VirtualKeyCode::W) {
            self.move_interval -= 0.2;
            self.move_interval = self.move_interval.max(0.2);
        }

        // Auto Scroll
        //self.grid_offset.x = -(self.time_passed * 4.0 / 2.0).round() * CELL_SIZE as f32;
        if input.key_pressed(VirtualKeyCode::Up) {
            self.grid_offset.y += CELL_SIZE as f32;
        }
        if input.key_pressed(VirtualKeyCode::Down) {
            self.grid_offset.y -= CELL_SIZE as f32;
        }
        if input.key_pressed(VirtualKeyCode::Left) {
            self.grid_offset.x += CELL_SIZE as f32;
        }
        if input.key_pressed(VirtualKeyCode::Right) {
            self.grid_offset.x -= CELL_SIZE as f32;
        }

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let index = y * GRID_WIDTH + x;

                if (self.grid[index] & CellType::PLAYER_FRONT.bits) > 0 {
                    let mut new_index = get_index(
                        (x as i32 + player_dir_x) as u32,
                        (y as i32 + player_dir_y) as u32,
                    ) as usize;

                    let mut is_track =
                        (self.grid[new_index] & CellType::TRACK.bits) == CellType::TRACK.bits;

                    if new_index != index && !is_track {
                        let up =
                            get_grid_cell_pos((x as i32) as u32, (y as i32 - 1) as u32, &self.grid);
                        let right =
                            get_grid_cell_pos((x as i32 + 1) as u32, (y as i32) as u32, &self.grid);
                        let down =
                            get_grid_cell_pos((x as i32) as u32, (y as i32 + 1) as u32, &self.grid);

                        if self.last_dir_x > 0 {
                            if up.is_some() {
                                if up.unwrap() & CellType::TRACK.bits > 0 {
                                    println!("UP");
                                    new_index = get_index((x as i32) as u32, (y as i32 - 1) as u32)
                                        as usize;
                                    player_dir_x = 0;
                                    player_dir_y = -1;
                                    is_track = true;
                                }
                            }
                            if down.is_some() {
                                if down.unwrap() & CellType::TRACK.bits > 0 {
                                    println!("DOWN");
                                    player_dir_x = 0;
                                    player_dir_y = 1;
                                    new_index = get_index((x as i32) as u32, (y as i32 + 1) as u32)
                                        as usize;
                                    is_track = true;
                                }
                            }
                        } else {
                            if right.is_some() {
                                if right.unwrap() & CellType::TRACK.bits > 0 {
                                    println!("RIGHT");
                                    player_dir_x = 1;
                                    player_dir_y = 0;
                                    new_index = get_index((x as i32 + 1) as u32, (y as i32) as u32)
                                        as usize;
                                    is_track = true;
                                }
                            }
                        }
                    }

                    if new_index != index && is_track {
                        let pos_x = x as i32;
                        let pos_y = y as i32;
                        if pos_x + player_dir_x >= 0
                            && pos_x + player_dir_x as i32 <= GRID_WIDTH as i32 - 1
                            && pos_y + player_dir_y >= 0
                            && pos_y + player_dir_y as i32 <= GRID_WIDTH as i32 - 1
                        {
                            self.new_grid[index] = self.grid[index] | CellType::PLAYER_MIDDLE.bits;
                            self.new_grid[index] =
                                self.new_grid[index] & !CellType::PLAYER_FRONT.bits;

                            self.new_grid[new_index] =
                                self.grid[new_index] | CellType::PLAYER_FRONT.bits;
                        }
                    } else {
                        self.new_grid[index] = self.grid[index] | CellType::PLAYER_FRONT.bits;
                    }
                } else if (self.grid[index] & 0b00000111) > 0 {
                    if player_dir_x != 0 || player_dir_y != 0 {
                        self.new_grid[index] = (self.grid[index]) - 1;
                    } else {
                        self.new_grid[index] = self.new_grid[index] | self.grid[index];
                    }
                } else {
                    self.new_grid[index] = self.new_grid[index] | self.grid[index];
                }
            }
        }

        for i in 0..GRID_WIDTH * GRID_HEIGHT {
            self.grid[i] = self.new_grid[i];
        }

        // Update train
        if player_dir_x != 0 || player_dir_y != 0 {
            self.train_test[0].vel = Vec2 {
                x: player_dir_x as f32,
                y: player_dir_y as f32,
            };

            self.last_dir_x = player_dir_x;
            self.last_dir_y = player_dir_y;
        }

        //let mut old_pos = self.train_test[0].pos - self.train_test[0].vel * 32.0;
        let mut old_vel = self.train_test[0].vel;
        self.train_test[0].pos = self.train_test[0].pos + self.train_test[0].vel;
        for cart in self.train_test.iter_mut().skip(1) {
            let old = cart.vel;
            cart.pos = cart.pos + old_vel;
            old_vel = old;
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

        // for cart in &self.train_test {
        //     renderer.draw_sprite(cart.pos, &cart.sprite);
        // }

        let info = format!("Interval {}", self.move_interval);
        renderer.draw_text(
            Vec2 { x: 32.0, y: 60.0 },
            &info,
            32.0,
            24.0,
            [0x18, 0x7d, 0x0f, 0xff],
        );

        renderer.set_offset(Vec2 {
            x: CELL_SIZE,
            y: CELL_SIZE,
        });

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let index = y * GRID_WIDTH + x;

                if self.grid_offset.x + (x as f32) * CELL_SIZE < 0.0
                    || self.grid_offset.x + (x as f32) * CELL_SIZE + CELL_SIZE > WIDTH as f32
                    || self.grid_offset.y + (y as f32) * CELL_SIZE < 0.0
                    || self.grid_offset.y + (y as f32) * CELL_SIZE + CELL_SIZE > HEIGHT as f32
                {
                    continue;
                }

                if self.grid[index] == CellType::EMPTY.bits {
                    renderer.draw_sprite_color(
                        self.grid_offset
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        &self.grass_sprite,
                        [0xcc, 0xb2, 0x10, 0xff],
                    );
                }

                if (self.grid[index] & CellType::TRACK.bits) > 0 {
                    let directions = (self.grid[index] >> 6) as u8;

                    // Up - Down Direction
                    if (directions & Directions::UP.bits) > 0
                        && (directions & Directions::DOWN.bits) > 0
                    {
                        if (directions & Directions::RIGHT.bits) > 0 {
                            renderer.draw_sprite_color(
                                self.grid_offset
                                    + Vec2 {
                                        x: x as f32 * 32.0,
                                        y: y as f32 * 32.0,
                                    },
                                &self.track_tile_t_junction_flipped,
                                CELL_TRACK_COLOR,
                            );
                        } else {
                            renderer.draw_sprite_color(
                                self.grid_offset
                                    + Vec2 {
                                        x: x as f32 * 32.0,
                                        y: y as f32 * 32.0,
                                    },
                                &self.track_tile_normal,
                                CELL_TRACK_COLOR,
                            );
                        }
                    }
                    // Right - left direction
                    else if (directions & Directions::RIGHT.bits) > 0
                        && (directions & Directions::LEFT.bits) > 0
                    {
                        if (directions & Directions::DOWN.bits) > 0 {
                            renderer.draw_sprite_color(
                                self.grid_offset
                                    + Vec2 {
                                        x: x as f32 * 32.0,
                                        y: y as f32 * 32.0,
                                    },
                                &self.track_tile_t_junction_flipped,
                                CELL_TRACK_COLOR,
                            );
                        } else {
                            renderer.draw_sprite_color(
                                self.grid_offset
                                    + Vec2 {
                                        x: x as f32 * 32.0,
                                        y: y as f32 * 32.0,
                                    },
                                &self.track_tile_flipped,
                                CELL_TRACK_COLOR,
                            );
                        }
                    }
                    // Corners
                    // Up - Right
                    else if (directions & Directions::UP.bits) > 0
                        && (directions & Directions::RIGHT.bits) > 0
                    {
                        renderer.draw_sprite_color(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.track_tile_corner_t_r,
                            CELL_TRACK_COLOR,
                        );
                    }
                    // Up - Left
                    else if (directions & Directions::UP.bits) > 0
                        && (directions & Directions::LEFT.bits) > 0
                    {
                        renderer.draw_sprite_color(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.track_tile_corner_t_l,
                            CELL_TRACK_COLOR,
                        );
                    }
                    // Down - Right
                    else if (directions & Directions::DOWN.bits) > 0
                        && (directions & Directions::RIGHT.bits) > 0
                    {
                        renderer.draw_sprite_color(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.track_tile_corner_b_r,
                            CELL_TRACK_COLOR,
                        );
                    }
                    // Down - Left
                    else if (directions & Directions::DOWN.bits) > 0
                        && (directions & Directions::LEFT.bits) > 0
                    {
                        renderer.draw_sprite_color(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.track_tile_corner_b_l,
                            CELL_TRACK_COLOR,
                        );
                    } else {
                        renderer.draw_sprite_color(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.track_tile_crossing,
                            CELL_TRACK_COLOR,
                        );
                    }
                }

                if (self.grid[index] & CellType::PLAYER_FRONT.bits) > 0 {
                    if self.last_dir_x > 0 {
                        renderer.draw_sprite(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.player_sprite_right,
                        );
                    } else if self.last_dir_y > 0 {
                        renderer.draw_sprite(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.player_sprite_up,
                        );
                    } else {
                        renderer.draw_sprite(
                            self.grid_offset
                                + Vec2 {
                                    x: x as f32 * 32.0,
                                    y: y as f32 * 32.0,
                                },
                            &self.player_sprite_down,
                        );
                    }
                }

                if (self.grid[index] & 0b00000111) > 0 {
                    renderer.draw_sprite(
                        self.grid_offset
                            + Vec2 {
                                x: x as f32 * 32.0,
                                y: y as f32 * 32.0,
                            },
                        &self.train_test[0].sprite,
                    );
                }
            }
        }
    }
}
