pub const BOSS_FLOOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
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

// This is a random color, it just can't conflict with anything esle
pub const LIGHTNING: [[f32; 4]; 4] = [
    [0.414, 0.0, 0.414, 0.414],
    [0.0, 0.0, 0.5, 1.0],
    [0.0, 0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0, 1.0],
];

pub const BASIC_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const BOMBER: [f32; 4] = [0.0, 0.8, 0.3, 1.0];
pub const SHOOTING_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const KNIGHT_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const OGRE_ENEMY: [f32; 4] = [0.8, 0.3, 0.3, 1.0];
pub const MINI_BOSS: [f32; 4] = [0.25, 0.2, 0.9, 1.0];
pub const MAJOR_BOSS: [f32; 4] = [0.8, 0.3, 0.3, 1.0];

pub const PLAYER: [f32; 4] = [1.0, 1.0, 1.0, 0.5];
