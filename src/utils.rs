use crate::{enemy::Enemy, tile};

pub struct Boss {
    pub position: Position,
    pub color: [f32; 4],
    pub surrounding: [Option<Enemy>; 8],
}

impl Boss {
    pub fn new(x: usize, y: usize, color: [f32; 4]) -> Self {
        let mut surrounding: [Option<Enemy>; 8] = Default::default();
        let mut index = 0;
        for i in 0..3 {
            for j in 0..3 {
                if i != 1 && j != 1 {
                    surrounding[index] = Some(Enemy::new(x+i, y+j, 1, tile::MINI_BOSS));
                }
                index += 1;
            }
        }
        Boss {
            position: Position::new(x, y),
            color,
            surrounding,
        }
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

