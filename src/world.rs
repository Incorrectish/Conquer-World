use crate::{
    direction::Direction,
    enemy::Enemy,
    entity::Entity,
    player::Player,
    projectile::Projectile,
    random,
    tile::{self, ENEMY, FLOOR, PLAYER},
    BOARD_SIZE, WORLD_SIZE,
};

use rand::rngs::ThreadRng;
use std::cmp::{max, min};

pub struct World {
    // world to store the state of tiles in between frames
    pub world: [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],

    // board that stores the internal world
    pub board: [[[f32; 4]; BOARD_SIZE.0 as usize]; BOARD_SIZE.1 as usize],

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
}

impl World {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut board = [[tile::GRASS; BOARD_SIZE.0 as usize]; BOARD_SIZE.1 as usize];
        World::gen_boss(&mut board);
        World::gen_water(&mut rng, &mut board);
        let mut world = [[tile::GRASS; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];
        World::draw_world(&mut world, &mut board);
        let player = Player::new(&mut world);
        let enemies = vec![Enemy::new(&mut world, 10, 10)];
        World {
            world,
            board,
            top_left: (0, 0),
            bottom_right: (WORLD_SIZE.0 as usize, WORLD_SIZE.1 as usize),
            board_top_left: (0, 0),
            board_bottom_right: (BOARD_SIZE.0 as usize, BOARD_SIZE.1 as usize),
            x_offset: 0,
            y_offset: 0,
            player,
            enemies,
            projectiles: Vec::new(),
        }
    }

    // draws initial world from board (0 offsets)
    pub fn draw_world(
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
        board: &mut [[[f32; 4]; BOARD_SIZE.0 as usize]; BOARD_SIZE.1 as usize],
    ) {
        for i_coord in 0..WORLD_SIZE.0 {
            for j_coord in 0..WORLD_SIZE.1 {
                // ishan you can clean this up lol i'm still not super familiar with which
                // dimension is which
                let i = i_coord as usize;
                let j = j_coord as usize;
                world[i][j] = board[i][j];
            }
        }
    }

    // redraws entire world from board based on offsets and stuff
    // used when camera shifts (so every position needs to be redrawn)
    //  __________  ___  ____
    // /_  __/ __ \/ _ \/ __ \
    //  / / / /_/ / // / /_/ /
    // /_/  \____/____/\____/
    // TODO: ishan redraw entities too (i.e. projectiles, enemies, etc.)
    pub fn refresh_world(world: &mut World) {
        for x in 0..WORLD_SIZE.0 as usize {
            for y in 0..WORLD_SIZE.1 as usize {
                world.world[y][x] = world.board[y + world.y_offset][x + world.x_offset];
            }
        }
        // ULTRA HACK TO GET AROUND MUTABLE POINTER STUFF, CAN BE SERIOUSLY OPTIMIZED TODO
        let mut enemies_in_world = Vec::new();
        for enemy in &world.enemies {
            if enemy.pos.0 >= world.x_offset
                && enemy.pos.0 < world.x_offset + WORLD_SIZE.1 as usize
                && enemy.pos.1 >= world.y_offset
                && enemy.pos.1 < world.y_offset + WORLD_SIZE.0 as usize
            {
                enemies_in_world.push((enemy.pos, enemy.color));
            }
        }
        for enemy in enemies_in_world {
            world.world[enemy.0.1 - world.y_offset][enemy.0.0 - world.x_offset] = enemy.1; 
        }

        let mut projectiles_in_world = Vec::new();
        for projectile in &world.projectiles {
            if projectile.pos.0 >= world.x_offset
                && projectile.pos.0 < world.x_offset + WORLD_SIZE.1 as usize
                && projectile.pos.1 >= world.y_offset
                && projectile.pos.1 < world.y_offset + WORLD_SIZE.0 as usize
            {
                projectiles_in_world.push((projectile.pos, projectile.color));
            }
        }
        for projectile in projectiles_in_world {
            world.world[projectile.0.1 - world.y_offset][projectile.0.0 - world.x_offset] = projectile.1; 
        }

        world.world[world.player.pos.1 - world.y_offset][world.player.pos.0 - world.x_offset] = tile::PLAYER;
    }

    // this function just returns whether a set of coordinates are within the bounds of the dynamic
    // world. takes in the world, x, and y, and returns true if the coordinates are inside the
    // world, and false otherwise
    pub fn coordinates_are_within_world(world: &mut World, x: usize, y: usize) -> bool {
        // POTENTIAL ERRORS WITH </<=
        x >= world.x_offset
            && x < world.x_offset + WORLD_SIZE.1 as usize
            && y >= world.y_offset
            && y < world.y_offset + WORLD_SIZE.0 as usize
    }

    // Returns true if coordinates inside board (note distinction from world), false otherwise
    // Distinction from coordinates_are_within_world() is important for shifting cameras when
    // crossing edge
    pub fn coordinates_are_within_board(world: &mut World, x: usize, y: usize) -> bool {
        x < world.board_bottom_right.1
            && x >= world.board_top_left.1
            && y < world.board_bottom_right.0
            && y >= world.board_top_left.0
    }

    // this is the "move()" function but move is a reserved keyword so I just used the first
    // synonym I googled "travel()"
    pub fn travel(world: &mut World, entity_type: Entity) -> bool {
        // You need to implement the layering system in order for this to work properly, as
        // entities will no longer have covered tiles
        let (x, y, direction, speed, index) = match entity_type.clone() {
            Entity::Player => (
                world.player.pos.0,
                world.player.pos.1,
                world.player.direction.clone(),
                world.player.speed,
                None,
            ),
            Entity::Enemy(i) => (
                world.enemies[i].pos.0,
                world.enemies[i].pos.1,
                world.enemies[i].direction.clone(),
                world.enemies[i].speed,
                Some(i),
            ),
            Entity::Projectile(i) => (
                world.projectiles[i].pos.0,
                world.projectiles[i].pos.1,
                world.projectiles[i].direction.clone(),
                world.projectiles[i].speed,
                Some(i),
            ),
        };

        let new_position = Self::new_position(x, y, direction.clone(), world, speed);

        // if the new position is the same as the old position, movement is impossible and this
        // function returns false as it wasn't able to move the player or projectile, either
        // because it reached the bounds or the end of the map
        if !Self::coordinates_are_within_board(world, new_position.0, new_position.1)
            || new_position == (x, y)
            //|| (!Self::coordinates_are_within_world(world, new_position.0, new_position.1)
            //    && entity_type != Entity::Player)
            || (Self::coordinates_are_within_world(world, new_position.0, new_position.1)
                && !world.can_travel_to(entity_type.clone(), new_position.0, new_position.1))
        {
            return false;
        }
        // Coordinates are still inside board, but not world (necessitates camera shift)
        // TODO/POSSIBLE BUG: check if entity is Player and not Enemy/Projectile (haven't tested might
        // cause problems)
        else if Self::coordinates_are_within_board(world, new_position.0, new_position.1)
            && !Self::coordinates_are_within_world(world, new_position.0, new_position.1)
        {
            match entity_type {
                Entity::Player => {
                    match direction {
                        // shift based on direction of movement
                        // x_offset and y_offset explained in class definition (see above)
                        // max and min make sure that with the set of offsets the whole world is contained
                        // in the board; (this is only relevant if BOARD_SIZE is not a multiple of
                        // WORLD_SIZE)
                        Direction::North => {
                            world.y_offset = max(0, world.y_offset - WORLD_SIZE.0 as usize);
                        }
                        Direction::East => {
                            world.x_offset = min(
                                world.board_bottom_right.1 - WORLD_SIZE.1 as usize,
                                world.x_offset + WORLD_SIZE.1 as usize,
                            );
                        }
                        Direction::West => {
                            world.x_offset = max(0, world.x_offset - WORLD_SIZE.1 as usize);
                        }
                        Direction::South => {
                            world.y_offset = min(
                                world.board_bottom_right.0 - WORLD_SIZE.0 as usize,
                                world.y_offset + WORLD_SIZE.0 as usize,
                            );
                        }
                    }
                    world.player.pos = new_position;
                    Self::refresh_world(world); // refresh world for new camera angle
                    return true;
                }
                Entity::Enemy(i) => {
                    world.enemies[i].pos = new_position;
                    if Self::coordinates_are_within_world(world, x, y) {
                        world.world[y - world.y_offset][x - world.x_offset] = world.board[y][x];
                    }
                    return true;
                }
                Entity::Projectile(i) => {
                    world.projectiles[i].pos = new_position;
                    if Self::coordinates_are_within_world(world, x, y) {
                        world.world[y - world.y_offset][x - world.x_offset] = world.board[y][x];
                    }
                    return true;
                }
            }
        }
        else {
            // these conditions should only trigger if the entity type is a projectile
            if world.world[new_position.1 - world.y_offset][new_position.0 - world.x_offset]
                == tile::ENEMY
            {
                match index {
                    Some(i) => {
                        let enemy_idx = Self::get_enemy(new_position.0, new_position.1, world).unwrap();
                        world.enemies[enemy_idx].damage(world.projectiles[index.unwrap()].damage);
                    }
                    None => {
                        unreachable!("Cannot have this conditional trigger because the tile has to be a projectile")
                    }
                }
                return false;
            } else if world.world[new_position.1 - world.y_offset][new_position.0 - world.x_offset]
                == tile::PLAYER
            {
                world
                    .player
                    .damage(world.projectiles[index.unwrap()].damage);
                return false;
            } else {
                // TODO: refactor the colors to be some sort of enum
                // If the new position is a tile that can be traveled to "all black" for now, then
                // remove the player from the current tile and place it on the new tile
                // this isn't needed because the travel is checked above
                // if world.world[new_position.1][new_position.0] == FLOOR {
                // TODO: refactor to remove covered tile, layer approach created by Ishan and Michael
                // something like: dynamic[y][x] = static[y][x]?????, michael this won't work unless
                // you fix

                let original_value = world.world[y-world.y_offset][x-world.x_offset];
                world.world[new_position.1 - world.y_offset][new_position.0 - world.x_offset] =
                    original_value;
                if Self::coordinates_are_within_world(world, x, y) {
                    world.world[y - world.y_offset][x - world.x_offset] = world.board[y][x];
                    // static stuff
                }
                //

                // dynamic board doesn't exist. TODO: michael fix
                match entity_type {
                    Entity::Player => world.player.pos = new_position,
                    Entity::Enemy(i) => world.enemies[i].pos = new_position,
                    // TODO: SUPER CHANGES MUST GO ON HERE BECAUSE RIGHT NOW IT JUST GOES OVER THE ENEMY
                    // WHICH MEANS ENEMY IS ERASED FROM THE BOARD
                    Entity::Projectile(i) => world.projectiles[i].pos = new_position,
                }
                // entity.set_covered_tile(world.world[entity.get_y()][entity.get_x()]);
                // above line is unusable because of the thing
                // refactor bot

                true
            }
        }
    }

    // This method assumes that x and y are valid coordinates and does NOT check them
    fn can_travel_to(&self, entity_type: Entity, x: usize, y: usize) -> bool {
        match entity_type {
            Entity::Player => {
                Player::can_travel_to(self.world[y - self.y_offset][x - self.x_offset])
            }
            Entity::Enemy(_) => {
                Enemy::can_travel_to(self.world[y - self.y_offset][x - self.x_offset])
            }
            Entity::Projectile(_) => {
                Projectile::can_travel_to(self.world[y - self.y_offset][x - self.x_offset])
            }
        }
    }

    // This very simply gets the new position from the old, by checking the direction and the
    // bounds. Should be refactored to give a travel distance instead of just one
    pub fn new_position(
        mut x: usize,
        mut y: usize,
        direction: Direction,
        world: &mut Self,
        travel_distance: usize,
    ) -> (usize, usize) {
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
        (x, y)
    }

    pub fn get_enemy(x: usize, y: usize, world: &mut World) -> Option<usize> {
        for i in 0..world.enemies.len() {
            if world.enemies[i].pos == (x, y) {
                return Some(i);
            }
        }
        None
    }

    // generates the center boss room for map
    pub fn gen_boss(board: &mut [[[f32; 4]; BOARD_SIZE.0 as usize]; BOARD_SIZE.1 as usize]) {
        // x and y of center of map
        let x: usize = (BOARD_SIZE.0 as usize) / 2 - 1;
        let y: usize = (BOARD_SIZE.1 as usize) / 2 - 1;

        // builds a 12x12 square around the center of WALL tiles
        for i in 0..12 {
            for j in 0..12 {
                board[x - 5 + i][y - 5 + j] = tile::WALL;
            }
        }

        // builds a 4x4 square in the center of PORTAL tiles
        for i in 0..4 {
            for j in 0..4 {
                board[x - 1 + i][y - 1 + j] = tile::PORTAL;
            }
        }
    }

    // generates water tiles around the map
    pub fn gen_water(
        rng: &mut ThreadRng,
        board: &mut [[[f32; 4]; BOARD_SIZE.0 as usize]; BOARD_SIZE.1 as usize],
    ) {
        let mut lakes_added = 0;
        const TOTAL_LAKES: i16 = 12;
        while lakes_added < TOTAL_LAKES {
            let x = random::rand_range(rng, 5, BOARD_SIZE.0); // random x coordinate
            let y = random::rand_range(rng, 5, BOARD_SIZE.1); // random y coordinate

            Self::gen_lake_helper(rng, x, y, 0, board); // new lake centered at (x, y)
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
        board: &mut [[[f32; 4]; BOARD_SIZE.0 as usize]; BOARD_SIZE.1 as usize],
    ) {
        // sets curr tile to water
        if board[x as usize][y as usize] == tile::GRASS {
            board[x as usize][y as usize] = tile::WATER;
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
                    Self::gen_lake_helper(rng, i, j, dist + 1, board);
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
