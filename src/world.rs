use crate::{
    direction::Direction,
    enemy::Enemy,
    entity::Entity,
    player::Player,
    projectile::Projectile,
    random,
    tile::{self, FLOOR, PLAYER, *},
    utils::Position,
    BOARD_SIZE, TILE_SIZE, WORLD_SIZE, UNIVERSAL_OFFSET
};

use ggez::graphics::{self, Canvas};

use rand::rngs::ThreadRng;

use std::{
    cmp::{max, min},
    collections::HashMap,
};

const TOTAL_LAKES: i16 = 50;

pub struct World {
    pub world_position: Position,

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
    pub terrain_map: [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize]; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    pub rng: ThreadRng,
}

impl World {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut entity_positions = HashMap::new();      
        let terrain_positions = HashMap::new();
        let mut terrain_map:
        [[HashMap<Position, [f32; 4]>; 
        (BOARD_SIZE.0 / WORLD_SIZE.0) as usize]; 
        (BOARD_SIZE.1 / WORLD_SIZE.1) as usize] = Default::default();  
        World::gen_boss(&mut terrain_map);
        World::gen_outer_boss_walls(&mut terrain_map); 
        World::gen_water(&mut rng, &mut terrain_map);
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
            world_position: Position::new(0,0),
            top_left: (0,0),
            bottom_right: (WORLD_SIZE.0 as usize, (WORLD_SIZE.1) as usize),
            board_top_left: (0,0),
            board_bottom_right: (BOARD_SIZE.0 as usize, (BOARD_SIZE.1) as usize),
            x_offset: 0,
            y_offset: 0,
            player,
            enemies,
            projectiles: Vec::new(),
            entity_positions,
            terrain_map,
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
                        .insert(random_position, (tile::BASIC_ENEMY, Entity::Enemy(enemies.len())));
                    enemies.push(Enemy::new(x as usize, y as usize, 1, tile::BASIC_ENEMY));
                    break;
                }
            }
        }
    }

    //Draws the map on the top right and corner of the world
    pub fn draw_world_map(&self, canvas: &mut graphics::Canvas) {
        //Get number of cells on each x and y axis
        let mut x = BOARD_SIZE.0 as usize / 50; 
        let mut y = BOARD_SIZE.1 as usize / 50;
        let player_indicator = [0.9, 0.1, 0.1, 1.0]; //Color of the dot on the map
        for i in 0..(x * 6 - x + 1) {  //Calculate length and iterate that many times
            for j in 0..(y * 6 - y + 1) { //Calculate height and iterate that many times
                if i % 5 == 0 || i == 0 || i == x * 6 - x || //Draw the horizontal lines but keep the cells empty
                   j % 5 == 0 || j == 0 || j == y * 6 - y { //See above comment but for vertical lines
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (i as i32 + 360) * 2 as i32,
                                (j as i32 + 2)* 2 as i32,
                                2,
                                2
                            ))
                            .color([1.0,1.0,1.0,1.0]),
                    )
                }
            }
        }

        //Drawing the colored dot indicator
        //Get initial (0,0) position
        x = 2 + (self.world_position.x as usize) * 5; 
        y = 2 + (self.world_position.y as usize) * 5;  

        //Make square at that specific position
        for i in x..x+2 { 
            for j in y..y+2 {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (i as i32 + 360) * 2 as i32,
                            (j as i32 + 2)* 2 as i32,
                            2,
                            2
                        ))
                        .color(player_indicator),
                )
            }
        }


    }

    //This function draws the whole entire world that is seen by the player
    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        //Draw the black bar on top that has the health/energy indicators
        for i in 0..WORLD_SIZE.0 {
            for j in 0..UNIVERSAL_OFFSET {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (i) as i32 * TILE_SIZE.0 as i32,
                            (j) as i32 * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                        ))
                        .color([0.0,0.0,0.0,1.0]),
                )
            }
        }

        //Draw health and energy indicators
        self.player.draw_health(canvas);
        self.player.draw_energy(canvas);
        self.draw_world_map(canvas);
        
        //Draw every pixel that is contained in the terrain HashMap
        let curr_world_terrain_map = &self.terrain_map[self.world_position.y][self.world_position.x];
        for (loc, color) in curr_world_terrain_map {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new_i32(
                        loc.x as i32 * TILE_SIZE.0 as i32,
                        (loc.y as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                        TILE_SIZE.0 as i32,
                        TILE_SIZE.1 as i32,
                    ))
                    .color(*color),
            )
        }
        
        //Draw every pixel that is contained in the entity HashMap
        for (loc, color) in &self.entity_positions {
            if self.y_offset <= loc.y && self.x_offset <= loc.x {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (loc.x - self.x_offset) as i32 * TILE_SIZE.0 as i32,
                            (loc.y - self.y_offset + UNIVERSAL_OFFSET as usize) as i32 * TILE_SIZE.1 as i32,
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

    //Takes in a previous location and new location Position object and updates that specific
    //entity inside of the HashMap to move from the previous location to the new location
    pub fn update_position(world: &mut World, prev_position: Position, new_position: Position) {
        let info = world.entity_positions.get(&prev_position); //Access contents of what was at previous position
        if let Some(contents) = info {
            let tile_color = contents.0;
            let tile_type = contents.1.clone();
            world
                .entity_positions
                .insert(new_position, (tile_color, tile_type)); //Insert same contents into new position
            world.entity_positions.remove(&prev_position); //Remove old position
        }
    }

    //This function runs calculations and moves an entity to where ever they are meant to go
    //returns if it was successfully able to move there or not
    pub fn travel(world: &mut World, entity_type: Entity) -> bool {
        let (pos, direction, speed, index) = match entity_type { //Check what type of entity is moving and match the corresponding values
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

        let new_position = Self::new_position(pos, direction.clone(), world, speed); //Get where the entity is supposed to go

        if !Self::coordinates_are_within_board(world, new_position) || new_position == pos { //If new location is not within the board, returns false
            return false;
        } else {
            match entity_type { //Determine entity time again as each behaves differently
                Entity::Player => {
                    if !Self::coordinates_are_within_world(world, new_position) //If new position is not within world but the player can travel to it
                        && Player::can_travel_to(                               //need to shift camera view for the user
                            world,
                            new_position,
                        )
                    {
                        match direction { //Shifts camera using x and y offsets depending on which way the player is moving
                            Direction::North => {
                                dbg!(world.player.pos.y);
                                world.y_offset = max(0, world.y_offset - WORLD_SIZE.1 as usize);
                                world.world_position = Position::new(world.world_position.x, world.world_position.y - 1);
                            }
                            Direction::East => {
                                world.x_offset = min(
                                    world.board_bottom_right.0 - WORLD_SIZE.0 as usize,
                                    world.x_offset + WORLD_SIZE.0 as usize,
                                );
                                world.world_position = Position::new(world.world_position.x + 1, world.world_position.y);

                            }
                            Direction::West => {
                                world.x_offset = max(0, world.x_offset - WORLD_SIZE.0 as usize);
                                world.world_position = Position::new(world.world_position.x - 1, world.world_position.y);
                            }
                            Direction::South => {
                                world.y_offset = min(
                                    world.board_bottom_right.0 - WORLD_SIZE.1 as usize,
                                    world.y_offset + WORLD_SIZE.1 as usize,
                                );
                                world.world_position = Position::new(world.world_position.x, world.world_position.y + 1);
                            }
                        }
                    }

                    if Player::can_travel_to( //If the player can travel to the area, update its position in the HashMap and object
                        world,
                        new_position,
                    ) {
                        Self::update_position(world, world.player.pos, new_position);
                        world.player.pos = new_position;
                    }
                    return true;
                }

                Entity::Enemy(i) => { //Enemy movement is in the enemy.rs file TODO: move it over here
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
                    for index in 0..world.enemies.len()  { //Check if the projectile will hit an enemy, if so damage the enemy 
                        if new_position == world.enemies[index].pos {
                            world.enemies[index].damage(world.projectiles[i].damage);
                            return false; //Will delete the projectile that hits the enemy
                        }
                    }
                    Self::update_position(world, world.projectiles[i].pos, new_position); //Update projectile position to new position it is moving to
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
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize]; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize]
    ) {
        // x and y of center of map
        let x: usize = (BOARD_SIZE.0 as usize) / 2 - 1;
        let y: usize = (BOARD_SIZE.1 as usize) / 2 - 1;

        // builds a 12x12 square around the center of WALL tiles
        let world_map = &mut terrain_map[(WORLD_SIZE.1 / 50 / 2 + 1) as usize][(WORLD_SIZE.0 / 50 / 2 + 1) as usize];
        for i in 0..12 {
            for j in 0..12 {
                let loc = Position::new(x - 5 + i as usize, y - 5 + j as usize);
                world_map.insert(loc, tile::WALL);
            }
        }

        // builds a 4x4 square in the center of PORTAL tiles
        for i in 0..4 {
            for j in 0..4 {
                let loc = Position::new(x - 1 + i as usize, y - 1 + j as usize);
                world_map.insert(loc, tile::PORTAL);
            }
        }
    }

    // generates water tiles around the map
    pub fn gen_water(
        rng: &mut ThreadRng,
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize]; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    ) {
        let mut lakes_added = 0;
        while lakes_added < TOTAL_LAKES {
            let x = random::rand_range(rng, 5, BOARD_SIZE.0); // random x coordinate
            let y = random::rand_range(rng, 5, BOARD_SIZE.1); // random y coordinate
            Self::gen_lake_helper(rng, x, y, 0, terrain_map); // new lake centered at (x, y)
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
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize]; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize]
    ) {
        // sets curr tile to water
        let world_loc = Position::new((x / 50) as usize, (y / 50) as usize);
        let loc = Position::new((x - (50 * world_loc.x as i16)) as usize, (y - (50 * world_loc.y as i16)) as usize);
        let world_map = &mut terrain_map[world_loc.y][world_loc.x];
        if !world_map.contains_key(&loc) {
            world_map.insert(loc, tile::WATER);
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
                    Self::gen_lake_helper(rng, i, j, dist + 1, terrain_map);
                }
            }
        }
    }

    // Gets probability of continuing to expand lake outwards
    fn prob_expand_lake(rng: &mut ThreadRng, dist: i16) -> bool {
        random::bernoulli(rng, 1. - 0.2 * (dist as f32))
    }
    //TODO: make faster, makes the game really slow rn
    fn gen_outer_boss_walls(
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize]; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize]
    ) {
        // the upper left corner of each mini boss room
        const UP_LEFT_CORNERS: [[i16; 2]; 4] = [[WORLD_SIZE.0, WORLD_SIZE.1],
                                                [WORLD_SIZE.0*5, WORLD_SIZE.1],
                                                [WORLD_SIZE.0, WORLD_SIZE.1*5],
                                                [WORLD_SIZE.0*5, WORLD_SIZE.1*5]];

        for corner in UP_LEFT_CORNERS {
            for i in 0..50 {
                // generates a thickness 2 wall around each mini boss room square
                let mut world_map = &mut terrain_map[corner[1] as usize / 50 ][corner[0] as usize / 50];
                let mut loc = Position::new(0,i);
                world_map.insert(loc, tile::WALL);
                loc = Position::new(i,0);
                world_map.insert(loc, tile::WALL);
                loc = Position::new(i,WORLD_SIZE.0 as usize - 1);
                world_map.insert(loc, tile::WALL);
                loc = Position::new(0,WORLD_SIZE.0 as usize -1);
                world_map.insert(loc, tile::WALL);

                world_map = &mut terrain_map[corner[1] as usize / 50][corner[0] as usize / 50 + 1];
                loc = Position::new(0,i);
                world_map.insert(loc, tile::WALL);

                world_map = &mut terrain_map[corner[1] as usize / 50][corner[0] as usize / 50 - 1];
                loc = Position::new(WORLD_SIZE.0 as usize - 1,i);
                world_map.insert(loc, tile::WALL);

                world_map = &mut terrain_map[corner[1] as usize / 50 + 1][corner[0] as usize / 50];
                loc = Position::new(i,0);
                world_map.insert(loc, tile::WALL);

                world_map = &mut terrain_map[corner[1] as usize / 50 - 1][corner[0] as usize / 50];
                loc = Position::new(i,WORLD_SIZE.1 as usize - 1);
                world_map.insert(loc, tile::WALL);


                // let mut loc = Position::new(corner[1] as usize, (corner[0] + i) as usize);
                // terrain_positions.insert(loc, tile::WALL);
                // loc = Position::new((corner[1] + WORLD_SIZE.1 - 1) as usize, (corner[0] + i) as usize);
                // terrain_positions.insert(loc, tile::WALL);
                // loc = Position::new((corner[1] + i) as usize, corner[0] as usize);
                // terrain_positions.insert(loc, tile::WALL);
                // loc = Position::new((corner[1] + i) as usize, (corner[0] + WORLD_SIZE.0 - 1) as usize);
                // terrain_positions.insert(loc, tile::WALL);

                // let mut loc = Position::new((corner[1] - 1) as usize, (corner[0] + i) as usize);
                // terrain_positions.insert(loc, tile::WALL);
                // loc = Position::new((corner[1] + WORLD_SIZE.1) as usize, (corner[0] + i) as usize);
                // terrain_positions.insert(loc, tile::WALL);
                // loc = Position::new((corner[1] + i) as usize, (corner[0] - 1) as usize);
                // terrain_positions.insert(loc, tile::WALL);
                // loc = Position::new((corner[1] + i) as usize, (corner[0] + WORLD_SIZE.0) as usize);
                // terrain_positions.insert(loc, tile::WALL);
            }
        }
        // in progress: creates a hole in the left wall of the upper left mini boss room
        // terrain_positions.remove(&Position::new(WORLD_SIZE.1 as usize, (WORLD_SIZE.0 + WORLD_SIZE.0 / 2) as usize));
        // terrain_positions.remove(&Position::new((WORLD_SIZE.1 - 1) as usize, (WORLD_SIZE.0 + WORLD_SIZE.0 / 2) as usize));
    }

}
