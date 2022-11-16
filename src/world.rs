use crate::{
    WORLD_SIZE,
    player::Player,
    enemy::Enemy,
    projectile::Projectile,
    tile,
    random, 
    direction::Direction, movable::Movable,
    entity::Entity,
};
use rand::rngs::ThreadRng;

pub struct World {
    // world to store the state of tiles in between frames
    pub world: [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],

    // store an instance of a player
    pub player: Player,

    // list of enemies in our world
    pub enemies: Vec<Enemy>,

    // list of all the projectiles in the world
    pub projectiles: Vec<Projectile>,
}

impl World {

    // this is the "move()" function but move is a reserved keyword so I just used the first
    // synonym I googled "travel()"
    pub fn travel(
        world: &mut World,
        entity_type: Entity,
    ) {
        let entity = match entity_type {
            Entity::Player => &world.player,
            Entity::Enemy(i) => &world::enemies[i],
            Entity::Projectile(i) => &world.projectiles[i],
        }
        let new_position = Self::new_position(entity.get_x(), entity.get_y(), &entity.get_direction());
        // TODO: refactor the colors to be some sort of enum
        // If the new position is a tile that can be traveled to "all black" for now, then 
        // remove the player from the current tile and place it on the new tile 
        if world.world[new_position.1][new_position.0] == [0., 0., 0., 0.] {
            // TODO: refactor to remove covered tile, layer approach created by Ishan and Michael
            world.world[entity.get_y()][entity.get_x()] = entity.get_covered_tile();
            entity.set_pos(new_position);
            entity.set_covered_tile(world.world[entity.get_y()][entity.get_x()]);
            world.world[entity.get_y()][entity.get_x()] = entity.get_color();
        }
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

    // generates the center boss room for map
    pub fn gen_boss(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        // x and y of center of map
        let x: usize = (WORLD_SIZE.0 as usize) / 2 - 1;
        let y: usize = (WORLD_SIZE.1 as usize) / 2 - 1;

        // builds a 8x8 square around the center of WALL tiles
        for i in 0..8 {
            for j in 0..8 {
                world[x-3+i][y-3+j] = tile::WALL;
            }
        }

        // builds a 2x2 square in the center of PORTAL tiles
        world[x][y] = tile::PORTAL;
        world[x+1][y] = tile::PORTAL;
        world[x][y+1] = tile::PORTAL;
        world[x+1][y+1] = tile::PORTAL;
    }

    // generates water tiles around the map
    pub fn gen_water(rng: &mut ThreadRng, world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        let mut lakes_added = 0;
        const TOTAL_LAKES: i16 = 5;
        while lakes_added < TOTAL_LAKES {
            let x = random::rand_range(rng, 5, WORLD_SIZE.0); // random x coordinate
            let y = random::rand_range(rng, 5, WORLD_SIZE.1); // random y coordinate

            Self::gen_water_helper(rng, x, y, 0, world); // new lake centered at (x, y)
            lakes_added += 1;
        }
    }

    // Recursively generates lakes -- floodfill-esque idea around the center, but expansion is
    // limited probabilistically (probability of expansion decreases as we range further from the
    // center)
    fn gen_water_helper(rng: &mut ThreadRng, x: i16, y: i16, dist: i16, world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        // sets curr tile to water
        if world[x as usize][y as usize] == tile::FLOOR {
            world[x as usize][y as usize] = tile::WATER;
        }

        const DIRECTIONS: [[i16; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]]; // orthogonal dirs
        for dir in DIRECTIONS { // for each tile in an orthogonal direction
            // With certain probability, continue expanding lake in that direction
            if Self::prob_expand_water(rng, dist) {
                let i = x + dir[0];
                let j = y + dir[1];
                // if in bounds, recursively call fn on adjacent tile (draws WATER at that tile)
                if i >= 0 && i < WORLD_SIZE.0 && j >= 0 && j < WORLD_SIZE.1 {
                    Self::gen_water_helper(rng, i, j, dist+1, world);
                }
            }
        }
    }

    // Gets probability of continuing to expand lake outwards
    fn prob_expand_water(rng: &mut ThreadRng, dist: i16) -> bool {
        random::bernoulli(rng, 1. - 0.2 * (dist as f32))
    }
}

