use crate::{
    direction::Direction,
    enemy::{self, Enemy},
    entity::Entity,
    projectile::Projectile,
    tile,
    utils::Position,
    world::World,
    utils::Boss,
    BOARD_SIZE, TILE_SIZE, UNIVERSAL_OFFSET, WORLD_SIZE, world::BOSS_ROOMS,
};

use std::cmp::{max, min};

use ggez::graphics::{self, Canvas};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::event::VirtualKeyCode;
use std::collections::HashMap;

// Can change easily
const MAX_PLAYER_HEALTH: usize = 100;
const MAX_PLAYER_ENERGY: usize = 100;
const PLAYER_MELEE_DAMAGE: usize = 25;
const PLAYER_SLAM_DAMAGE: usize = 15;
const TELEPORTATION_COST: usize = 10;
const HEAL_COST: usize = 20;
const FIRE_COST: usize = 15;
const HEAL_ABILITY_RETURN: usize = 20;
const LIGHTNING_COST: usize = 15;
const INVISIBILITY_COST: usize = 50;

const INVISIBILITY_DURATION: usize = 7;

const MELEE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::A;

// TODO look over these values
const DIRECTION_LEFT: VirtualKeyCode = KeyCode::N;
const DIRECTION_DOWN: VirtualKeyCode = KeyCode::E;
const DIRECTION_UP: VirtualKeyCode = KeyCode::I;
const DIRECTION_RIGHT: VirtualKeyCode = KeyCode::O;
const HEAL_KEYCODE: VirtualKeyCode = KeyCode::H;
const TELEPORT_KEYCODE: VirtualKeyCode = KeyCode::T;
const LIGHTNING_KEYCODE: VirtualKeyCode = KeyCode::L;
const SLAM_KEYCODE: VirtualKeyCode = KeyCode::S;
const FLAME_KEYCODE: VirtualKeyCode = KeyCode::F;
const BUILD_KEYCODE: VirtualKeyCode = KeyCode::B;
const PROJECTILE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::Space;
const PLAYER_PROJECTILE_SPEED: usize = 1;
const PLAYER_PROJECTILE_DAMAGE: usize = 1;
const PLAYER_INITIAL_SPEED: usize = 1;
const PLAYER_INITIAL_ENERGY: usize = 100;
const PERMISSIBLE_TILES: [[f32; 4]; 1] = [tile::GRASS];
const LIGHTNING_COOLDOWN: usize = 5;
const TELEPORT_COOLDOWN: usize = 5;
const FIRE_COOLDOWN: usize = 5;
const SLAM_COOLDOWN: usize = 1;
const PROJECTILE_COOLDOWN: usize = 1;
const INVISIBILITY_COOLDOWN: usize = 25;

// This is with the covered tile model, but we could use the static/dynamic board paradighm or
// something else entirely
pub struct Player {
    // This is the position in the form (x, y)
    pub pos: Position,

    // The direction that the player is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    pub direction: Direction,

    // This controls the number of tiles a player moves in a direction in a given keypress
    pub speed: usize,

    // This is the player color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the player is represented as a solid color
    pub color: [f32; 4],

    // Stores player health: for player death and such
    health: usize,

    // planned energy, used for healing, projectiles, (teleportation?), building
    energy: usize,

    // This is the position queued by mouse clicks, used for teleportation, etc
    pub queued_position: Option<Position>,

    // duration of visibility: 0 means visible, N > 0 means invisible for N more turns
    visible: i16,


    // Cooldowns for the various abilities
    projectile_cooldown: i16,
    lightning_cooldown: i16,
    slam_cooldown: i16,
    fire_cooldown: i16,
    teleport_cooldown: i16,
}

impl Player {
    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        self.health -= damage;
    }

    pub fn visible(&self) -> bool {
        self.visible <= 0
    }

    pub fn new() -> Self {
        let temp = Self {
            pos: Position::new(0, 0),
            direction: Direction::South,
            speed: PLAYER_INITIAL_SPEED,
            color: tile::PLAYER,
            health: MAX_PLAYER_HEALTH,
            energy: PLAYER_INITIAL_ENERGY,
            queued_position: None,
            visible: 0,
            projectile_cooldown: 0,
            slam_cooldown: 0,
            fire_cooldown: 0,
            lightning_cooldown: 0,
            teleport_cooldown: 0,
        };
        temp
    }

    //Draws hearts on open space above the screen
    pub fn draw_health(&self, canvas: &mut graphics::Canvas) {
        let outline = [
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (7, 0),
            (8, 0),
            (9, 0),
            (10, 0),
            (1, 1),
            (6, 1),
            (11, 1),
            (0, 2),
            (12, 2),
            (0, 3),
            (12, 3),
            (0, 4),
            (12, 4),
            (0, 5),
            (12, 5),
            (0, 6),
            (12, 6),
            (1, 7),
            (11, 7),
            (2, 8),
            (10, 8),
            (3, 9),
            (9, 9),
            (4, 10),
            (8, 10),
            (5, 11),
            (7, 11),
            (6, 12),
        ]; //Manually input coordinates of the outline of the heart
        for i in 0..5 {
            //Draw one heart each time in the loop
            for coord in outline {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            ((coord.0) as i32 + 1) * 5 + i * 70, //x coordinate of each outline pixel from array
                            ((coord.1) as i32 + 2) * 5, //y coordinate of each outline pixel from array
                            5,
                            5,
                        ))
                        .color([0.0, 0.0, 0.0, 1.0]), //Color of outline
                )
            }
            Self::color_heart(&self, canvas, outline, i); //Color in the heart
        }
    }

    //Draws energy symbols on space above screen, works exactly the same as draw_health() except has different outline positions
    pub fn draw_energy(&self, canvas: &mut graphics::Canvas) {
        let outline = [
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
            (9, 0),
            (3, 1),
            (9, 1),
            (2, 2),
            (8, 2),
            (2, 3),
            (7, 3),
            (1, 4),
            (6, 4),
            (1, 5),
            (5, 5),
            (6, 5),
            (7, 5),
            (8, 5),
            (0, 6),
            (8, 6),
            (0, 7),
            (1, 7),
            (2, 7),
            (3, 7),
            (7, 7),
            (3, 8),
            (6, 8),
            (2, 9),
            (5, 9),
            (2, 10),
            (4, 10),
            (1, 11),
            (3, 11),
            (1, 12),
            (2, 12),
        ];
        for i in 0..5 {
            for coord in outline {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            ((coord.0) as i32 + 85) * 5 + i * 53,
                            ((coord.1) as i32 + 2) * 5,
                            5,
                            5,
                        ))
                        .color([0.0, 0.0, 0.0, 1.0]),
                )
            }
            Self::color_energy(&self, canvas, outline, i);
        }
    }

    //Colors in the hearts based on current health
    pub fn color_heart(
        &self,
        canvas: &mut graphics::Canvas,
        outline: [(usize, usize); 32],
        iteration: i32,
    ) {
        let master_heart_color: [f32; 4]; //True value for specific heart, used so half hearts can be colored correctly
        let stage4 = [0.145, 0.682, 0.745, 1.0];
        let stage3 = [0.2, 0.8, 0.2, 1.0];
        let stage2 = [1.0, 0.8, 0.1, 1.0];
        let stage1 = [1.0, 0.1, 0.1, 1.0];
        let mut health_check: i32 = self.health as i32 - (iteration as i32 * 5); //Checks what chunk you're of health you're on
        if health_check > 75 {
            master_heart_color = stage4;
            health_check -= 75;
        } else if health_check > 50 {
            master_heart_color = stage3;
            health_check -= 50;
        } else if health_check > 25 {
            master_heart_color = stage2;
            health_check -= 25;
        } else {
            master_heart_color = stage1;
        }
        if health_check > 0 {
            for i in 8..outline.len() - 2 {
                //Skip first row of outline (first 8 pixels)
                if outline[i].1 == outline[i + 1].1 {
                    //while the outline pixel and next outline pixel are on the same y axis
                    //Color the pixels inbetween each outline position (fill in the heart)
                    let mut offset = 1;
                    let mut temp_heart_color = master_heart_color; //Temp color incase it switches due to half heart
                    while outline[i].0 + offset != outline[i + 1].0 {
                        let pos = (outline[i].0 + offset, outline[i].1); //Get the position going to be colored (saves space)
                                                                         //If it is only half a full heart, only color in half (stop at x position 6)
                                                                         //However, if the color isn't red, color in the other half the color one stage down
                        if health_check >= 5
                            || (outline[i].0 + offset <= (12 / 5 * health_check) as usize
                                || master_heart_color != stage1)
                        {
                            if health_check < 5
                                && outline[i].0 + offset > (12 / 5 * health_check) as usize
                            {
                                if master_heart_color == stage4 {
                                    temp_heart_color = stage3;
                                } else if master_heart_color == stage3 {
                                    temp_heart_color = stage2;
                                } else if master_heart_color == stage2 {
                                    temp_heart_color = stage1;
                                }
                            }
                            if pos == (2, 2) || pos == (3, 2) || pos == (2, 3) {
                                //For the three white pixels :)
                                temp_heart_color = [1.0, 1.0, 1.0, 1.0];
                            }
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(graphics::Rect::new_i32(
                                        ((pos.0) as i32 + 1) * 5 + iteration * 70,
                                        ((pos.1) as i32 + 2) * 5,
                                        5,
                                        5,
                                    ))
                                    .color(temp_heart_color),
                            );
                            temp_heart_color = master_heart_color;
                        }
                        offset += 1;
                    }
                }
            }
        }
    }

    //Colors in the energies based on current energy
    //Works exactly the same as color_heart(), but instead the half energy uses half the height, not the width
    pub fn color_energy(
        &self,
        canvas: &mut graphics::Canvas,
        outline: [(usize, usize); 37],
        iteration: i32,
    ) {
        let master_energy_color: [f32; 4];
        let stage4 = [0.0, 0.1, 0.3, 1.0];
        let stage3 = [0.15, 0.2, 0.85, 1.0];
        let stage2 = [0.4, 0.45, 0.8, 1.0];
        let stage1 = [0.0, 0.6, 0.98, 1.0];
        let mut energy_check: i32 = self.energy as i32 - (iteration as i32 * 5);
        if energy_check > 75 {
            master_energy_color = stage4;
            energy_check -= 75;
        } else if energy_check > 50 {
            master_energy_color = stage3;
            energy_check -= 50;
        } else if energy_check > 25 {
            master_energy_color = stage2;
            energy_check -= 25;
        } else {
            master_energy_color = stage1;
        }
        if energy_check > 0 {
            for i in 7..outline.len() - 1 {
                if outline[i].1 == outline[i + 1].1 {
                    let mut offset = 1;
                    let mut temp_energy_color = master_energy_color;
                    if energy_check >= 5
                        || (outline[i + 1].1 >= (12 / 5 * (5 - energy_check)) as usize
                            || master_energy_color != stage1)
                    {
                        if energy_check < 5 && outline[i].1 < (12 / 5 * (5 - energy_check)) as usize
                        {
                            if master_energy_color == stage4 {
                                temp_energy_color = stage3;
                            } else if master_energy_color == stage3 {
                                temp_energy_color = stage2;
                            } else if master_energy_color == stage2 {
                                temp_energy_color = stage1;
                            }
                        }
                        while outline[i].0 + offset != outline[i + 1].0 {
                            let pos = (outline[i].0 + offset, outline[i].1);
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(graphics::Rect::new_i32(
                                        ((pos.0) as i32 + 85) * 5 + iteration * 53,
                                        ((pos.1) as i32 + 2) * 5,
                                        5,
                                        5,
                                    ))
                                    .color(temp_energy_color),
                            );
                            offset += 1;
                        }
                    }
                }
            }
        }
    }

    // eventually this should be the functionality to like shoot projectiles and stuff but for now
    // it just handles like arrow keys
    // Returns if the move should consume a turn
    pub fn use_input(key: KeyInput, world: &mut World) -> bool {
        match key.keycode {
            Some(key_pressed) => match key_pressed {
                KeyCode::Down => {
                    world.player.direction = Direction::South;
                    World::travel(world, Entity::Player, None);
                    world.player.projectile_cooldown -= 1;
                    world.player.slam_cooldown -= 1;
                    world.player.fire_cooldown -= 1;
                    world.player.lightning_cooldown -= 1;
                    world.player.teleport_cooldown -= 1;
                }
                KeyCode::Up => {
                    world.player.direction = Direction::North;
                    World::travel(world, Entity::Player, None);
                    world.player.projectile_cooldown -= 1;
                    world.player.slam_cooldown -= 1;
                    world.player.fire_cooldown -= 1;
                    world.player.lightning_cooldown -= 1;
                    world.player.teleport_cooldown -= 1;
                }
                KeyCode::Left => {
                    world.player.direction = Direction::West;
                    World::travel(world, Entity::Player, None);
                    world.player.projectile_cooldown -= 1;
                    world.player.slam_cooldown -= 1;
                    world.player.fire_cooldown -= 1;
                    world.player.lightning_cooldown -= 1;
                    world.player.teleport_cooldown -= 1;
                }
                KeyCode::Right => {
                    world.player.direction = Direction::East;
                    World::travel(world, Entity::Player, None);
                    world.player.projectile_cooldown -= 1;
                    world.player.slam_cooldown -= 1;
                    world.player.fire_cooldown -= 1;
                    world.player.lightning_cooldown -= 1;
                    world.player.teleport_cooldown -= 1;
                }

                DIRECTION_UP => {
                    world.player.direction = Direction::North;
                    return false;
                }

                DIRECTION_DOWN => {
                    world.player.direction = Direction::South;
                    return false;
                }

                DIRECTION_RIGHT => {
                    world.player.direction = Direction::East;
                    return false;
                }

                DIRECTION_LEFT => {
                    world.player.direction = Direction::West;
                    return false;
                }

                // Arbitrarily chosen for attack, can change later
                MELEE_ATTACK_KEYCODE => {
                    Player::melee_attack(world);
                    world.player.projectile_cooldown -= 1;
                    world.player.slam_cooldown -= 1;
                    world.player.fire_cooldown -= 1;
                    world.player.lightning_cooldown -= 1;
                    world.player.teleport_cooldown -= 1;
                }
                PROJECTILE_ATTACK_KEYCODE => {
                    if world.player.energy > 0 && world.player.projectile_cooldown <= 0 {
                        Player::projectile_attack(world);
                        world.player.energy -= 1;
                        world.player.projectile_cooldown = PROJECTILE_COOLDOWN as i16;
                        world.player.slam_cooldown -= 1;
                        world.player.fire_cooldown -= 1;
                        world.player.lightning_cooldown -= 1;
                        world.player.teleport_cooldown -= 1;
                    } else {
                        return false;
                    }
                }
                HEAL_KEYCODE => {
                    if world.player.energy >= HEAL_COST {
                        world.player.health += HEAL_ABILITY_RETURN;
                        world.player.energy -= HEAL_COST;
                        world.player.projectile_cooldown -= 1;
                        world.player.slam_cooldown -= 1;
                        world.player.fire_cooldown -= 1;
                        world.player.lightning_cooldown -= 1;
                        world.player.teleport_cooldown -= 1;
                    } else {
                        return false;
                    }
                }
                BUILD_KEYCODE => {
                    if world.player.energy > 2 && Player::build(world) {
                        world.player.projectile_cooldown -= 1;
                        world.player.slam_cooldown -= 1;
                        world.player.fire_cooldown -= 1;
                        world.player.lightning_cooldown -= 1;
                        world.player.teleport_cooldown -= 1;
                    } else {
                        return false;
                    }
                }
                LIGHTNING_KEYCODE => {
                    if world.player.energy >= LIGHTNING_COST && world.player.lightning_cooldown <= 0
                    {
                        Player::lightning(world);
                        world.player.projectile_cooldown -= 1;
                        world.player.slam_cooldown -= 1;
                        world.player.fire_cooldown -= 1;
                        world.player.lightning_cooldown = LIGHTNING_COOLDOWN as i16;
                        world.player.teleport_cooldown -= 1;
                    } else {
                        return false;
                    }
                }

                // TODO FINISH COSTS REFACTORING
                TELEPORT_KEYCODE => {
                    if world.player.energy >= TELEPORTATION_COST
                        && world.player.queued_position.is_some()
                        && world.player.teleport_cooldown <= 0
                    {
                        Self::teleport(world);
                        world.player.projectile_cooldown -= 1;
                        world.player.slam_cooldown -= 1;
                        world.player.fire_cooldown -= 1;
                        world.player.lightning_cooldown -= 1;
                        world.player.teleport_cooldown = TELEPORT_COOLDOWN as i16;
                    } else {
                        return false;
                    }
                }
                SLAM_KEYCODE => {
                    if world.player.slam_cooldown <= 0 {
                        Self::slam(world);
                        world.player.projectile_cooldown -= 1;
                        world.player.slam_cooldown = SLAM_COOLDOWN as i16;
                        world.player.fire_cooldown -= 1;
                        world.player.lightning_cooldown -= 1;
                        world.player.teleport_cooldown -= 1;
                    }
                }
                FLAME_KEYCODE => {
                    if world.player.slam_cooldown <= 0 {
                        Self::slam(world);
                        world.player.projectile_cooldown -= 1;
                        world.player.slam_cooldown -= 1;
                        world.player.fire_cooldown = FIRE_COOLDOWN as i16;
                        world.player.lightning_cooldown -= 1;
                        world.player.teleport_cooldown -= 1;
                    }
                }
                _ => {
                    return false;
                }
            },
            None => {
                return false;
            }
        }
        if BOSS_ROOMS.contains(&world.world_position) {
            Boss::update(world);
        }
        return true;
    }

    pub fn slam(world: &mut World) {
        // this gets the positions that should be damaged
        // let delta = match world.player.direction.clone() {
        //     Direction::East => (1, 0),
        //     Direction::West => (-1, 0),
        //     Direction::North =>(0, -1),
        //     Direction::South => (0, 1),
        // };
        const delta_xs: [i16; 4] = [0, 0, -1, 1]; 
        const delta_ys: [i16; 4] = [1, -1, 0, 0]; 

        // check all the enemies
        for enemy in &mut world.enemies {
            // it's fine if the position is out of bounds, because we aren't indexing anything
            for delta_x in delta_xs {
                for delta_y in delta_ys {
                    let position = Position::new((world.player.pos.x as i16 + delta_x) as usize, (world.player.pos.y as i16 + delta_y) as usize);
                    if enemy.pos == position {
                        enemy.damage(PLAYER_SLAM_DAMAGE);
                    }
                }
            }
        }
    }

    pub fn lightning(world: &mut World) {
        // if let Some(queued_position) = world.player.queued_position {
        //     // TODO: Damage
        //     let projectile = Projectile::new(
        //         queued_position.x,
        //         queued_position.y,
        //         0,
        //         3,
        //         Direction::North,
        //         tile::LIGHTNING_PLACEHOLDER,
        //     );
        //     world.projectiles.push(projectile)
        // }
    }

    // THIS METHOD EXPECTS A QUEUED POSITION
    pub fn teleport(world: &mut World) {
        if let Some(pos) = world.player.queued_position {
            if Player::can_travel_to(world, (pos, world.world_position)) {
                World::update_position(world, world.player.pos, (pos, world.world_position));
                world.player.pos = pos;
                world.player.energy -= TELEPORTATION_COST;
            }
        }
    }

    pub fn build(world: &mut World) -> bool {
        if let Some(pos) = world.player.queued_position {
            if (pos.x as i32 - world.player.pos.x as i32).abs() < 2
                && (pos.y as i32 - world.player.pos.y as i32).abs() < 2
            {
                // get the things to check
                let world_pos = world.world_position;
                let terrain_map = &world.terrain_map[world_pos.y][world_pos.x];
                let entity_map = &world.entity_map[world_pos.y][world_pos.x];
                let atmosphere_map = &mut world.atmosphere_map[world_pos.y][world_pos.x];

                // make sure build position has no terrain
                if !terrain_map.contains_key(&pos) {
                    // make sure there are no entities
                    if !entity_map.contains_key(&pos) {
                        // make sure the atmosphere doesn't contain anything
                        if !atmosphere_map.contains_key(&pos) {
                            atmosphere_map.insert(pos, tile::STRUCTURE);
                            world.player.energy -= 2;
                            return true;
                        } else {
                            match atmosphere_map.get(&pos).expect("This should be impossible because we checked that it contained a key before") {
                                &tile::STRUCTURE => {
                                    atmosphere_map.remove(&pos);
                                    return true;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn melee_attack(world: &mut World) {
        // gets the position that the attack will be applied to, one tile forward of the player in
        // the direction that they are facing
        let (attacking_position, _) = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            world.player.speed,
            Entity::Player,
            None,
        );

        //     // We do not know what enemies are on the tile being attacked, so we need to go through the
        //     // enemies and check if any of them are on the attacking tile, then damage them
        let world_pos = world.world_position;
        if let Some(entity) = world.entity_map[world_pos.y][world_pos.x].get(&attacking_position) {
            if entity.1 == Entity::Enemy {
                for enemy in &mut world.enemies {
                    if attacking_position == enemy.pos {
                        enemy.damage(PLAYER_MELEE_DAMAGE);
                        world.player.change_energy(2);
                    }
                }
            }
        }

        if let Some(terrain) = world.terrain_map[world_pos.y][world_pos.x].get(&attacking_position) {
            if terrain == &tile::BOSS_SURROUNDINGS {
                Boss::damage(world, PLAYER_MELEE_DAMAGE, world_pos);
            }
  
        }
        
    }

    // // This function should just spawn a projectile, the mechanics of dealing with the projectile
    // // and such should be determined by the projectile object itself
    pub fn projectile_attack(world: &mut World) {
        let projectile_spawn_pos = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            world.player.speed,
            Entity::Projectile,
            Some(world.projectiles.len()),
        );
        if projectile_spawn_pos.0 != world.player.pos
            && projectile_spawn_pos.1 == world.world_position
        {
            let projectile = Projectile::player_projectile(
                projectile_spawn_pos.0.x,
                projectile_spawn_pos.0.y,
                world.player.direction.clone(),
                world.world_position,
            );
            for index in 0..world.enemies.len() {
                //Check if it's spawning on enemy, if so damage the enenmy and not spawn a projectile
                if projectile_spawn_pos.0 == world.enemies[index].pos
                    && projectile_spawn_pos.1 == world.enemies[index].world_pos
                {
                    world.enemies[index].damage(projectile.damage);
                    return;
                }
            }
            world.entity_map[world.world_position.y][world.world_position.x].insert(
                projectile.pos,
                (tile::PROJECTILE_PLAYER, Entity::Projectile),
            );
            world.projectiles.push(projectile);
        }
    }

    pub fn can_travel_to(
        world: &mut World,
        position_info: (Position, Position), //Where .0 is the position, and .1 is the world_position
    ) -> bool {
        //Get the map on which the position is on
        let terrain_map = &world.terrain_map;
        let entity_map = &world.entity_map;
        let atmosphere_map = &world.atmosphere_map;
        let curr_terrain_map = &terrain_map[position_info.1.y][position_info.1.x];
        let curr_entity_map = &entity_map[position_info.1.y][position_info.1.x];
        let curr_atmosphere_map = &atmosphere_map[position_info.1.y][position_info.1.x];
        if curr_entity_map.contains_key(&position_info.0)
            || curr_terrain_map.contains_key(&position_info.0)
            || curr_atmosphere_map.contains_key(&position_info.0)
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

    pub fn change_energy(&mut self, delta: i32) {
        self.energy = max(0, min(self.energy as i32 + delta, MAX_PLAYER_ENERGY as i32)) as usize;
    }
}
