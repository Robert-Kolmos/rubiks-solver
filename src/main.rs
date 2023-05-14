pub mod model;

use model::bit_cube::{BitCube, Move};
use model::rotation::Rotation;
use model::rubiks_cube::{RubiksCube};
use model::color::ALL_COLORS;

use std::time::Instant;

const COLOR_MAPPING: [&str; 6] = ["w", "r", "b", "o", "g", "y"];

const SIZES: [usize; 4] = [10_000, 100_000, 1_000_000, 10_000_000];

fn main() {
    test_clone();
    test_turn();
}

fn test_clone() {
    println!("Benchmarking Clone:");

    let mut bit_cube = BitCube::solved();
    let mut rubiks_cube = RubiksCube::solved();
    let mut rng = rand::thread_rng();

    for size in SIZES {
        println!("\n\nTrial {}", size);

        let rubiks_start = Instant::now();
        for _ in 0..size {
            rubiks_cube.clone();
        }
        println!("RubiksCube: {}ms", rubiks_start.elapsed().as_millis());

        let bit_start = Instant::now();
        for _ in 0..size {
            bit_cube.clone();
        }
        println!("BitCube: {}ms", bit_start.elapsed().as_millis());
    }
}

fn test_turn() {
    println!("Benchmarking Turn:");

    let mut bit_cube = BitCube::solved();
    let mut rubiks_cube = RubiksCube::solved();
    let mut rng = rand::thread_rng();

    for size in SIZES {
        println!("\n\nTrial {}", size);
            
        let mut random_moves = Vec::new();
        let mut random_rotations = Vec::new();
        for _ in 0..size {
            let random_move = Move::random(&mut rng);
            let color = ALL_COLORS[random_move.face];
            let direction = random_move.direction.clone();
            random_moves.push(random_move);
            random_rotations.push(Rotation { face: color, direction: direction });
        }

        let rubiks_start = Instant::now();
        for val in random_rotations {
            rubiks_cube.turn(&val);
        }
        println!("RubiksCube: {}ms", rubiks_start.elapsed().as_millis());
        println!("{}", rubiks_cube);

        let bit_start = Instant::now();
        for val in random_moves {
            bit_cube.turn(val.face, val.direction);
        }
        println!("BitCube: {}ms", bit_start.elapsed().as_millis());
        println!("{}", bit_cube.pretty_to_string(&COLOR_MAPPING));
    }
}
