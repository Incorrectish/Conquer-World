use crate::{direction::Direction, WORLD_SIZE, enemy::{Enemy, self}, projectile::Projectile, world::World, tile, movable::Movable};
use crate::entity::Entity;
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
            color: tile::PLAYER,
            health: MAX_PLAYER_HEALTH,
        };
        world[temp.pos.1][temp.pos.0] = temp.color;
        temp
    }

    // eventually this should be the functionality to like shoot projectiles and stuff but for now
    // it just handles like arrow keys
    pub fn use_input(
        key: KeyInput,
        world: &mut World,
    ) {
        match key.keycode {
            Some(key_pressed) => match key_pressed {
                KeyCode::Down => {
                    world.player.direction = Direction::South;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Up => {
                    world.player.direction = Direction::North;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Left => {
                    world.player.direction = Direction::West;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Right => {
                    world.player.direction = Direction::East;
                    World::travel(world, Entity::Player);
                },

                // Arbitrarily chosen for attack, can change later
                MELEE_ATTACK_KEYCODE => {
                    Player::melee_attack(world);
                },
                PROJECTILE_ATTACK_KEYCODE => {
                    Player::projectile_attack(world);
                }
                _ => {}
            },
            None => {}
        }
    }


    pub fn melee_attack(world: &mut World) {
        // gets the position that the attack will be applied to, one tile forward of the player in
        // the direction that they are facing
        let attacking_position = World::new_position(world.player.pos.0, world.player.pos.1, &world.player.direction);
        
        // We do not know what enemies are on the tile being attacked, so we need to go through the
        // enemies and check if any of them are on the attacking tile, then damage them
        for enemy in &mut world.enemies {
            if enemy.pos == attacking_position {
                enemy.health -= PLAYER_MELEE_DAMAGE;
            }
        }
    }

    // This function should just spawn a projectile, the mechanics of dealing with the projectile
    // and such should be determined by the projectile object itself
    pub fn projectile_attack(world: &mut World) {
        let projectile_spawn_pos = World::new_position(world.player.pos.0, world.player.pos.1, &world.player.direction);
        let projectile = Projectile::new(projectile_spawn_pos.0, projectile_spawn_pos.1, PLAYER_PROJECTILE_SPEED, world.player.direction.clone(), world);
        world.projectiles.push(projectile);
    }

}

impl Movable for Player {
    fn set_pos(&mut self, new_pos: (usize, usize)) {
        todo!()
    }

    fn get_pos(&self) -> (usize, usize) {
        todo!()
    }

    fn get_x(&self) -> usize {
        todo!()
    }

    fn get_y(&self) -> usize {
        todo!()
    }

    fn get_covered_tile(&self) -> [f32; 4] {
        todo!()
    }

    fn set_covered_tile(&mut self, new_tile: [f32; 4]) {
        todo!()
    }

    fn get_color(&self) -> [f32; 4] {
        todo!()
    }

    fn get_direction(&self) -> Direction {
        todo!()
    }
}
