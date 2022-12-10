pub struct Entity2 {
    pub position: Position,
    pub color: [f32; 4],
}

impl Entity2 {
    pub fn new(x: usize, y: usize, color: [f32; 4]) -> Self {
        Entity2 {
            position: Position::new(x, y),
            color,
        }
    }
}


#[derive(Eq, Hash, PartialEq)]
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

