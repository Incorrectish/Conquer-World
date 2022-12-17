use crate::{
    direction::Direction, entity::Entity, player::Player, tile, utils::Position, world::World,
    TILE_SIZE, WORLD_SIZE,
};
use ggez::graphics::{self, Canvas};
use std::{
    collections::HashMap,
    cmp::{max, min},
};

const LIGHTNING_DAMAGE: usize = 50;
const LIGHTNING_SPEED: usize = 0;
const LIGHTNING_SIZE: i16 = 2;
const PLAYER_PROJECTILE_DAMAGE: usize = 20;
const PLAYER_PROJECTILE_SPEED: usize = 1;
const FIRE_DAMAGE_INITIAL: usize = 1;
const FIRE_DAMAGE_SECONDARY: usize = 1;
const FIRE_DAMAGE_TERTIARY: usize = 1;
const FIRE_DAMAGE_FINAL: usize = 1;
const FIRE_SPEED: usize = 1;

const PERMISSIBLE_TILES: [[f32; 4]; 5] = [
    tile::WATER,
    tile::GRASS,
    tile::PLAYER,
    tile::PROJECTILE_PLAYER,
    tile::CHASING_ENEMY,
];

#[derive(Debug)]
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
    pub fn player_projectile(
        x: usize,
        y: usize,
        direction: Direction,
        world_pos: Position,
    ) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed: PLAYER_PROJECTILE_SPEED,
            direction,
            color: tile::PROJECTILE_PLAYER,
            damage: PLAYER_PROJECTILE_DAMAGE,
            world_pos,
        }
    }

    pub fn lightning(x: usize, y: usize, world_pos: Position) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed: LIGHTNING_SPEED,
            direction: Direction::North,
            color: tile::LIGHTNING_PLACEHOLDER,
            damage: LIGHTNING_DAMAGE,
            world_pos,
        }
    }

    pub fn player_fire(x: usize, y: usize, direction: Direction, world_pos: Position) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed: FIRE_SPEED,
            direction,
            color: tile::FIRE_PLACEHOLDER,
            damage: FIRE_DAMAGE_INITIAL,
            world_pos,
        }
    }

    fn new(
        x: usize,
        y: usize,
        speed: usize,
        damage: usize,
        direction: Direction,
        color: [f32; 4],
        player_pos: Position,
    ) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed,
            damage,
            direction,
            color,
            world_pos: player_pos,
        }
    }

    pub fn update(world: &mut World) {
        let mut index: i32 = 0;
        for _ in 0..world.projectiles.len() {
            match world.projectiles[index as usize].color {
                tile::LIGHTNING_PLACEHOLDER => {
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::LIGHTNING_INITIAL;
                    world.atmosphere_map[world_pos.y][world_pos.x]
                        .insert(pos, tile::LIGHTNING_INITIAL);
                }
                tile::LIGHTNING_INITIAL => {
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::LIGHTNING_SECONDARY;
                    world.atmosphere_map[world_pos.y][world_pos.x]
                        .insert(pos, tile::LIGHTNING_SECONDARY);
                }
                tile::LIGHTNING_SECONDARY => {
                    const deltas: [i16; 3] = [0, 1, -1];
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::LIGHTNING_FINAL;
                    // basically checks the 8 around and including the projectile and turns
                    // them to their original state
                    for i in 1..=LIGHTNING_SIZE {
                        for x_delta in deltas {
                            for y_delta in deltas {
                                if pos.x < (WORLD_SIZE.0 - x_delta * i) as usize
                                    && pos.y < (WORLD_SIZE.1 - y_delta * i) as usize
                                    && pos.x as i16 >= -(x_delta * i)
                                    && pos.y as i16 >= -(y_delta * i)
                                {
                                    let new_position = Position::new(
                                        (pos.x as i16 + (x_delta * i)) as usize,
                                        (pos.y as i16 + (y_delta * i)) as usize,
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .insert(new_position, tile::LIGHTNING_FINAL);
                                    for enemy in &mut world.enemies {
                                        if enemy.pos == new_position {
                                            enemy.damage(LIGHTNING_DAMAGE);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                tile::LIGHTNING_FINAL => {
                    const deltas: [i16; 3] = [0, 1, -1];
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles.remove(index as usize);
                    // basically checks the 8 around and including the projectile and turns
                    // them to their original state
                    for i in 1..=LIGHTNING_SIZE {
                        for x_delta in deltas {
                            for y_delta in deltas {
                                if pos.x < (WORLD_SIZE.0 - x_delta * i) as usize
                                    && pos.y < (WORLD_SIZE.1 - y_delta * i) as usize
                                    && pos.x as i16 >= -(x_delta * i)
                                    && pos.y as i16 >= -(y_delta * i)
                                {
                                    let new_position = Position::new(
                                        (pos.x as i16 + (x_delta * i)) as usize,
                                        (pos.y as i16 + (y_delta * i)) as usize,
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .remove(&new_position);
                                }
                            }
                        }
                    }
                }
                tile::FIRE_PLACEHOLDER => {
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::FIRE_INITIAL;
                    world.atmosphere_map[world_pos.y][world_pos.x]
                        .insert(pos, tile::FIRE_INITIAL);
                    for enemy in &mut world.enemies {
                        if enemy.pos == pos {
                            enemy.damage(FIRE_DAMAGE_INITIAL);
                        }
                    }
                }
                tile::FIRE_INITIAL => {
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let old_pos = world.projectiles[index as usize].pos;
                    world.projectiles[index as usize].color = tile::FIRE_SECONDARY;
                    world.atmosphere_map[world_pos.y][world_pos.x].remove(&old_pos);
                    let (new_pos, new_world_pos) = World::new_position(old_pos, world.projectiles[index as usize].direction, world, world.projectiles[index as usize].speed, Entity::Projectile, Some(index as usize));
                    let mut new_positions = Vec::new();
                    const INITIAL_DELTA: usize = 1;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) = match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => {
                                    (Position::new(new_pos.x + i, new_pos.y), Position::new(max(0, new_pos.x as i32 - i as i32) as usize, new_pos.y))
                                }
                                Direction::East | Direction::West => {
                                    (Position::new(new_pos.x, new_pos.y + i), Position::new(new_pos.x, max(0, new_pos.y as i32 - i as i32) as usize))
                                }
                        };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for enemy in &mut world.enemies {
                        for new_pos in &new_positions {
                            if enemy.pos == *new_pos {
                                enemy.damage(FIRE_DAMAGE_SECONDARY);
                            }
                        }
                    }
                    for new_position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x]
                                .insert(*new_position, tile::FIRE_SECONDARY);
                    }
                }
                // TODO: Copy paste code
                tile::FIRE_SECONDARY => {
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let old_pos = world.projectiles[index as usize].pos;
                    world.projectiles[index as usize].color = tile::FIRE_TERTIARY;
                    world.atmosphere_map[world_pos.y][world_pos.x].remove(&old_pos);
                    let (new_pos, new_world_pos) = World::new_position(old_pos, world.projectiles[index as usize].direction, world, world.projectiles[index as usize].speed, Entity::Projectile, Some(index as usize));
                    let mut new_positions = Vec::new();
                    const INITIAL_DELTA: usize = 2;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) = match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => {
                                    (Position::new(new_pos.x + i, new_pos.y), Position::new(max(0, new_pos.x as i32 - i as i32) as usize, new_pos.y))
                                }
                                Direction::East | Direction::West => {
                                    (Position::new(new_pos.x, new_pos.y + i), Position::new(new_pos.x, max(0, new_pos.y as i32 - i as i32) as usize))
                                }
                        };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for enemy in &mut world.enemies {
                        for new_pos in &new_positions {
                            if enemy.pos == *new_pos {
                                enemy.damage(FIRE_DAMAGE_TERTIARY);
                            }
                        }
                    }
                    for new_position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x]
                                .insert(*new_position, tile::FIRE_TERTIARY);
                    }
                }
                tile::FIRE_TERTIARY => {
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let old_pos = world.projectiles[index as usize].pos;
                    world.projectiles[index as usize].color = tile::FIRE_FINAL;
                    world.atmosphere_map[world_pos.y][world_pos.x].remove(&old_pos);
                    let (new_pos, new_world_pos) = World::new_position(old_pos, world.projectiles[index as usize].direction, world, world.projectiles[index as usize].speed, Entity::Projectile, Some(index as usize));
                    let mut new_positions = Vec::new();
                    const INITIAL_DELTA: usize = 3;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) = match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => {
                                    (Position::new(new_pos.x + i, new_pos.y), Position::new(max(0, new_pos.x as i32 - i as i32) as usize, new_pos.y))
                                }
                                Direction::East | Direction::West => {
                                    (Position::new(new_pos.x, new_pos.y + i), Position::new(new_pos.x, max(0, new_pos.y as i32 - i as i32) as usize))
                                }
                        };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for enemy in &mut world.enemies {
                        for new_pos in &new_positions {
                            if enemy.pos == *new_pos {
                                enemy.damage(FIRE_DAMAGE_FINAL);
                            }
                        }
                    }
                    for new_position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x]
                                .insert(*new_position, tile::FIRE_FINAL);
                    }
                }
                tile::FIRE_FINAL => {
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let mut new_positions = Vec::new();
                    let new_pos = world.projectiles[index as usize].pos;
                    const INITIAL_DELTA: usize = 3;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) = match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => {
                                    (Position::new(new_pos.x + i, new_pos.y), Position::new(max(0, new_pos.x as i32 - i as i32) as usize, new_pos.y))
                                }
                                Direction::East | Direction::West => {
                                    (Position::new(new_pos.x, new_pos.y + i), Position::new(new_pos.x, max(0, new_pos.y as i32 - i as i32) as usize))
                                }
                        };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x].remove(position);
                    }
                    Projectile::kill(index as usize, world);
                }
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
        world.entity_map[world.projectiles[index].world_pos.y]
            [world.projectiles[index].world_pos.x]
            .remove(&world.projectiles[index].pos);
        world.projectiles.remove(index);
    }

    pub fn can_travel_to(
        world: &mut World,
        position_info: (Position, Position), //Where .0 is the position, and .1 is the world_position
    ) -> bool {
        //Get the map on which the position is on
        let terrain_map = &world.terrain_map;
        let entity_map = &world.entity_map;
        let curr_terrain_map = &terrain_map[position_info.1.y][position_info.1.x];
        let curr_entity_map = &entity_map[position_info.1.y][position_info.1.x];
        if curr_entity_map.contains_key(&position_info.0)
            || curr_terrain_map.contains_key(&position_info.0)
        {
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
