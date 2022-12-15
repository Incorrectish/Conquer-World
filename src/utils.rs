use crate::{
    enemy::Enemy, 
    tile, 
    direction::Direction, 
    world::World,
    BOARD_SIZE, TILE_SIZE, UNIVERSAL_OFFSET, WORLD_SIZE,
    entity::Entity,
};
use std::collections::HashMap;

const BOSS_HEALTH: usize = 100;

pub struct Boss {
    pub position: Position,
    pub color: [f32; 4],
    pub surrounding: Vec<Option<Enemy>>,
    pub world_position: Position,
    pub health: usize,
}

impl Boss {
    pub fn new(x: usize, y: usize, color: [f32; 4], world_position: Position,
    entity_loc: &mut HashMap<Position, ([f32; 4], Entity)>,
    ) -> Self {

        let mut surrounding: Vec<Option<Enemy>> = Vec::new();
        let mut index = 0;
        if color == tile::MAJOR_BOSS {
            for i in 0..=6 {
                for j in 0..=6 {
                    if i == 0 || j == 0 || i == 6 || j == 6 {
                        surrounding.push(Some(Enemy::minor_boss(x+i, y+j, world_position)));
                        entity_loc.insert(Position::new(x+i, y+j), 
                        (tile::MINOR_BOSS, Entity::Enemy)
                    );
                    index += 1;
                    } else { 
                        entity_loc.insert(Position::new(x+i, y+j), 
                        (tile::MAJOR_BOSS, Entity::Enemy));
                    }
                }
            }
        } else {

        }
        Boss {
            position: Position::new(x, y),
            color,
            surrounding,
            world_position,
            health: BOSS_HEALTH,
        }
    }
    
    pub fn update(world: &mut World) {
        // for index in (0..world.bosses.len()).rev() {
        //     if world.bosses[index].health <= 0 {
        //         Enemy::kill(world, index);
        //     } else {
        //         if world.world_position == world.bosses[index].world_position {
        //             Self::move_boss(index, world);
        //         }
        //     }
        // }
    }

    pub fn move_boss(index: usize, world: &mut World) {

    }

    pub fn kill(world: &mut World, index: usize) {
        let pos = world.bosses[index].position;
        let world_pos = world.bosses[index].world_position;
        let curr_world = &mut world.entity_map[world_pos.y][world_pos.x];
        for enemy_type in &world.bosses[index].surrounding {
            if let Some(enemy) = enemy_type {
                curr_world.remove(&enemy.pos);
            }
        }
        curr_world.remove(&pos);
        world.bosses.remove(index);
        // when kill is implemented this should reopen doors
        world.boss_defeated[world.world_position.y][world.world_position.x] = true;
        World::toggle_doors(&mut world.terrain_map, world.world_position,
            world.player.pos, world.boss_defeated);
    }
}


#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]

pub struct Position {
    pub x: usize, 
    pub y: usize,
}
impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Position {
            x,
            y,
        }
    }
}

