use crate::{direction::Direction, player::Player, world::World, WORLD_SIZE, entity::Entity, tile};

const PERMISSIBLE_TILES: [[f32; 4]; 2] = [tile::WATER, tile::FLOOR];

pub struct Projectile {
    pub pos: (usize, usize),
    pub speed: usize,
    pub direction: Direction,
    color: [f32; 4],
    covered_tile: [f32; 4],
    // maybe add an alignment so projectiles from enemies cannot damage themselves and projectiles
    // from players cannot damage themselves
}

impl Projectile {
    pub fn new(x: usize, y: usize, speed: usize, direction: Direction, world: &mut World) -> Self {
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

    pub fn update(world: &mut World) {
        for index in (0..world.projectiles.len()).rev() {
            // if the projectile goes out of bounds, the position won't change
           
            // CURRENTLY THIS WON'T WORK ON IMPACTS BECAUSE PROJECTILE THIKS THAT ENEMIES/PLAYERS
            // ARE ILLEGAL TILES AND DESTROYS ITSELF. ADD TO PERMISSIBLE_TILES TO FIX THIS
            if !World::travel(world, Entity::Enemy(index)) {
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
        world.world[world.projectiles[index].pos.1][world.projectiles[index].pos.0] =
            world.projectiles[index].covered_tile;
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
