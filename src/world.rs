use crate::{
    direction::Direction,
    enemy::Enemy,
    entity::Entity,
    player::Player,
    projectile::Projectile,
    random,
    tile::{self, ENEMY, FLOOR, PLAYER},
    utils::Position,
    BOARD_SIZE, TILE_SIZE, WORLD_SIZE,
};

use ggez::graphics::{self, Canvas};

use rand::rngs::ThreadRng;

use std::{
    cmp::{max, min},
    collections::HashMap,
};

const TOTAL_LAKES: i16 = 12;

pub struct World {

    // stores the bottom left and top right coordinates of the currently rendered world, useful for
    // querying whether a coordinate is in the current world
    pub top_left: (usize, usize),
    pub bottom_right: (usize, usize),

    // same as above, but for the board instead of world
    pub board_top_left: (usize, usize),
    pub board_bottom_right: (usize, usize),

    // offset in x and y direction for world
    // for example, if x_offset = 25 and y_offset = 10, board will span from
    // 25 <= x < 25 + WORLD_SIZE.0 and 10 <= y < 10 + WORLD_SIZE.1
    pub x_offset: usize,
    pub y_offset: usize,

    // store an instance of a player
    pub player: Player,

    // list of enemies in our world
    pub enemies: Vec<Enemy>,

    // list of all the projectiles in the world
    pub projectiles: Vec<Projectile>,

    // Hashmap of positions to colors
    pub entity_positions: HashMap<Position, ([f32; 4], Entity)>,
    pub terrain_positions: HashMap<Position, [f32; 4]>,

    pub rng: ThreadRng,
}

impl World {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut entity_positions = HashMap::new();
        let mut terrain_positions = HashMap::new();
        World::gen_water(&mut rng, &mut terrain_positions);
        World::gen_boss(&mut terrain_positions);
        let player = Player::new();
        entity_positions.insert(player.pos, (player.color, (Entity::Player)));
        let mut enemies = Vec::new();
        World::gen_enemies(
            &mut rng,
            &terrain_positions,
            &mut entity_positions,
            &mut enemies,
        );
        World {
            top_left: (0, 0),
            bottom_right: (WORLD_SIZE.0 as usize, WORLD_SIZE.1 as usize),
            board_top_left: (0, 0),
            board_bottom_right: (BOARD_SIZE.0 as usize, BOARD_SIZE.1 as usize),
            x_offset: 0,
            y_offset: 0,
            player,
            enemies,
            projectiles: Vec::new(),
            entity_positions,
            terrain_positions,
            rng,
        }
    }

    pub fn gen_enemies(
        rng: &mut ThreadRng,
        terrain_positions: &HashMap<Position, [f32; 4]>,
        entity_positions: &mut HashMap<Position, ([f32; 4], Entity)>,
        enemies: &mut Vec<Enemy>,
    ) {
        for _ in 0..10 {
            // the loop just generates new positions until it finds an open one, and it inserts an
            // enemy there
            loop {
                let x = random::rand_range(rng, 0, BOARD_SIZE.0); // random x coordinate
                let y = random::rand_range(rng, 0, BOARD_SIZE.1); // random y coordinate
                let random_position = Position::new(x as usize, y as usize);

                // if the random position is blank, then create an enemy there
                if !terrain_positions.contains_key(&random_position)
                    && !entity_positions.contains_key(&random_position)
                {
                    entity_positions
                        .insert(random_position, (tile::ENEMY, Entity::Enemy(enemies.len())));
                    enemies.push(Enemy::new(x as usize, y as usize, 1));
                    break;
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        for (loc, color) in &self.terrain_positions {
            if self.y_offset <= loc.y && self.x_offset <= loc.x {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (loc.x - self.x_offset) as i32 * TILE_SIZE.0 as i32,
                            (loc.y - self.y_offset) as i32 * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                        ))
                        .color(*color),
                )
            }
        }

        for (loc, color) in &self.entity_positions {
            if self.y_offset <= loc.y && self.x_offset <= loc.x {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (loc.x - self.x_offset) as i32 * TILE_SIZE.0 as i32,
                            (loc.y - self.y_offset) as i32 * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                        ))
                        .color(color.0),
                )
            }
        }
    }

    // this function just returns whether a set of coordinates are within the bounds of the dynamic
    // world. takes in the world, x, and y, and returns true if the coordinates are inside the
    // world, and false otherwise
    pub fn coordinates_are_within_world(world: &mut World, position: Position) -> bool {
        // POTENTIAL ERRORS WITH </<=
        position.x >= world.x_offset
            && position.x < world.x_offset + WORLD_SIZE.0 as usize
            && position.y >= world.y_offset
            && position.y < world.y_offset + WORLD_SIZE.1 as usize
    }

    // Returns true if coordinates inside board (note distinction from world), false otherwise
    // Distinction from coordinates_are_within_world() is important for shifting cameras when
    // crossing edge
    pub fn coordinates_are_within_board(world: &mut World, position: Position) -> bool {
        position.x < world.board_bottom_right.0
            && position.x >= world.board_top_left.0
            && position.y < world.board_bottom_right.1
            && position.y >= world.board_top_left.1
    }

    pub fn update_position(world: &mut World, prev_position: Position, new_position: Position) {
        let info = world.entity_positions.get(&prev_position);
        if let Some(contents) = info {
            let tile_color = contents.0;
            let tile_type = contents.1.clone();
            world
                .entity_positions
                .insert(new_position, (tile_color, tile_type));
            world.entity_positions.remove(&prev_position);
        }
    }

    pub fn travel(world: &mut World, entity_type: Entity) -> bool {
        let (pos, direction, speed, index) = match entity_type.clone() {
            Entity::Player => (
                world.player.pos,
                world.player.direction.clone(),
                world.player.speed,
                None,
            ),
            Entity::Enemy(i) => (
                world.enemies[i].pos,
                world.enemies[i].direction.clone(),
                world.enemies[i].speed,
                Some(i),
            ),
            Entity::Projectile(i) => (
                world.projectiles[i].pos,
                world.projectiles[i].direction.clone(),
                world.projectiles[i].speed,
                Some(i),
            ),
        };

        let new_position = Self::new_position(pos, direction.clone(), world, speed);

        if !Self::coordinates_are_within_board(world, new_position) || new_position == pos {
            return false;
        } else {
            match entity_type {
                Entity::Player => {
                    if !Self::coordinates_are_within_world(world, new_position)
                        && Player::can_travel_to(
                            new_position,
                            &world.entity_positions,
                            &world.terrain_positions,
                        )
                    {
                        match direction {
                            Direction::North => {
                                world.y_offset = max(0, world.y_offset - WORLD_SIZE.1 as usize);
                            }
                            Direction::East => {
                                world.x_offset = min(
                                    world.board_bottom_right.0 - WORLD_SIZE.0 as usize,
                                    world.x_offset + WORLD_SIZE.0 as usize,
                                );
                            }
                            Direction::West => {
                                world.x_offset = max(0, world.x_offset - WORLD_SIZE.0 as usize);
                            }
                            Direction::South => {
                                world.y_offset = min(
                                    world.board_bottom_right.0 - WORLD_SIZE.1 as usize,
                                    world.y_offset + WORLD_SIZE.1 as usize,
                                );
                            }
                        }
                    }
                    if Player::can_travel_to(
                        new_position,
                        &world.entity_positions,
                        &world.terrain_positions,
                    ) {
                        Self::update_position(world, world.player.pos, new_position);
                        world.player.pos = new_position;
                    }
                    return true;
                }

                Entity::Enemy(i) => {
                    return true;
                }

                Entity::Projectile(i) => {
                    if !Projectile::can_travel_to(
                        new_position,
                        &world.entity_positions,
                        &world.terrain_positions,
                    ) {
                        return false;
                    }
                    Self::update_position(world, world.projectiles[i].pos, new_position);
                    world.projectiles[i].pos = new_position;
                    true
                }
            }
        }
    }


    // This method assumes that x and y are valid coordinates and does NOT check them

    // This very simply gets the new position from the old, by checking the direction and the
    // bounds. Should be refactored to give a travel distance instead of just one
    pub fn new_position(
        pos: Position,
        direction: Direction,
        world: &mut Self,
        travel_distance: usize,
    ) -> Position {
        let mut x = pos.x;
        let mut y = pos.y;
        match direction {
            Direction::North => {
                // may be a bug in here because I can't math TODO: verify
                // we want to go as far up until we hit the bounds of the "world"
                y = max(
                    y as i16 - travel_distance as i16,
                    world.board_top_left.1 as i16,
                ) as usize;
            }
            Direction::South => {
                y = min(
                    y as i16 + travel_distance as i16,
                    world.board_bottom_right.1 as i16,
                ) as usize;
            }
            Direction::East => {
                x = min(
                    x as i16 + travel_distance as i16,
                    world.board_bottom_right.0 as i16,
                ) as usize;
            }
            Direction::West => {
                x = max(
                    x as i16 - travel_distance as i16,
                    world.board_top_left.0 as i16,
                ) as usize;
            }
        }
        return Position::new(x, y);
    }

    pub fn get_enemy(position: Position, world: &mut World) -> Option<usize> {
        for i in 0..world.enemies.len() {
            if world.enemies[i].pos == position {
                return Some(i);
            }
        }
        None
    }

    // generates the center boss room for map
    pub fn gen_boss(
        terrain_positions: &mut HashMap<Position, [f32; 4]>,
    ) {
        // x and y of center of map
        let x: usize = (BOARD_SIZE.0 as usize) / 2 - 1;
        let y: usize = (BOARD_SIZE.1 as usize) / 2 - 1;

        // builds a 12x12 square around the center of WALL tiles
        for i in 0..12 {
            for j in 0..12 {
                let loc = Position::new(x - 5 + i as usize, y - 5 + j as usize);
                terrain_positions.insert(loc, tile::WALL);
            }
        }

        // builds a 4x4 square in the center of PORTAL tiles
        for i in 0..4 {
            for j in 0..4 {
                let loc = Position::new(x - 1 + i as usize, y - 1 + j as usize);
                terrain_positions.insert(loc, tile::PORTAL);
            }
        }
    }

    // generates water tiles around the map
    pub fn gen_water(
        rng: &mut ThreadRng,
        terrain_positions: &mut HashMap<Position, [f32; 4]>,
    ) {
        let mut lakes_added = 0;
        while lakes_added < TOTAL_LAKES {
            let x = random::rand_range(rng, 5, BOARD_SIZE.0); // random x coordinate
            let y = random::rand_range(rng, 5, BOARD_SIZE.1); // random y coordinate
            Self::gen_lake_helper(rng, x, y, 0, terrain_positions); // new lake centered at (x, y)
            lakes_added += 1;
        }
    }

    // Recursively generates lakes -- floodfill-esque idea around the center, but expansion is
    // limited probabilistically (probability of expansion decreases as we range further from the
    // center)
    fn gen_lake_helper(
        rng: &mut ThreadRng,
        x: i16,
        y: i16,
        dist: i16,
        terrain_positions: &mut HashMap<Position, [f32; 4]>,
    ) {
        // sets curr tile to water
        let loc = Position::new(x as usize, y as usize);
        if !terrain_positions.contains_key(&loc) {
            terrain_positions.insert(loc, tile::WATER);
        }

        const DIRECTIONS: [[i16; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]]; // orthogonal dirs
        for dir in DIRECTIONS {
            // for each tile in an orthogonal direction
            // With certain probability, continue expanding lake in that direction
            if Self::prob_expand_lake(rng, dist) {
                let i = x + dir[0];
                let j = y + dir[1];
                // if in bounds, recursively call fn on adjacent tile (draws WATER at that tile)
                if i >= 0 && i < BOARD_SIZE.0 && j >= 0 && j < BOARD_SIZE.1 {
                    Self::gen_lake_helper(rng, i, j, dist + 1, terrain_positions);
                }
            }
        }
    }

    // Gets probability of continuing to expand lake outwards
    fn prob_expand_lake(rng: &mut ThreadRng, dist: i16) -> bool {
        random::bernoulli(rng, 1. - 0.2 * (dist as f32))
    }

    // checks to see if there is an adjacent lake with 1 space of padding i.e.
    fn check_adjacent_lake(x: i16, y: i16, world: &mut World) {}
}
