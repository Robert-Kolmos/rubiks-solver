use std::fmt::{Display, Formatter, Error};
use std::result::Result;

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

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let direction_str= match self.direction {
            Direction::Clockwise => "",
            Direction::CounterClockwise => "'",
        };
        write!(f, "{}{}", self.face.abrv, direction_str)
    }
}