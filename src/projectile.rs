use crate::{
    direction::Direction,
    player::Player,
    WORLD_SIZE
};

pub struct Projectile {
    pos: (usize, usize),
    speed: usize,
    direction: Direction,
    color: [f32; 4],
    covered_tile: [f32; 4],
    // maybe add an alignment so projectiles from enemies cannot damage themselves and projectiles
    // from players cannot damage themselves
}

impl Projectile {
    pub fn new(
        x: usize,
        y: usize,
        speed: usize,
        direction: Direction,
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
    ) -> Self {
        let color = [1., 0., 0., 0.5];
        let temp = Projectile {
            pos: (x, y),
            speed,
            direction,
            color,
            covered_tile: world[y][x],
        };
        world[y][x] = color;
        temp
    }

    pub fn update(
        projectiles: &mut Vec<Projectile>,
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
    ) {
        for index in (0..projectiles.len()).rev() {
            let new_position = Player::new_position(
                projectiles[index].pos.0,
                projectiles[index].pos.1,
                &projectiles[index].direction,
            );

            // if the projectile goes out of bounds, the position won't change 
            if projectiles[index].pos == new_position {
                projectiles[index].kill(world);
                projectiles.remove(index);
            }

            // case for impact with player

            // case for impact with enemy
        }
    }

    pub fn kill(&self, world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        world[self.pos.1][self.pos.0] = self.covered_tile;
    }
}
