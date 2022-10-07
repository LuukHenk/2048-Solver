use strum_macros::EnumIter;
use enum_display::EnumDisplay;

#[derive(Debug, PartialEq, EnumIter, Copy, Clone, EnumDisplay)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None
}