use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Hash, std::cmp::Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}