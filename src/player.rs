use crate::{
    direction::Direction,
    enemy::{self, Enemy},
    entity::Entity,
    projectile::Projectile,
    tile,
    world::World,
    WORLD_SIZE,
};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::event::VirtualKeyCode;

// Can change easily
const MAX_PLAYER_HEALTH: usize = 10;
const PLAYER_MELEE_DAMAGE: usize = 1;
const MELEE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::A;
// TODO look over these values
const HEAL_ABILITY_RETURN: usize = 2;
const HEAL_KEYCODE: VirtualKeyCode = KeyCode::H;
const LIGHTNING_KEYCODE: VirtualKeyCode = KeyCode::L;
const BUILD_KEYCODE: VirtualKeyCode = KeyCode::B;
const PROJECTILE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::Space;
const PLAYER_PROJECTILE_SPEED: usize = 1;
const PLAYER_PROJECTILE_DAMAGE: usize = 1;
const PLAYER_INITIAL_SPEED: usize = 1;
const PLAYER_INITIAL_ENERGY: usize = 5;
const PERMISSIBLE_TILES: [[f32; 4]; 1] = [tile::GRASS];

// This is with the covered tile model, but we could use the static/dynamic board paradighm or
// something else entirely
pub struct Player {
    // This is the position in the form (x, y)
    pub pos: (usize, usize),

    // The direction that the player is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    pub direction: Direction,

    // This controls the number of tiles a player moves in a direction in a given keypress
    pub speed: usize,

    // This is the player color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the player is represented as a solid color
    color: [f32; 4],

    // Stores player health: for player death and such
    health: usize,

    // planned energy, used for healing, projectiles, (teleportation?), building
    energy: usize,

    // Queued position: for mouse clicks and so on
    pub queued_position: Option<(usize, usize)>,
}

impl Player {
    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        self.health -= damage
    }

    pub fn new(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) -> Self {
        let temp = Self {
            pos: (0, 0),
            direction: Direction::North,
            speed: PLAYER_INITIAL_SPEED,
            color: tile::PLAYER,
            health: MAX_PLAYER_HEALTH,
            energy: PLAYER_INITIAL_ENERGY,
            queued_position: None,
        };
        world[temp.pos.1][temp.pos.0] = temp.color;
        temp
    }

    // eventually this should be the functionality to like shoot projectiles and stuff but for now
    // it just handles like arrow keys
    pub fn use_input(key: KeyInput, world: &mut World) {
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
                }

                // Arbitrarily chosen for attack, can change later
                MELEE_ATTACK_KEYCODE => {
                    Player::melee_attack(world);
                }
                PROJECTILE_ATTACK_KEYCODE => {
                    if (world.player.energy > 0) {
                        Player::projectile_attack(world);
                        // world.player.energy -= 1;
                        // commented out so I can test everything
                    }
                }
                HEAL_KEYCODE => {
                    if world.player.energy >= 5 {
                        world.player.health += HEAL_ABILITY_RETURN;
                        world.player.energy -= 5;
                    }
                }
                BUILD_KEYCODE => {
                    Player::build(world);
                }
                LIGHTNING_KEYCODE => {
                    Player::lightning(world);
                }
                _ => {}
            },
            None => {}
        }
    }

    pub fn lightning(world: &mut World) {
        if let Some(queued_position) = world.player.queued_position {
            // TODO: Damage
            let projectile = Projectile::new(
                queued_position.0,
                queued_position.1,
                0,
                3,
                Direction::North,
                world,
                tile::LIGHTNING_PLACEHOLDER,
            );
            world.projectiles.push(projectile)
        }
    }

    pub fn build(world: &mut World) {
        let position = World::new_position(
            world.player.pos.0,
            world.player.pos.1,
            world.player.direction.clone(),
            world,
            1,
        );
        if world.world[position.1][position.0] == tile::STRUCTURE {
            world.world[position.1][position.0] = world.board[position.1][position.0]
        } else if position != world.player.pos {
            world.world[position.1][position.0] = tile::STRUCTURE;
        }
    }

    pub fn melee_attack(world: &mut World) {
        // gets the position that the attack will be applied to, one tile forward of the player in
        // the direction that they are facing
        let attacking_position = World::new_position(
            world.player.pos.0,
            world.player.pos.1,
            world.player.direction.clone(),
            world,
            world.player.speed,
        );

        // We do not know what enemies are on the tile being attacked, so we need to go through the
        // enemies and check if any of them are on the attacking tile, then damage them
        for enemy in &mut world.enemies {
            if enemy.pos == attacking_position {
                enemy.damage(PLAYER_MELEE_DAMAGE);
            }
        }
    }

    // This function should just spawn a projectile, the mechanics of dealing with the projectile
    // and such should be determined by the projectile object itself
    pub fn projectile_attack(world: &mut World) {
        let projectile_spawn_pos = World::new_position(
            world.player.pos.0,
            world.player.pos.1,
            world.player.direction.clone(),
            world,
            world.player.speed,
        );
        if projectile_spawn_pos != world.player.pos {
            let projectile = Projectile::new(
                projectile_spawn_pos.0,
                projectile_spawn_pos.1,
                PLAYER_PROJECTILE_SPEED,
                PLAYER_PROJECTILE_DAMAGE,
                world.player.direction.clone(),
                world,
                tile::PROJECTILE,
            );
            world.projectiles.push(projectile);
        }
    }

    pub fn can_travel_to(tile: [f32; 4]) -> bool {
        for permissible_tile in PERMISSIBLE_TILES {
            if tile == permissible_tile {
                return true;
            }
        }
        false
    }
}
