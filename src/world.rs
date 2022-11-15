use crate::{
    WORLD_SIZE,
    player::Player,
    enemy::Enemy,
    projectile::Projectile,
    tile,
    random
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
    // generates the center boss room for map
    pub fn gen_boss(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        let x: usize = (WORLD_SIZE.0 as usize) / 2 - 1;
        let y: usize = (WORLD_SIZE.1 as usize) / 2 - 1;
        for i in 0..8 {
            for j in 0..8 {
                world[x-3+i][y-3+j] = tile::WALL;
            }
        }
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

            Self::gen_water_helper(rng, x, y, 0, world); // new lake center at (x, y)
            lakes_added += 1;
        }
    }

    fn gen_water_helper(rng: &mut ThreadRng, x: i16, y: i16, dist: i16, world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        if world[x as usize][y as usize] == tile::FLOOR {
            world[x as usize][y as usize] = tile::WATER;
        }

        const DIRECTIONS: [[i16; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
        for dir in DIRECTIONS {
            if random::bernoulli(rng, 1. - 0.2 * (dist as f32)) {
                let i = x + dir[0];
                let j = y + dir[1];
                if i >= 0 && i < WORLD_SIZE.0 && j >= 0 && j < WORLD_SIZE.1 {
                    Self::gen_water_helper(rng, i, j, dist+1, world);
                }
            }
        }
    }
}

