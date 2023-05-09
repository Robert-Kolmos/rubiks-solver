pub mod model;

use model::rubiks_cube::RubiksCube;
use model::rotation::Rotation;


fn main() {
    let mut solved = RubiksCube::solved();
    println!("Initial");
    println!("{}", solved);

    let mut rng = rand::thread_rng();
    for _ in 0..40 {
        let rotation = Rotation::random(&mut rng);
        println!("Turned {}", rotation);
        solved.turn(rotation);
    }

    println!("After Scramble");
    println!("{}", solved);
}

