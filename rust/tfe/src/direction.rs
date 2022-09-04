use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}