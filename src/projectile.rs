use crate::{
    direction::Direction,
    player::Player,
    WORLD_SIZE, world::World,
};

pub struct Projectile {
    pub pos: (usize, usize),
    speed: usize,
    pub direction: Direction,
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
        world: &mut World,
    ) -> Self {
        let color = [1., 0., 0., 0.5];
        let temp = Projectile {
            pos: (x, y),
            speed,
            direction,
            color,
            covered_tile: world.world[y][x],
        };
        world.world[y][x] = color;
        temp
    }

    pub fn update(
        world: &mut World
    ) {
        for index in (0..world.projectiles.len()).rev() {
            let new_position = World::new_position(
                world.projectiles[index].pos.0,
                world.projectiles[index].pos.1,
                &world.projectiles[index].direction,
            );

            // if the projectile goes out of bounds, the position won't change 
            if world.projectiles[index].pos == new_position {
                Projectile::kill(index, world);
                world.projectiles.remove(index);
                return;
            }

            // case for impact with player

            // case for impact with enemy

            // general projectile movement

        }
    }

    pub fn kill(index: usize, world: &mut World) {
        world.world[world.projectiles[index].pos.1][world.projectiles[index].pos.0] = world.projectiles[index].covered_tile;
    }
}

