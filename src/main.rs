pub mod model;

use model::rubiks_cube::RubiksCube;


fn main() {
    let mut solved = RubiksCube::solved();
    println!("Initial");
    println!("{}", solved);
    println!();

    let mut rng = rand::thread_rng();
    let rotations = solved.scramble(&mut rng, 40);

    for rotation in rotations {
        println!("Turned {}", rotation);
    }


    println!();
    println!("After Scramble");
    println!("{}", solved);
}

