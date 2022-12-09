use crate::{direction::Direction, entity::Entity, player::Player, tile, world::World, WORLD_SIZE};

const PERMISSIBLE_TILES: [[f32; 4]; 4] = [tile::WATER, tile::GRASS, tile::PLAYER, tile::ENEMY];

pub struct Projectile {
    pub pos: (usize, usize),
    pub speed: usize,
    pub direction: Direction,
    pub color: [f32; 4],
    pub damage: usize,
    // maybe add an alignment so projectiles from enemies cannot damage themselves and projectiles
    // from players cannot damage themselves
}

impl Projectile {
    pub fn new(
        x: usize,
        y: usize,
        speed: usize,
        damage: usize,
        direction: Direction,
        world: &mut World,
        color: [f32; 4]
    ) -> Self {
        let temp = Projectile {
            pos: (x, y),
            speed,
            damage,
            direction,
            color
        };
        world.world[y - world.y_offset][x - world.x_offset] = temp.color;
        temp
    }

    pub fn update(world: &mut World) {
        for index in (0..world.projectiles.len()).rev() {
            // if the projectile goes out of bounds, the position won't change

            if world.projectiles[index].speed != 0 {
                // CURRENTLY THIS WON'T WORK ON IMPACTS BECAUSE PROJECTILE THIKS THAT ENEMIES/PLAYERS
                // ARE ILLEGAL TILES AND DESTROYS ITSELF. ADD TO PERMISSIBLE_TILES TO FIX THIS
                if !World::travel(world, Entity::Projectile(index)) {
                    Projectile::kill(index, world);
                    return;
                }
            } else {
                // This is only lightning for now
                match world.projectiles[index].color {
                    tile::LIGHTNING_PLACEHOLDER => {
                        let pos = world.projectiles[index].pos;
                        world.projectiles[index].color = tile::LIGHTNING_INITIAL;
                        world.world[pos.1][pos.0] = tile::LIGHTNING_INITIAL;
                    },
                    tile::LIGHTNING_INITIAL => {
                        let pos = world.projectiles[index].pos;
                        world.projectiles[index].color = tile::LIGHTNING_SECONDARY;
                        world.world[pos.1][pos.0] = tile::LIGHTNING_SECONDARY;
                    }
                    tile::LIGHTNING_SECONDARY => {
                        let deltas: [i16; 3] = [0, 1, -1];
                        let pos = world.projectiles[index].pos;
                        world.projectiles[index].color = tile::LIGHTNING_FINAL;
                        // basically checks the 8 around and including the projectile and turns
                        // them to their original state
                        for x_delta in deltas {
                            for y_delta in deltas {
                                if (pos.0 < (WORLD_SIZE.0 - x_delta) as usize
                                    && pos.1 < (WORLD_SIZE.1 - y_delta) as usize
                                    && pos.0 as i16 >= -x_delta
                                    && pos.1 as i16 >= -y_delta)
                                {
                                    world.world[(pos.1 as i16 + y_delta) as usize]
                                        [(pos.0 as i16 + x_delta) as usize] = tile::LIGHTNING_FINAL;
                                }
                            }
                        }
                    }
                    tile::LIGHTNING_FINAL => {
                        let deltas: [i16; 3] = [0, 1, -1];
                        let pos = world.projectiles[index].pos;

                        // basically checks the 8 around and including the projectile and turns
                        // them to their original state
                        for x_delta in deltas {
                            for y_delta in deltas {
                                if (pos.0 < (WORLD_SIZE.0 - x_delta) as usize
                                    && pos.1 < (WORLD_SIZE.1 - y_delta) as usize
                                    && pos.0 as i16 >= -x_delta
                                    && pos.1 as i16 >= -y_delta)
                                {
                                    world.world[(pos.1 as i16 + y_delta) as usize]
                                        [(pos.0 as i16 + x_delta) as usize] = world.board
                                        [(pos.1 as i16 + y_delta) as usize]
                                        [(pos.0 as i16 + x_delta) as usize];
                                }
                            }
                        }
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }

            // case for impact with player

            // case for impact with enemy

            // general projectile movement
        }
    }

    pub fn kill(index: usize, world: &mut World) {
        if World::coordinates_are_within_world(
            world,
            world.projectiles[index].pos.0,
            world.projectiles[index].pos.1,
        ) {
            world.world[world.projectiles[index].pos.1 - world.y_offset]
                [world.projectiles[index].pos.0 - world.x_offset] =
                world.board[world.projectiles[index].pos.1][world.projectiles[index].pos.0];
        }
        world.projectiles.remove(index);
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
