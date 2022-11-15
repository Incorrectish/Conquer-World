use crate::direction::Direction;

pub trait Movable {
    fn set_pos(&mut self, new_pos: (usize, usize));

    fn get_pos(&self) -> (usize, usize);

    fn get_x(&self) -> usize;

    fn get_y(&self) -> usize;

    fn get_covered_tile(&self) -> [f32; 4];
    
    fn set_covered_tile(&mut self, new_tile: [f32; 4]);

    fn get_color(&self) -> [f32; 4];

    // not working rn 
    // IMPORTANT TODO: GET THIS STUFF WORKING 
    fn get_direction(&self) -> Direction;
}
