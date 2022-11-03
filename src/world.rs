use crate::{
    WORLD_SIZE,
    player::Player,
    enemy::Enemy,
    projectile::Projectile,
    tile
};

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

//    pub fn gen_water(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
//        // replace following lines with randomly pick a point
//        let x = 35;
//        let y = 35;
//    }
//
//    pub fn gen_water(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize], x: i32, y: i32) {
//
//    }
}

