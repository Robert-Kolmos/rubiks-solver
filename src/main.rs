pub mod model;

use model::color::{WHITE, RED, ALL_COLORS};
use model::rubiks_cube::RubiksCube;
use model::rotation::{Rotation, Direction};
use rand::Rng;

use crate::model::rotation;

fn get_random_rotation() -> Rotation {
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range(0..12);

    let direction = match i {
        i if i % 2 == 0 => Direction::Clockwise,
        _ => Direction::CounterClockwise
    };
    Rotation { face: ALL_COLORS[i / 2], direction: direction }
}

fn main() {
    let mut solved = RubiksCube::solved();
    println!("Initial");
    println!("{}", solved);

    for _ in 0..40 {
        let rotation = get_random_rotation();
        println!("Turned {}", rotation);
        solved.turn(rotation);
    }

    println!();
    println!("{}", solved);
}

