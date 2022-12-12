use crate::{direction::Direction, entity::Entity, player::Player, tile, world::World, WORLD_SIZE, utils::Position, TILE_SIZE};
use ggez::graphics::{self, Canvas};
use std::{
    collections::HashMap,
};

const PERMISSIBLE_TILES: [[f32; 4]; 4] = [tile::WATER, tile::GRASS, tile::PLAYER, tile::PROJECTILE];

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
    pub fn new(x: usize, y: usize, speed: usize, damage: usize, direction: Direction) -> Self {
        let temp = Projectile {
            pos: Position::new(x, y),
            speed,
            damage,
            direction,
            color: tile::PROJECTILE
        };
        temp
    }

    pub fn update(world: &mut World) {
        let mut index = 1; 
        for random in 0..world.projectiles.len() { 
            if !World::travel(world, Entity::Projectile(index-1)) {
                Projectile::kill(index-1, world);
               //When projectile dies, whole array shifts back one, 
               //so need to account for this in order to check the next projectile  in array
               index -= 1; 
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
        terrain_positions: &HashMap<Position, [f32;4]>
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
