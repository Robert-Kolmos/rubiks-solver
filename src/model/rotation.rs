use std::fmt::{Display, Formatter, Error};
use std::result::Result;
use rand::Rng;

use super::color::{Color, NUM_COLORS, ALL_COLORS};

#[derive(Eq, PartialEq, Clone)]
pub enum Direction {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[derive(Clone)]
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

impl Rotation {
    pub fn random(rng: &mut impl Rng) -> Self {
        let i: usize = rng.gen_range(0..NUM_COLORS * 2);
    
        let direction = match i {
            i if i % 2 == 0 => Direction::Clockwise,
            _ => Direction::CounterClockwise
        };
        Rotation { face: ALL_COLORS[i / 2], direction: direction }
    }
}
