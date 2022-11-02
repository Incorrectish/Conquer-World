use crate::{
    WORLD_SIZE,
    player::Player,
    enemy::Enemy,
    projectile::Projectile,
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
