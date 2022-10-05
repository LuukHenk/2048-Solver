use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}