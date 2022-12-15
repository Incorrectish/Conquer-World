pub const BOSS_FLOOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
pub const FLOOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
pub const WATER: [f32; 4] = [0.145, 0.588, 0.8, 1.0];
pub const LAVA: [f32; 4] = [0.988, 0.612, 0.078, 1.0];
pub const MOUNTAIN: [[f32; 4]; 5] = [
    // [1.000, 1.000, 1.000, 1.0],
    [0.389, 0.289, 0.126, 1.0],
    [0.449, 0.373, 0.313, 1.0],
    [0.579, 0.403, 0.343, 1.0],
    [0.619, 0.463, 0.403, 1.0],
    [0.659, 0.523, 0.463, 1.0],
];
pub const PORTAL: [f32; 4] = [0.631, 0.012, 0.988, 1.0];
pub const WALL: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
pub const PROJECTILE_PLAYER: [f32; 4] = [0.5, 0.0, 0.0, 1.0];
pub const GRASS: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
pub const STRUCTURE: [f32; 4] = [0.3, 0.0, 0.0, 1.0];

// pub const LIGHTNING: [[f32; 4]; 4] = [
// This is a random color, it just can't conflict with anything esle
pub const LIGHTNING_PLACEHOLDER: [f32; 4] = [0.414, 0.0, 0.414, 0.414];
pub const LIGHTNING_INITIAL: [f32; 4] = [0.0, 0.0, 0.5, 1.0];
pub const LIGHTNING_SECONDARY: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const LIGHTNING_FINAL: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
// ];


// THIS COLOR MUST NOT CONFLICT WITH ANYTHING ELSE
pub const FIRE_PLACEHOLDER: [f32; 4] = [0.732, 0.732, 0.732, 0.732];
pub const FIRE_INITIAL: [f32; 4] = [253.0/255.0, 249.0/255.0, 212.0/255.0, 1.0];
pub const FIRE_SECONDARY: [f32; 4] = [250.0/255.0, 192.0/255.0, 0.0/255.0, 1.0];
pub const FIRE_TERTIARY: [f32; 4] = [226.0/255.0, 88.0/255.0, 34.0/255.0, 1.0];
pub const FIRE_FINAL: [f32; 4] = [215.0/255.0, 53.0/255.0, 2.0/255.0, 1.0];


pub const CHASING_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const BOMBER_ENEMY: [f32; 4] = [0.0, 0.8, 0.3, 1.0];
pub const SHOOTER_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const KNIGHT_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const MAJOR_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const MINOR_BOSS: [f32; 4] = [0.25, 0.2, 0.9, 1.0];
pub const MAJOR_BOSS: [f32; 4] = [0.8, 0.3, 0.3, 1.0];

// pub const ENEMIES: [[f32; 4]; 7] = [
//     // basic
//     [0.8, 0.3, 0.3, 1.0],
//     // bomber
//     [0.0, 0.8, 0.3, 1.0],
//     // shooter
//     [0.8, 0.3, 0.3, 1.0],
//     // knight
//     [0.8, 0.3, 0.3, 1.0],
//     // 3x3, major enemy
//     [0.8, 0.3, 0.3, 1.0],
//     // mini boss
//     [0.25, 0.2, 0.9, 1.0],
//     // major boss
//     [0.8, 0.3, 0.3, 1.0],
// ];

pub const PLAYER: [f32; 4] = [1.0, 1.0, 1.0, 0.5];
