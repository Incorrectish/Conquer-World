#[derive(PartialEq, Eq, Clone, Copy, Debug, serde::Serialize, serde::Deserialize)] 
// Super simple direction enum, pretty self explanatory
pub enum Direction {
    North,
    South,
    East, 
    West,
}
