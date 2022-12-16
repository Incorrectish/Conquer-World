use crate::{
    direction::Direction,
    enemy::Enemy,
    entity::Entity,
    player::Player,
    projectile::Projectile,
    random,
    tile::{self, FLOOR, PLAYER, *},
    utils::Boss,
    utils::Position,
    BOARD_SIZE, TILE_SIZE, UNIVERSAL_OFFSET, WORLD_SIZE, 
};

use ggez::graphics;

use rand::rngs::ThreadRng;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use std::cmp::min;
use std::collections::HashMap;

pub const BOSS_ROOMS: [Position; 5] = [
    Position::new(1, 1),
    Position::new(1, 5),
    Position::new(3, 3),
    Position::new(5, 1),
    Position::new(5, 5),
];
pub const FINAL_BOSS_ROOM: Position = Position::new(3, 3);
const LAKES_PER_WORLD: i16 = 3;
const TOTAL_MOUNTAINS: i16 = 60;
const ENEMY_COUNT: usize = 500;

pub struct World {
    //Stores which world the player is in
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
    // pub x_offset: usize,
    // pub y_offset: usize,

    // store an instance of a player
    pub player: Player,

    // list of enemies in our world
    pub enemies: Vec<Enemy>,

    //Vector of Bosses
    pub bosses: Vec<Boss>,

    // list of all the projectiles in the world
    pub projectiles: Vec<Projectile>,

    // Hashmap of positions to colors
    pub entity_positions: HashMap<Position, ([f32; 4], Entity)>,
    pub terrain_positions: HashMap<Position, [f32; 4]>,
    pub entity_map: [[HashMap<Position, ([f32; 4], Entity)>;
        (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
        (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    pub terrain_map: [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
        (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    pub atmosphere_map: [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
        (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    pub boss_defeated: [[bool; 7]; 7],
    pub boss_lasers: Vec<(Position, [f32; 4], usize)>,
    pub boss_asteroids: Vec<(Position, [f32; 4], usize)>,
    pub bomber_explosions: [[Vec<(Position, [f32; 4])>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
        (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    pub rng: ChaCha8Rng,
}

impl World {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        let entity_positions = HashMap::new();
        let terrain_positions = HashMap::new();
        let mut entity_map: [[HashMap<Position, ([f32; 4], Entity)>;
            (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
            (BOARD_SIZE.1 / WORLD_SIZE.1) as usize] = Default::default();
        let mut terrain_map: [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
            (BOARD_SIZE.1 / WORLD_SIZE.1) as usize] = Default::default();
        let mut boss_defeated = [[false; 7]; 7];
        World::gen_boss(&mut terrain_map);
        World::gen_outer_boss_walls(&mut terrain_map);
        World::gen_mountain(&mut rng, &mut terrain_map);
        World::gen_lake(&mut rng, &mut terrain_map);
        // World::add_doors(&mut terrain_map);
        let player = Player::new();
        let starting_map = &mut entity_map[player.pos.y][player.pos.x];
        starting_map.insert(player.pos, (player.color, Entity::Player));
        let mut enemies = Vec::new();
        let mut bosses = Vec::new();
        let mut bomber_explosions: [[Vec<(Position, [f32; 4])>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
            (BOARD_SIZE.1 / WORLD_SIZE.1) as usize] = Default::default();
        World::gen_enemies(&mut rng, &mut terrain_map, &mut entity_map, &mut enemies);
        World::gen_bosses(&mut terrain_map, &mut entity_map, &mut bosses);
        World {
            world_position: Position::new(0, 0),
            top_left: (0, 0),
            bottom_right: (WORLD_SIZE.0 as usize, (WORLD_SIZE.1) as usize),
            board_top_left: (0, 0),
            board_bottom_right: (BOARD_SIZE.0 as usize, (BOARD_SIZE.1) as usize),
            player,
            enemies,
            bosses,
            projectiles: Vec::new(),
            entity_positions,
            entity_map,
            terrain_map,
            terrain_positions,
            atmosphere_map: Default::default(),
            boss_defeated,
            boss_lasers: Vec::new(),
            boss_asteroids: Vec::new(),
            bomber_explosions,
            rng,
        }
    }

    pub fn gen_enemies(
        rng: &mut ChaCha8Rng,
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
                 (BOARD_SIZE.1 / WORLD_SIZE.1) as usize],

        entity_map: &mut [[HashMap<Position, ([f32; 4], Entity)>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
                 (BOARD_SIZE.1 / WORLD_SIZE.1) as usize],

        enemies: &mut Vec<Enemy>,
    ) {
        for _ in 0..ENEMY_COUNT {
            // the loop just generates new positions until it finds an open one, and it inserts an
            // enemy there
            loop {
                // let x = random::rand_range(rng, 0, BOARD_SIZE.0); // random x coordinate
                // let y = random::rand_range(rng, 0, BOARD_SIZE.1); // random y coordinate
                let x = random::rand_range(rng, 0, WORLD_SIZE.0); // random x coordinate
                let y = random::rand_range(rng, 0, WORLD_SIZE.1); // random y coordinate
                let world_x = random::rand_range(rng, 0, BOARD_SIZE.0 / WORLD_SIZE.0) as usize;
                let world_y = random::rand_range(rng, 0, BOARD_SIZE.1 / WORLD_SIZE.1) as usize;
                // let world_x = 0;
                // let world_y = 0;
                let random_loc = Position::new(x as usize, y as usize);
                let world_map_entity = &mut entity_map[world_y][world_x];
                let world_map_terrain = &mut terrain_map[world_y][world_x];

                // if the random position is blank, then create an enemy there
                if !world_map_terrain.contains_key(&random_loc)
                    && !world_map_entity.contains_key(&random_loc)
                        // check if it is in the starting world. If it is, then make sure the x and
                        // y positions are greater than 5
                    && ((world_x, world_y) != (0, 0) || ((x > 5) && y > 5))
                {
//                    world_map_entity.insert(random_loc, (tile::CHASING_ENEMY, Entity::Enemy));
                    world_map_entity.insert(random_loc, (tile::BOMBER_ENEMY, Entity::Enemy));
//                    enemies.push(Enemy::chasing(
                    enemies.push(Enemy::bomber(
                        x as usize,
                        y as usize,
                        Position::new(world_x, world_y),
                    ));
                    break;
                }
            }
        }
    }

    pub fn gen_bosses(
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
                 (BOARD_SIZE.1 / WORLD_SIZE.1) as usize],

        entity_map: &mut [[HashMap<Position, ([f32; 4], Entity)>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
                 (BOARD_SIZE.1 / WORLD_SIZE.1) as usize],

        bosses: &mut Vec<Boss>,
    ) {
        for room_coord in BOSS_ROOMS {
            let world_map_terrain = &mut terrain_map[room_coord.y][room_coord.x];
            let x = WORLD_SIZE.0 as usize / 2 - 3;
            let y = WORLD_SIZE.1 as usize / 2 - 3;
            if room_coord == Position::new(3,3) {
                bosses.push(Boss::new(
                    x as usize,
                    y as usize,
                    tile::MAJOR_BOSS,
                    room_coord,
                    world_map_terrain,
                ));
            } else {
                bosses.push(Boss::new(
                    x as usize,
                    y as usize,
                    tile::MINOR_BOSS,
                    room_coord,
                    world_map_terrain,
                ));
            }
           
        }
    }

    //Draws the map on the top right and corner of the world
    pub fn draw_world_map(&self, canvas: &mut graphics::Canvas) {
        //Get number of cells on each x and y axis
        let mut x = BOARD_SIZE.0 as usize / WORLD_SIZE.0 as usize;
        let mut y = BOARD_SIZE.1 as usize / WORLD_SIZE.0 as usize;
        let player_indicator = [0.9, 0.1, 0.1, 1.0]; //Color of the dot on the map
        const dungeon_indicator: [f32; 4] = [0.1, 0.5, 0.1, 1.0]; //Color of boss indicators
        const boss_room_indicator: [f32; 4] = [0.1, 0.5, 0.1, 1.0]; //Color of boss indicators
        for i in 0..(x * 6 - x + 1) {
            //Calculate length and iterate that many times
            for j in 0..(y * 6 - y + 1) {
                //Calculate height and iterate that many times
                if i % 5 == 0 || i == 0 || i == x * 6 - x || //Draw the horizontal lines but keep the cells empty
                   j % 5 == 0 || j == 0 || j == y * 6 - y
                {
                    //See above comment but for vertical lines
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (i as i32 + 360) * 2 as i32,
                                (j as i32 + 2) * 2 as i32,
                                2,
                                2,
                            ))
                            .color([0.0, 0.0, 0.0, 1.0]),
                    )
                }
            }
        }

        //Draw boss room indicators
        for position in BOSS_ROOMS {
            x = 2 + (position.x as usize) * 5;
            y = 2 + (position.y as usize) * 5;
            for i in x..x + 2 {
                for j in y..y + 2 {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (i as i32 + 360) * 2 as i32,
                                (j as i32 + 2) * 2 as i32,
                                2,
                                2,
                            ))
                            .color(dungeon_indicator),
                    )
                }
            }
        }
        //Drawing the colored dot indicator
        //Get initial position at the corner of the cell
        x = 2 + (self.world_position.x as usize) * 5;
        y = 2 + (self.world_position.y as usize) * 5;

        //Make square at that specific position
        for i in x..x + 2 {
            for j in y..y + 2 {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (i as i32 + 360) * 2 as i32,
                            (j as i32 + 2) * 2 as i32,
                            2,
                            2,
                        ))
                        .color(player_indicator),
                )
            }
        }
    }

    //This function draws the whole entire world that is seen by the player
    pub fn draw(&mut self, canvas: &mut graphics::Canvas) {
        //Draw lasers if in boss room
        if BOSS_ROOMS.contains(&self.world_position) {
            for index in 0..self.bosses.len() {
                if self.bosses[index].world_position == self.world_position {
                    Boss::draw_boss_stuff(self, canvas, index);
                }
            }
        }
        Enemy::draw_bomber_explosion(self, canvas);
        
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
                        .color([1.0, 1.0, 1.0, 1.0]),
                )
            }
        }

        //Draw health and energy indicators
        self.player.draw_health(canvas);
        self.player.draw_energy(canvas);
        self.draw_world_map(canvas);

        //Draw every pixel that is contained in the terrain HashMap
        let curr_world_terrain_map =
            &self.terrain_map[self.world_position.y][self.world_position.x];
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
                    .color(Self::related_color(&mut self.rng, *color)),
            )
        }

        //Draw every pixel that is contained in the entity HashMap
        let curr_world_entity_map = &self.entity_map[self.world_position.y][self.world_position.x];

        for (loc, color) in curr_world_entity_map {
            let mut color = color.0;
            if color == tile::PLAYER {
                color = if self.player.is_visible() {
                    tile::PLAYER
                } else {
                    tile::PLAYER_INVISIBLE
                }
            }
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new_i32(
                        (loc.x as i32 * TILE_SIZE.0 as i32) as i32,
                        ((loc.y + UNIVERSAL_OFFSET as usize) as i32) * TILE_SIZE.1 as i32,
                        // (loc.x - (self.world_position.x * WORLD_SIZE.0 as usize)) as i32
                        //     * TILE_SIZE.0 as i32,
                        // (loc.y - (self.world_position.y * WORLD_SIZE.1 as usize)
                        //     + UNIVERSAL_OFFSET as usize) as i32
                        //     * TILE_SIZE.1 as i32,
                        TILE_SIZE.0 as i32,
                        TILE_SIZE.1 as i32,
                    ))
                    .color(color),
            )
        }

        //Draw every pixel that is contained in the terrain HashMap
        let curr_world_atmosphere_map =
            &self.atmosphere_map[self.world_position.y][self.world_position.x];
        for (loc, color) in curr_world_atmosphere_map {
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
    }

    // this function just returns whether a set of coordinates are within the bounds of the dynamic
    // world. takes in the world, x, and y, and returns true if the coordinates are inside the
    // world, and false otherwise
    pub fn coordinates_are_within_world(world: &mut World, world_position: Position) -> bool {
        return world_position == world.world_position;
    }

    // Returns true if coordinates inside board (note distinction from world), false otherwise
    // Distinction from coordinates_are_within_world() is important for shifting cameras when
    // crossing edge
    pub fn coordinates_are_within_board(world: &mut World, world_position: Position) -> bool {
        return world_position.x > 0
            || world_position.x < WORLD_SIZE.0 as usize / WORLD_SIZE.0 as usize
            || world_position.y > 0
            || world_position.y < WORLD_SIZE.1 as usize / WORLD_SIZE.0 as usize;
    }

    //Takes in a previous location and new location Position object and updates that specific
    //entity inside of the HashMap to move from the previous location to the new location
    pub fn update_position(
        world: &mut World,
        prev_position: Position,
        new_position_info: (Position, Position),
    ) {
        let curr_world = &mut world.entity_map[new_position_info.1.y][new_position_info.1.x];
        let info = &curr_world.get(&prev_position); //Access contents of what was at previous position
        if let Some(contents) = info {
            let tile_color = contents.0;
            let tile_type = contents.1.clone();
            curr_world.insert(new_position_info.0, (tile_color, tile_type)); //Insert same contents into new position
            curr_world.remove(&prev_position); //Remove old position
        }
    }

    //This function runs calculations and moves an entity to where ever they are meant to go
    //returns if it was successfully able to move there or not
    pub fn travel(world: &mut World, entity_type: Entity, index: Option<usize>) -> bool {
        let (pos, direction, speed, index) = match entity_type {
            //Check what type of entity is moving and match the corresponding values
            Entity::Player => (
                world.player.pos,
                world.player.direction.clone(),
                world.player.speed,
                None,
            ),
            Entity::Enemy => {
                let i = index.unwrap();
                (
                    world.enemies[i].pos,
                    world.enemies[i].direction.clone(),
                    world.enemies[i].speed,
                    Some(i),
                )
            }
            Entity::Projectile => {
                let i = index.unwrap();
                (
                    world.projectiles[i].pos,
                    world.projectiles[i].direction.clone(),
                    world.projectiles[i].speed,
                    Some(i),
                )
            }
        };

        let new_position = Self::new_position(
            pos,
            direction.clone(),
            world,
            speed,
            entity_type.clone(),
            index,
        ); //Get where the entity is supposed to go

        if !Self::coordinates_are_within_board(world, new_position.1) || new_position.0 == pos {
            //If new location is not within the board, returns false
            return false;
        } else {
            match entity_type.clone() {
                //Determine entity time again as each behaves differently
                Entity::Player => {
                    //If new position is not within world but the player can travel to it
                    //Necessitates camera shift
                    if !Self::coordinates_are_within_world(world, new_position.1)
                        && Player::can_travel_to(world, new_position)
                    {
                        let mut curr_player_map =
                            &mut world.entity_map[world.world_position.y][world.world_position.x];
                        curr_player_map.remove(&pos);
                        match direction {
                            //Shifts world_position and puts player on first tile on next screen
                            Direction::North => {
                                world.world_position =
                                    Position::new(new_position.1.x, new_position.1.y);
                                curr_player_map = &mut world.entity_map[world.world_position.y]
                                    [world.world_position.x];
                                curr_player_map.insert(
                                    Position::new(new_position.0.x, WORLD_SIZE.1 as usize - 1),
                                    (tile::PLAYER, Entity::Player),
                                );
                                world.player.pos =
                                    Position::new(new_position.0.x, WORLD_SIZE.1 as usize - 1);
                            }
                            Direction::East => {
                                world.world_position =
                                    Position::new(new_position.1.x, new_position.1.y);
                                curr_player_map = &mut world.entity_map[world.world_position.y]
                                    [world.world_position.x];
                                curr_player_map.insert(
                                    Position::new(0, new_position.0.y),
                                    (tile::PLAYER, Entity::Player),
                                );
                                world.player.pos = Position::new(0, new_position.0.y);
                            }
                            Direction::West => {
                                world.world_position =
                                    Position::new(new_position.1.x, new_position.1.y);
                                curr_player_map = &mut world.entity_map[world.world_position.y]
                                    [world.world_position.x];
                                curr_player_map.insert(
                                    Position::new(WORLD_SIZE.0 as usize - 1, new_position.0.y),
                                    (tile::PLAYER, Entity::Player),
                                );
                                world.player.pos =
                                    Position::new(WORLD_SIZE.0 as usize - 1, new_position.0.y);
                            }
                            Direction::South => {
                                world.world_position =
                                    Position::new(new_position.1.x, new_position.1.y);
                                curr_player_map = &mut world.entity_map[world.world_position.y]
                                    [world.world_position.x];
                                curr_player_map.insert(
                                    Position::new(new_position.0.x, 0),
                                    (tile::PLAYER, Entity::Player),
                                );
                                world.player.pos = Position::new(new_position.0.x, 0);
                            }
                        }
                    } else {
                        if Player::can_travel_to(world, new_position) {
                            Self::update_position(world, world.player.pos, new_position);
                            world.player.pos = new_position.0;
                        }
                    }

                    Self::toggle_doors(
                        &mut world.terrain_map,
                        world.world_position,
                        world.player.pos,
                        world.boss_defeated,
                    );
                    return true;
                }

                Entity::Enemy => {
                    //Enemy movement is in the enemy.rs file TODO: move it over here
                    return true;
                }

                Entity::Projectile => {
                    if !Projectile::can_travel_to(world, new_position)
                        && new_position.0 != pos
                        && new_position.1 != world.player.pos
                    {
                        return false;
                    }
                    let i = index.unwrap();
                    for index in 0..world.enemies.len() {
                        //Check if the projectile will hit an enemy, if so damage the enemy
                        if new_position.0 == world.enemies[index].pos
                            && new_position.1 == world.enemies[index].world_pos
                        {
                            world.enemies[index].damage(world.projectiles[i].damage);
                            return false; //Will delete the projectile that hits the enemy
                        }
                    }
                    Self::update_position(world, world.projectiles[i].pos, new_position); //Update projectile position to new position it is moving to
                    world.projectiles[i].pos = new_position.0;
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
        entity_type: Entity,
        index: Option<usize>,
    ) -> (Position, Position) {
        //Where .0 is the phyiscal coordinate position and .1 is the world_position
        let mut x = pos.x as i16;
        let mut y = pos.y as i16;
        let mut world_pos = world.world_position;
        if (entity_type == Entity::Projectile && x == 0 && direction == Direction::West) 
            || (entity_type == Entity::Projectile && y == WORLD_SIZE.0 - 1 && direction == Direction::East)
            || (entity_type == Entity::Projectile && x == 0 && direction == Direction::North)
            || (entity_type == Entity::Projectile && y == WORLD_SIZE.1 - 1 && direction == Direction::South)
        {

        }


        match direction {
            Direction::North => {
                y = y as i16 - travel_distance as i16;
                if y < 0 {
                    match entity_type {
                        Entity::Enemy => {
                            let i = index.unwrap();
                            return (world.enemies[i].pos, world.enemies[i].world_pos);
                        }
                        Entity::Projectile => {
                            let i = index.unwrap();
                            return (world.projectiles[i].pos, world.projectiles[i].world_pos);
                        }
                        _ => {
                            //If the new coordinate is negative, we know we have to shift up
                            y = WORLD_SIZE.1 - 1 as i16; //Puts coordinate at spot on next camera view
                            if world_pos.y == 0 {
                                //If we are at the edge of the board, don't shift, instead return same value
                                return (Position::new(pos.x as usize, pos.y as usize), world_pos);
                            }
                            //Shifts world
                            world_pos =
                                Position::new(world.world_position.x, world.world_position.y - 1);
                        }
                    }
                }
            }
            Direction::South => {
                //Same as North but for the South direction
                y = y as i16 + travel_distance as i16;
                if y >= WORLD_SIZE.1 as i16 {
                    match entity_type {
                        Entity::Enemy => {
                            let i = index.unwrap();
                            return (world.enemies[i].pos, world.enemies[i].world_pos);
                        }
                        Entity::Projectile => {
                            let i = index.unwrap();
                            return (world.projectiles[i].pos, world.projectiles[i].world_pos);
                        }
                        _ => {
                            y = 0;
                            if world_pos.y == BOARD_SIZE.1 as usize / WORLD_SIZE.0 as usize {
                                return (Position::new(pos.x as usize, pos.y as usize), world_pos);
                            }
                            world_pos =
                                Position::new(world.world_position.x, world.world_position.y + 1);
                        }
                    }
                }
            }
            Direction::East => {
                //Same as North but for the East direction
                x = x as i16 + travel_distance as i16;
                if x >= WORLD_SIZE.0 as i16 {
                    match entity_type {
                        Entity::Enemy => {
                            let i = index.unwrap();
                            return (world.enemies[i].pos, world.enemies[i].world_pos);
                        }
                        Entity::Projectile => {
                            let i = index.unwrap();
                            return (world.projectiles[i].pos, world.projectiles[i].world_pos);
                        }
                        _ => {
                            x = 0;
                            if world_pos.x == BOARD_SIZE.1 as usize / WORLD_SIZE.0 as usize {
                                return (Position::new(pos.x as usize, pos.y as usize), world_pos);
                            }
                            world_pos =
                                Position::new(world.world_position.x + 1, world.world_position.y);
                        }
                    }
                }
            }
            Direction::West => {
                //Same as North but for the West Direction
                x = x as i16 - travel_distance as i16;
                if x < 0 {
                    match entity_type {
                        Entity::Enemy => {
                            let i = index.unwrap();
                            return (world.enemies[i].pos, world.enemies[i].world_pos);
                        }
                        Entity::Projectile => {
                            let i = index.unwrap();
                            return (world.projectiles[i].pos, world.projectiles[i].world_pos);
                        }
                        _ => {
                            x = WORLD_SIZE.0 as i16 - 1;
                            if world_pos.x == 0 {
                                return (Position::new(pos.x as usize, pos.y as usize), world_pos);
                            }
                            world_pos =
                                Position::new(world.world_position.x - 1, world.world_position.y);
                        }
                    }
                }
            }
        }
        return (Position::new(x as usize, y as usize), world_pos);
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
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
                 (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    ) {
        // x and y of center of map
        let x: usize = (BOARD_SIZE.0 as usize) / 2 - 1;
        let y: usize = (BOARD_SIZE.1 as usize) / 2 - 1;

        // builds a 12x12 square around the center of WALL tiles
        let world_map = &mut terrain_map[(WORLD_SIZE.1 / WORLD_SIZE.0 / 2 + 1) as usize]
            [(WORLD_SIZE.0 / WORLD_SIZE.0  / 2 + 1) as usize];
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
    pub fn gen_lake(
        rng: &mut ChaCha8Rng,
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
                 (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    ) {
        for i in 0..7 {
            for j in 0..7 {
                let mut lakes_added = 0;
                while lakes_added < LAKES_PER_WORLD {
                    let x = random::rand_range(rng, 5, WORLD_SIZE.0); // random x coordinate
                    let y = random::rand_range(rng, 5, WORLD_SIZE.1); // random y coordinate

                    let mut lake: HashMap<Position, [f32; 4]> = HashMap::new();
                    Self::gen_lake_helper(
                        rng,
                        i * WORLD_SIZE.0 + x,
                        j * WORLD_SIZE.1 + y,
                        0,
                        terrain_map,
                        &mut lake,
                    ); // new lake centered at (x, y)
                    if lake.len() > 0 {
                        Self::combine_into_terrain(terrain_map, &lake);
                        lakes_added += 1;
                    }
                }
            }
        }
    }

    // Recursively generates lakes -- floodfill-esque idea around the center, but expansion is
    // limited probabilistically (probability of expansion decreases as we range further from the
    // center)
    fn gen_lake_helper(
        rng: &mut ChaCha8Rng,
        x: i16,
        y: i16,
        dist: i16,
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
                 (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
        lake: &mut HashMap<Position, [f32; 4]>,
    ) {
        let pos = Position::new(x as usize, y as usize);
        if !Self::has_adjacent_terrain(x as usize, y as usize, &terrain_map) {
            // sets curr tile to water
            let world_loc = Position::new((x / WORLD_SIZE.0) as usize, (y / WORLD_SIZE.0) as usize);

            let tile: [f32; 4];
            if BOSS_ROOMS.contains(&world_loc) {
                tile = tile::LAVA;
            } else {
                tile = tile::WATER;
            }

            if !lake.contains_key(&pos) {
                lake.insert(pos, tile);
            }
        } else {
            return;
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
                    Self::gen_lake_helper(rng, i, j, dist + 1, terrain_map, lake);
                }
            }
        }
    }

    // Gets probability of continuing to expand lake outwards
    fn prob_expand_lake(rng: &mut ChaCha8Rng, dist: i16) -> bool {
        random::bernoulli(rng, 1. - 0.15 * (dist as f32))
    }

    // adds a little variability to lake color
    pub fn related_color(rng: &mut ChaCha8Rng, color: [f32; 4]) -> [f32; 4] {
        if color == tile::WATER {
            const MAX_DIFF: f32 = 0.05;
            return [
                color[0] + random::rand_fraction(rng) * 2.0 * MAX_DIFF - MAX_DIFF,
                color[1] + random::rand_fraction(rng) * 2.0 * MAX_DIFF - MAX_DIFF,
                color[2] + random::rand_fraction(rng) * 2.0 * MAX_DIFF - MAX_DIFF,
                color[3],
            ];
        } else if color == tile::LAVA {
            const MAX_DIFF_1: f32 = 0.01;
            const MAX_DIFF_2: f32 = 0.10;
            return [
                color[0] + random::rand_fraction(rng) * 2.0 * MAX_DIFF_1 - MAX_DIFF_1,
                color[1] + random::rand_fraction(rng) * 2.0 * MAX_DIFF_2 - MAX_DIFF_2,
                color[2] + random::rand_fraction(rng) * 2.0 * MAX_DIFF_1 - MAX_DIFF_1,
                color[3],
            ];
        } else if color == tile::GRASS {
            const MAX_DIFF_1: f32 = 0.01;
            const MAX_DIFF_2: f32 = 0.10;
            return [
                color[0] + random::rand_fraction(rng) * 2.0 * MAX_DIFF_1 - MAX_DIFF_1,
                color[1] + random::rand_fraction(rng) * 2.0 * MAX_DIFF_1 - MAX_DIFF_1,
                color[2] + random::rand_fraction(rng) * 2.0 * MAX_DIFF_2 - MAX_DIFF_2,
                color[3],
            ];
        }
        return color;
    }

    //TODO: make faster, makes the game really slow rn
    fn gen_outer_boss_walls(
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
                 (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    ) {
        // the upper left corner of each mini boss room
        const UP_LEFT_CORNERS: [[i16; 2]; 5] = [
            [WORLD_SIZE.0, WORLD_SIZE.1],
            [WORLD_SIZE.0 * 5, WORLD_SIZE.1],
            [WORLD_SIZE.0 * 3, WORLD_SIZE.1 * 3],
            [WORLD_SIZE.0, WORLD_SIZE.1 * 5],
            [WORLD_SIZE.0 * 5, WORLD_SIZE.1 * 5],
        ];

        for corner in UP_LEFT_CORNERS {
            for i in 0..WORLD_SIZE.0 as usize {
                // generates a thickness 2 wall around each mini boss room square
                let mut world_map =
                    &mut terrain_map[corner[1] as usize / WORLD_SIZE.0 as usize][corner[0] as usize / WORLD_SIZE.0 as usize];
                if i as i16 != WORLD_SIZE.0 / 2 - 1 && i as i16 != WORLD_SIZE.0 / 2 {
                    let mut loc = Position::new(0, i);
                    world_map.insert(loc, tile::WALL);
                    loc = Position::new(i, 0);
                    world_map.insert(loc, tile::WALL);
                    loc = Position::new(i, WORLD_SIZE.0 as usize - 1);
                    world_map.insert(loc, tile::WALL);
                    loc = Position::new(WORLD_SIZE.0 as usize - 1, i);
                    world_map.insert(loc, tile::WALL);

                    world_map =
                        &mut terrain_map[corner[1] as usize / WORLD_SIZE.0 as usize][corner[0] as usize / WORLD_SIZE.0 as usize + 1];
                    loc = Position::new(0, i);
                    world_map.insert(loc, tile::WALL);

                    world_map =
                        &mut terrain_map[corner[1] as usize / WORLD_SIZE.0 as usize][corner[0] as usize / WORLD_SIZE.0 as usize - 1];
                    loc = Position::new(WORLD_SIZE.0 as usize - 1, i);
                    world_map.insert(loc, tile::WALL);

                    world_map =
                        &mut terrain_map[corner[1] as usize / WORLD_SIZE.0 as usize + 1][corner[0] as usize / WORLD_SIZE.0 as usize];
                    loc = Position::new(i, 0);
                    world_map.insert(loc, tile::WALL);

                    world_map =
                        &mut terrain_map[corner[1] as usize / WORLD_SIZE.0 as usize - 1][corner[0] as usize / WORLD_SIZE.0 as usize];
                    loc = Position::new(i, WORLD_SIZE.1 as usize - 1);
                    world_map.insert(loc, tile::WALL);
                }

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
                // loc = Position::new((corner[1] + i) as usize, (corner[0] - 1) as usize); terrain_positions.insert(loc, tile::WALL); loc = Position::new((corner[1] + i) as usize, (corner[0] + WORLD_SIZE.0) as usize); terrain_positions.insert(loc, tile::WALL);
            }
        }
        // in progress: creates a hole in the left wall of the upper left mini boss room
        // terrain_positions.remove(&Position::new(WORLD_SIZE.1 as usize, (WORLD_SIZE.0 + WORLD_SIZE.0 / 2) as usize));
        // terrain_positions.remove(&Position::new((WORLD_SIZE.1 - 1) as usize, (WORLD_SIZE.0 + WORLD_SIZE.0 / 2) as usize));
    }

    pub fn gen_mountain(
        rng: &mut ChaCha8Rng,
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
                 (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
    ) {
        let mut mountains_added = 0;
        while mountains_added < TOTAL_MOUNTAINS {
            let x = random::rand_range(rng, 5, BOARD_SIZE.0); // random x coordinate
            let y = random::rand_range(rng, 5, BOARD_SIZE.1); // random y coordinate

            let world_loc = Position::new((x / WORLD_SIZE.0) as usize, (y / WORLD_SIZE.0) as usize);

            if BOSS_ROOMS.contains(&world_loc) {
                continue;
            }

            let mut mountain: HashMap<Position, [f32; 4]> = HashMap::new();
            Self::gen_mountain_helper(rng, x, y, 0, terrain_map, &mut mountain); // new lake centered at (x, y)
            if mountain.len() > 0 {
                Self::combine_into_terrain(terrain_map, &mountain);
                mountains_added += 1;
            }
        }
    }

    // Recursively generates lakes -- floodfill-esque idea around the center, but expansion is
    // limited probabilistically (probability of expansion decreases as we range further from the
    // center)
    fn gen_mountain_helper(
        rng: &mut ChaCha8Rng,
        x: i16,
        y: i16,
        dist: i16,
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
                 (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
        mountain: &mut HashMap<Position, [f32; 4]>,
    ) {
        let pos = Position::new(x as usize, y as usize);
        if !Self::has_adjacent_terrain(x as usize, y as usize, &terrain_map) {
            // sets curr tile to water
            let tile: [f32; 4] = tile::MOUNTAIN[min(4, (dist + 2) / 3) as usize];

            if !mountain.contains_key(&pos) {
                mountain.insert(pos, tile);
            }
        } else {
            return;
        }

        const DIRECTIONS: [[i16; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]]; // orthogonal dirs
        for dir in DIRECTIONS {
            // for each tile in an orthogonal direction
            // With certain probability, continue expanding lake in that direction
            if Self::prob_expand_mountain(rng, dist) {
                let i = x + dir[0];
                let j = y + dir[1];
                // if in bounds, recursively call fn on adjacent tile (draws WATER at that tile)
                if i >= 0 && i < BOARD_SIZE.0 && j >= 0 && j < BOARD_SIZE.1 {
                    Self::gen_mountain_helper(rng, i, j, dist + 1, terrain_map, mountain);
                }
            }
        }
    }

    // Gets probability of continuing to expand lake outwards
    fn prob_expand_mountain(rng: &mut ChaCha8Rng, dist: i16) -> bool {
        random::bernoulli(rng, 1. - 0.10 * (dist as f32))
    }

    pub fn toggle_doors(
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
                 (BOARD_SIZE.1 / WORLD_SIZE.1) as usize],
        world_loc: Position,
        loc: Position,
        boss_defeated: [[bool; 7]; 7],
    ) {
        let positions: [[i16; 4]; 8] = [
            [1, 0, 0, WORLD_SIZE.1 / 2 - 1],
            [1, 0, 0, WORLD_SIZE.1 / 2],
            [-1, 0, WORLD_SIZE.0 - 1, WORLD_SIZE.1 / 2 - 1],
            [-1, 0, WORLD_SIZE.0 - 1, WORLD_SIZE.1 / 2],
            [0, 1, WORLD_SIZE.0 / 2 - 1, 0],
            [0, 1, WORLD_SIZE.0 / 2, 0],
            [0, -1, WORLD_SIZE.0 / 2 - 1, WORLD_SIZE.1 - 1],
            [0, -1, WORLD_SIZE.0 / 2, WORLD_SIZE.1 - 1],
        ];

        if BOSS_ROOMS.contains(&world_loc)
            && loc.x != 0
            && loc.x != WORLD_SIZE.0 as usize - 1
            && loc.y != 0
            && loc.y != WORLD_SIZE.1 as usize - 1
            && !boss_defeated[world_loc.y][world_loc.x]
        {
            for pos in positions {
                let x = pos[2] as usize;
                let y = pos[3] as usize;
                let world_x = (world_loc.x as i16 + pos[0]) as usize;
                let world_y = (world_loc.y as i16 + pos[1]) as usize;
                let wall_pos = Position::new(y, x);
                if !terrain_map[world_loc.y][world_loc.x].contains_key(&wall_pos) {
                    terrain_map[world_loc.y][world_loc.x].insert(wall_pos, tile::WALL);
                }
                if !terrain_map[world_y][world_x].contains_key(&wall_pos) {
                    terrain_map[world_y][world_x].insert(wall_pos, tile::WALL);
                }
            }
        } else if boss_defeated[world_loc.y][world_loc.x] {
            for pos in positions {
                let x = pos[2] as usize;
                let y = pos[3] as usize;
                let world_x = (world_loc.x as i16 + pos[0]) as usize;
                let world_y = (world_loc.y as i16 + pos[1]) as usize;
                let wall_pos = Position::new(y, x);
                if terrain_map[world_loc.y][world_loc.x].contains_key(&wall_pos) {
                    terrain_map[world_loc.y][world_loc.x].remove(&wall_pos);
                }
                if terrain_map[world_y][world_x].contains_key(&wall_pos) {
                    terrain_map[world_y][world_x].remove(&wall_pos);
                }
            }
//        } else if boss_defeated[1][1] && boss_defeated[1][5] && boss_defeated[5][1] && boss_defeated[5][5] {
//            for pos in positions {
//                let x = pos[2] as usize;
//                let y = pos[3] as usize;
//                let world_x = (3 + pos[0]) as usize;
//                let world_y = (3 + pos[1]) as usize;
//                let wall_pos = Position::new(y, x);
//                if terrain_map[3][3].contains_key(&wall_pos) {
//                    terrain_map[3][3].remove(&wall_pos);
//                }
//                if terrain_map[world_x][world_y].contains_key(&wall_pos) {
//                    terrain_map[world_x][world_y].remove(&wall_pos);
//                }
//            }
//        } else {
//            for pos in positions {
//                let x = pos[2] as usize;
//                let y = pos[3] as usize;
//                let world_x = (3 + pos[0]) as usize;
//                let world_y = (3 + pos[1]) as usize;
//                let wall_pos = Position::new(y, x);
//                if !terrain_map[3][3].contains_key(&wall_pos) {
//                    terrain_map[3][3].insert(wall_pos, tile::WALL);
//                }
//                if !terrain_map[world_x][world_y].contains_key(&wall_pos) {
//                    terrain_map[world_x][world_y].insert(wall_pos, tile::WALL);
//                }
//            }
        }
    }

    fn has_adjacent_terrain(
        x: usize,
        y: usize,
        terrain_map: &[[HashMap<Position, [f32; 4]>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
             (BOARD_SIZE.1 / WORLD_SIZE.1) as usize],
    ) -> bool {
        if x == 0 || x == BOARD_SIZE.0 as usize - 1 || y == 0 || y == BOARD_SIZE.1 as usize - 1 {
            return true;
        }

        let directions: [[i16; 2]; 9] = [
            [0, 0],
            [1, 0],
            [1, 1],
            [0, 1],
            [-1, 1],
            [-1, 0],
            [-1, -1],
            [0, -1],
            [1, -1],
        ];

        for dir in directions {
            let other_x = x as i16 + dir[0];
            let other_y = y as i16 + dir[1];

            let world_loc = Position::new((other_x / WORLD_SIZE.0) as usize, (other_y / WORLD_SIZE.0) as usize);
            let loc = Position::new(
                (other_x - (WORLD_SIZE.0 * world_loc.x as i16)) as usize,
                (other_y - (WORLD_SIZE.0 * world_loc.y as i16)) as usize,
            );

            if let Some(tile) = terrain_map[world_loc.y][world_loc.x].get(&loc) {
                if *tile == tile::WALL {
                    return true;
                }
            }
        }
        return false;
    }

    fn combine_into_terrain(
        terrain_map: &mut [[HashMap<Position, [f32; 4]>; (BOARD_SIZE.0 / WORLD_SIZE.0) as usize];
                 (BOARD_SIZE.1 / WORLD_SIZE.1) as usize],
        other: &HashMap<Position, [f32; 4]>,
    ) {
        for (pos, tile) in other {
            let world_loc =
                Position::new((pos.x as i16 / WORLD_SIZE.0) as usize, (pos.y as i16 / WORLD_SIZE.0) as usize);
            let loc = Position::new(
                (pos.x as i16 - (WORLD_SIZE.0 * world_loc.x as i16)) as usize,
                (pos.y as i16 - (WORLD_SIZE.0 * world_loc.y as i16)) as usize,
            );

            if !terrain_map[world_loc.y][world_loc.x].contains_key(&loc) {
                terrain_map[world_loc.y][world_loc.x].insert(loc, *tile);
            }
        }
    }
}
