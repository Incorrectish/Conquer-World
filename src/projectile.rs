use crate::{
    direction::Direction, entity::Entity, player::Player, tile, utils::Position, world::World,
    TILE_SIZE, WORLD_SIZE,
};
use ggez::graphics::{self, Canvas};
use std::collections::HashMap;

const PERMISSIBLE_TILES: [[f32; 4]; 5] = [
    tile::WATER,
    tile::GRASS,
    tile::PLAYER,
    tile::PROJECTILE_PLAYER,
    tile::BASIC_ENEMY,
];

pub struct Projectile {
    pub pos: Position,
    pub speed: usize,
    pub direction: Direction,
    pub color: [f32; 4],
    pub damage: usize,
    pub world_pos: Position,
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
        player_pos: Position,
    ) -> Self {
        let temp = Projectile {
            pos: Position::new(x, y),
            speed,
            damage,
            direction,
            color,
            world_pos: player_pos        
        };
        temp
    }

    pub fn update(world: &mut World) {
        let mut index: i32 = 0;
        for _ in 0..world.projectiles.len() {
            match world.projectiles[index as usize] {
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
                    if !World::travel(world, Entity::Projectile, Some(index as usize)) {
                        Projectile::kill(index as usize, world);
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
        world.entity_map[world.projectiles[index].world_pos.y][world.projectiles[index].world_pos.x].remove(&world.projectiles[index].pos);
        world.projectiles.remove(index);
    }

    pub fn can_travel_to(
        world: &mut World,
        position_info: (Position, Position) //Where .0 is the position, and .1 is the world_position
    ) -> bool {
        //Get the map on which the position is on
        let terrain_map = &world.terrain_map;
        let entity_map = &world.entity_map;
        let curr_terrain_map = &terrain_map[position_info.1.y][position_info.1.x];
        let curr_entity_map = &entity_map[position_info.1.y][position_info.1.x];
        if curr_entity_map.contains_key(&position_info.0) || curr_terrain_map.contains_key(&position_info.0) {
            if let Some(info) = curr_entity_map.get(&position_info.0) {
                if PERMISSIBLE_TILES.contains(&info.0) {
                    return true;
                }
            }
            if let Some(info) = curr_terrain_map.get(&position_info.0) {
                if PERMISSIBLE_TILES.contains(&info) {
                    return true;
                }
            }
            return false;
        }
        true
    }
}
