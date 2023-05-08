use super::color::Color;

#[derive(Eq, PartialEq)]
pub enum Direction {
    Clockwise = 0,
    CounterClockwise = 1,
}

pub struct Rotation {
    pub face: &'static Color,
    pub direction: Direction,
}
