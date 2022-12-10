pub struct Entity2 {
    pub position: Position,
    pub color: [f32; 4],
}

impl Entity2 {
    pub fn new(x: i32, y: i32) -> Self {
        Entity2 {
            x,
            y,
        }
    }
}


pub struct Position {
    pub x: usize, 
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position {
            x,
            y,
        }
    }
}

