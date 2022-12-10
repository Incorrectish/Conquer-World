use crate::{direction::Direction, entity::Entity, player::Player, tile, world::World, WORLD_SIZE, utils::Position, TILE_SIZE};
use ggez::graphics::{self, Canvas};
use std::{
    collections::HashMap,
};

const PERMISSIBLE_TILES: [[f32; 4]; 4] = [tile::WATER, tile::GRASS, tile::PLAYER, tile::ENEMY];

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
    pub fn new(x: usize, y: usize, speed: usize, damage: usize, direction: Direction, world: &mut World) -> Self {
        let temp = Projectile {
            pos: Position::new(x, y),
            speed,
            damage,
            direction,
            color: tile::PROJECTILE
        };
        world.world[y-world.y_offset][x-world.x_offset] = temp.color;
        temp
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, world: &World) {
        let color = tile::PROJECTILE;
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new_i32(
                    (self.pos.x as i32 - world.x_offset as i32) * TILE_SIZE.0 as i32,
                    (self.pos.y as i32 - world.y_offset as i32) * TILE_SIZE.1 as i32,
                    TILE_SIZE.0 as i32,
                    TILE_SIZE.1 as i32
                ))
                .color(color),
        )
    }

    pub fn update(world: &mut World) {
        for index in (0..world.projectiles.len()).rev() {
            // if the projectile goes out of bounds, the position won't change

            // CURRENTLY THIS WON'T WORK ON IMPACTS BECAUSE PROJECTILE THIKS THAT ENEMIES/PLAYERS
            // ARE ILLEGAL TILES AND DESTROYS ITSELF. ADD TO PERMISSIBLE_TILES TO FIX THIS
            if !World::travel(world, Entity::Projectile(index)) {
                Projectile::kill(index, world);
                return;
            }

            // case for impact with player

            // case for impact with enemy

            // general projectile movement
        }
    }

    pub fn kill(index: usize, world: &mut World) {
        if World::coordinates_are_within_world(world, world.projectiles[index].pos) {
            world.world[world.projectiles[index].pos.y - world.y_offset][world.projectiles[index].pos.x - world.x_offset] =
                world.board[world.projectiles[index].pos.y][world.projectiles[index].pos.x] ;
        }
        world.projectiles.remove(index);
    }

    pub fn can_travel_to(
        tile: [f32; 4], 
        position: Position,
        entity_positions: &HashMap<Position, ([f32; 4], Option<Entity>)>
    ) -> bool {
        if entity_positions.contains_key(&position) {
            let info = entity_positions.get(&position);
            if info.is_some() {
                if(PERMISSIBLE_TILES.contains(&info.unwrap().0)) {
                    return true;
                }
            }
            return false;
        }
        true
    }
}
