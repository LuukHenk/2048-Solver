use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}