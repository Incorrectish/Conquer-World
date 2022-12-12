use crate::{
    direction::Direction, entity::Entity, player::Player, tile, utils::Position, world::World,
    TILE_SIZE, WORLD_SIZE,
};
use ggez::graphics::{self, Canvas};
use std::collections::HashMap;

const PERMISSIBLE_TILES: [[f32; 4]; 4] = [
    tile::WATER,
    tile::GRASS,
    tile::PLAYER,
    tile::PROJECTILE_PLAYER,
];

pub struct Projectile {
    pub pos: Position,
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
        color: [f32; 4],
    ) -> Self {
        let temp = Projectile {
            pos: Position::new(x, y),
            speed,
            damage,
            direction,
            color,
        };
        temp
    }

    pub fn update(world: &mut World) {
        let mut index = 0;
        for _ in 0..world.projectiles.len() {
            match world.projectiles[index] {
            //     tile::LIGHTNING_PLACEHOLDER => {
            //         let pos = world.projectiles[index].pos;
            //         world.projectiles[index].color = tile::LIGHTNING_INITIAL;
            //         world.world[pos.1][pos.0] = tile::LIGHTNING_INITIAL;
            //     }
            //     tile::LIGHTNING_INITIAL => {
            //         let pos = world.projectiles[index].pos;
            //         world.projectiles[index].color = tile::LIGHTNING_SECONDARY;
            //         world.world[pos.1][pos.0] = tile::LIGHTNING_SECONDARY;
            //     }
            //     tile::LIGHTNING_SECONDARY => {
            //         let deltas: [i16; 3] = [0, 1, -1];
            //         let pos = world.projectiles[index].pos;
            //         world.projectiles[index].color = tile::LIGHTNING_FINAL;
            //         // basically checks the 8 around and including the projectile and turns
            //         // them to their original state
            //         for x_delta in deltas {
            //             for y_delta in deltas {
            //                 if (pos.0 < (WORLD_SIZE.0 - x_delta) as usize
            //                     && pos.1 < (WORLD_SIZE.1 - y_delta) as usize
            //                     && pos.0 as i16 >= -x_delta
            //                     && pos.1 as i16 >= -y_delta)
            //                 {
            //                     world.world[(pos.1 as i16 + y_delta) as usize]
            //                         [(pos.0 as i16 + x_delta) as usize] = tile::LIGHTNING_FINAL;
            //                 }
            //             }
            //         }
            //     }
            //     tile::LIGHTNING_FINAL => {
            //         let deltas: [i16; 3] = [0, 1, -1];
            //         let pos = world.projectiles[index].pos;
            //
            //         // basically checks the 8 around and including the projectile and turns
            //         // them to their original state
            //         for x_delta in deltas {
            //             for y_delta in deltas {
            //                 if (pos.0 < (WORLD_SIZE.0 - x_delta) as usize
            //                     && pos.1 < (WORLD_SIZE.1 - y_delta) as usize
            //                     && pos.0 as i16 >= -x_delta
            //                     && pos.1 as i16 >= -y_delta)
            //                 {
            //                     world.world[(pos.1 as i16 + y_delta) as usize]
            //                         [(pos.0 as i16 + x_delta) as usize] = world.board
            //                         [(pos.1 as i16 + y_delta) as usize]
            //                         [(pos.0 as i16 + x_delta) as usize];
            //                 }
            //             }
            //         }
            //     }
                _ => {
                    if !World::travel(world, Entity::Projectile(index)) {
                        Projectile::kill(index, world);
                        //When projectile dies, whole array shifts back one,
                        //so need to account for this in order to check the next projectile  in array
                        index -= 1;
                    }
                }
            }
            index += 1;
            // case for impact with player

            // case for impact with enemy

            // general projectile movement
        }
    }

    pub fn kill(index: usize, world: &mut World) {
        world.entity_positions.remove(&world.projectiles[index].pos);
        world.projectiles.remove(index);
    }

    pub fn can_travel_to(
        position: Position,
        entity_positions: &HashMap<Position, ([f32; 4], Entity)>,
        terrain_positions: &HashMap<Position, [f32; 4]>,
    ) -> bool {
        if entity_positions.contains_key(&position) || terrain_positions.contains_key(&position) {
            let info = entity_positions.get(&position);
            let info2 = terrain_positions.get(&position);
            if let Some(info) = info {
                if PERMISSIBLE_TILES.contains(&info.0) {
                    return true;
                }
            }

            if let Some(info) = info2 {
                if PERMISSIBLE_TILES.contains(&info) {
                    return true;
                }
            }
            return false;
        }
        true
    }
}
