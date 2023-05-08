pub mod model;

use model::color::{WHITE, RED};
use model::rubiks_cube::RubiksCube;
use model::rotation::{Rotation, Direction};


fn main() {
    let mut solved = RubiksCube::solved();
    println!("Initial");
    println!("{}", solved);

    println!("-------------------");
    println!("Turned White Clockwise");
    solved.turn(Rotation { face: &WHITE, direction: Direction::Clockwise });
    println!("{}", solved);

    println!("-------------------");
    println!("Turned Red Clockwise");
    solved.turn(Rotation { face: &RED, direction: Direction::Clockwise });
    println!("{}", solved)
}
