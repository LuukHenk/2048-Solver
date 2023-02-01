use enum_display::EnumDisplay;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Copy, Clone, EnumDisplay)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}
