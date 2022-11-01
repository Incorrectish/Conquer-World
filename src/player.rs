use crate::{direction::Direction, WORLD_SIZE};
use ggez::input::keyboard::{KeyCode, KeyInput};

pub struct Player {
    pos: (usize, usize),
    direction: Direction,
    covered_tile: [f32; 4],
    color: [f32; 4],
}

impl Player {
    pub fn new(mut world: [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) -> Self {
        let temp = Self {
            pos: (0, 0),
            direction: Direction::North,
            covered_tile: world[0][0],
            color: [1., 1., 1., 1.],
        };
        world[temp.pos.1][temp.pos.0] = temp.color;
        temp
    }

    pub fn use_input(
        &mut self,
        key: KeyInput,
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
    ) {
        match key.keycode {
            Some(key_pressed) => match key_pressed {
                KeyCode::Down => {
                    self.direction = Direction::South;
                    self.travel(world);
                }
                KeyCode::Up => {
                    self.direction = Direction::North;
                    self.travel(world);
                }
                KeyCode::Left => {
                    self.direction = Direction::West;
                    self.travel(world);
                }
                KeyCode::Right => {
                    self.direction = Direction::East;
                    self.travel(world);
                }
                _ => {}
            },
            None => {}
        }
    }

    pub fn travel(
        &mut self,
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
    ) {
        world[self.pos.1][self.pos.0] = self.covered_tile;
        match self.direction {
            Direction::North => {
                if self.pos.1 > 0 {
                    self.pos.1 -= 1
                }
            }
            Direction::South => {
                if self.pos.1 < (WORLD_SIZE.1 - 1) as usize {
                    self.pos.1 += 1
                }
            }
            Direction::East => {
                if self.pos.0 < (WORLD_SIZE.0 - 1) as usize {
                    self.pos.0 += 1
                }
            }
            Direction::West => {
                if self.pos.0 > 0 {
                    self.pos.0 -= 1
                }
            }
        }
        self.covered_tile = world[self.pos.1][self.pos.0];
        world[self.pos.1][self.pos.0] = self.color;
    }
}
