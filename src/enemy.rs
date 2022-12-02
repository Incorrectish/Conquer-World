use crate::{direction::Direction, tile, world::World, WORLD_SIZE};

const ENEMY_HEALTH: usize = 5;
const PERMISSIBLE_TILES: [[f32; 4]; 1] = [tile::FLOOR];

// This is basically the same as the enemy for now, but I am just testing an enemy system
pub struct Enemy {
    // This is the position in the form (x, y)
    pub pos: (usize, usize),

    // The direction that the enemy is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    pub direction: Direction,

    // Just like in player controls the amount of tiles an enemy moves in one "turn"
    pub speed: usize,

    // This is the enemy color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the enemy is represented as a solid color
    color: [f32; 4],

    // Stores enemy health: for enemy death and such
    pub health: usize,
}

impl Enemy {
    pub fn new(
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
        x: usize,
        y: usize,
    ) -> Self {
        let temp = Self {
            pos: (x, y),
            direction: Direction::North,
            speed: 1,
            color: tile::ENEMY,
            health: ENEMY_HEALTH,
        };
        world[y][x] = temp.color;
        temp
    }

    // TODO: rewrite to make the travel function the same as player travel
    // pub fn travel(
    //     &mut self,
    //     world: &mut World,
    // ) {
    //     world.world[self.pos.1][self.pos.0] = self.covered_tile;
    //     match self.direction {
    //         Direction::North => {
    //             if self.pos.1 > 0 {
    //                 self.pos.1 -= 1
    //             }
    //         }
    //         Direction::South => {
    //             if self.pos.1 < (WORLD_SIZE.1 - 1) as usize {
    //                 self.pos.1 += 1
    //             }
    //         }
    //         Direction::East => {
    //             if self.pos.0 < (WORLD_SIZE.0 - 1) as usize {
    //                 self.pos.0 += 1
    //             }
    //         }
    //         Direction::West => {
    //             if self.pos.0 > 0 {
    //                 self.pos.0 -= 1
    //             }
    //         }
    //     }
    //     self.covered_tile = world.world[self.pos.1][self.pos.0];
    //     world.world[self.pos.1][self.pos.0] = self.color;
    // }

    pub fn update(world: &mut World) {
        // thinking of using a hack to remove all the enemies at the position instead because two
        // enemies cannot be on the same tile, would avoid the f32 lack of equality
        for index in (0..world.enemies.len()).rev() {
            if world.enemies[index].health <= 0 {
                Enemy::kill(world, index);
            }
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        // for now all it does is remove the tile on the world "board"
        world.world[world.enemies[index].pos.1][world.enemies[index].pos.0] =
            world.board[world.enemies[index].pos.1][world.enemies[index].pos.0];
        world.enemies.remove(index);
    }

    pub fn update_enemy(world: &mut World, index: usize) {
        let delta_pos = (world.player.pos.0 - world.enemies[index].pos.0, world.player.pos.1 - world.enemies[index].pos.1);
        
    }

    pub fn can_travel_to(tile: [f32; 4]) -> bool {
        for permissible_tile in PERMISSIBLE_TILES {
            if tile == permissible_tile {
                return true;
            }
        }
        false
    }
}
