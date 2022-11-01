use crate::{direction::Direction, WORLD_SIZE, enemy::{Enemy, self}, projectile::Projectile};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::event::VirtualKeyCode;

// Can change easily
const MAX_PLAYER_HEALTH: usize = 10;
const PLAYER_MELEE_DAMAGE: usize = 1;
const MELEE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::A;
const PROJECTILE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::Space;
const PLAYER_PROJECTILE_SPEED: usize = 1;

// This is with the covered tile model, but we could use the static/dynamic board paradighm or
// something else entirely
pub struct Player {
    // This is the position in the form (x, y)
    pos: (usize, usize),
    
    // The direction that the player is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    direction: Direction,
    
    // This simply stores the color of the tile that the player is currently on, so that when they
    // move off of it, it can be rendered properly back to what it was 
    covered_tile: [f32; 4],

    // This is the player color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the player is represented as a solid color
    color: [f32; 4],

    // Stores player health: for player death and such
    health: usize,

    // 
}

impl Player {
    pub fn new(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) -> Self {
        let temp = Self {
            pos: (0, 0),
            direction: Direction::North,
            covered_tile: world[0][0],
            color: [1., 1., 1., 1.],
            health: MAX_PLAYER_HEALTH,
        };
        world[temp.pos.1][temp.pos.0] = temp.color;
        temp
    }

    // eventually this should be the functionality to like shoot projectiles and stuff but for now
    // it just handles like arrow keys
    pub fn use_input(
        &mut self,
        key: KeyInput,
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
        enemies: &mut Vec<Enemy>,
        projectiles: &mut Vec<Projectile>,
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
                },

                // Arbitrarily chosen for attack, can change later
                MELEE_ATTACK_KEYCODE => {
                    self.melee_attack(enemies);
                },
                PROJECTILE_ATTACK_KEYCODE => {
                    self.projectile_attack(projectiles, world);
                }
                _ => {}
            },
            None => {}
        }
    }

    // this is the "move()" function but move is a reserved keyword so I just used the first
    // synonym I googled "travel()"
    pub fn travel(
        &mut self,
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
    ) {
        let new_position = Self::new_position(self.pos.0, self.pos.1, &self.direction);
        // TODO: refactor the colors to be some sort of enum
        // If the new position is a tile that can be traveled to "all black" for now, then 
        // remove the player from the current tile and place it on the new tile 
        if world[new_position.1][new_position.0] == [0., 0., 0., 0.] {
            world[self.pos.1][self.pos.0] = self.covered_tile;
            self.pos = new_position;
            self.covered_tile = world[self.pos.1][self.pos.0];
            world[self.pos.1][self.pos.0] = self.color;
        }
    }

    pub fn melee_attack(&mut self, enemies: &mut Vec<Enemy>) {
        // gets the position that the attack will be applied to, one tile forward of the player in
        // the direction that they are facing
        let attacking_position = Self::new_position(self.pos.0, self.pos.1, &self.direction);
        
        // We do not know what enemies are on the tile being attacked, so we need to go through the
        // enemies and check if any of them are on the attacking tile, then damage them
        for enemy in enemies {
            if enemy.pos == attacking_position {
                enemy.health -= PLAYER_MELEE_DAMAGE;
            }
        }
    }

    // This function should just spawn a projectile, the mechanics of dealing with the projectile
    // and such should be determined by the projectile object itself
    pub fn projectile_attack(&self, projectiles: &mut Vec<Projectile>, world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        let projectile_spawn_pos = Self::new_position(self.pos.1, self.pos.0, &self.direction);
        projectiles.push(Projectile::new(projectile_spawn_pos.0, projectile_spawn_pos.1, PLAYER_PROJECTILE_SPEED, self.direction.clone(), world));
    }

    // This very simply gets the new position from the old, by checking the direction and the
    // bounds. Should be refactored to give a travel distance instead of just one
    pub fn new_position(mut x: usize, mut y: usize, direction: &Direction) -> (usize, usize) {
        match direction {
            Direction::North => {
                if y > 0 as usize {
                    y -= 1
                }
            }
            Direction::South => {
                if y < (WORLD_SIZE.1 - 1) as usize {
                    y += 1
                }
            }
            Direction::East => {
                if x < (WORLD_SIZE.0 - 1) as usize {
                    x += 1
                }
            }
            Direction::West => {
                if x > 0 as usize {
                    x -= 1
                }
            }
        }
        (x, y)
    }
}
