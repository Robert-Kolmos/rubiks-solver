use std::fmt::{Display, Formatter, Error};
use std::result::Result;

use rand::Rng;
use queues::{Queue, IsQueue};

use super::block::BlockFace;
use super::color::{Color, WHITE, RED, BLUE, ORANGE, GREEN, YELLOW, NUM_COLORS, ALL_COLORS};
use super::rotation::{Rotation, Direction};
use super::block::Block;

const NUM_NEIGHBORS: usize = 4;
const SIDE_LEN: usize = 3;

// Stored in the order Top, Right, Bottom, Left. 
const ADJACENT_COLORS: [[&'static Color; NUM_NEIGHBORS]; NUM_COLORS] = [
    [&GREEN, &ORANGE, &BLUE, &RED], // White
    [&WHITE, &BLUE, &YELLOW, &GREEN], // Red
    [&WHITE, &ORANGE, &YELLOW, &RED], // Blue
    [&WHITE, &GREEN, &YELLOW, &BLUE], // Orange
    [&WHITE, &RED, &YELLOW, &ORANGE], // Green
    [&BLUE, &ORANGE, &GREEN, &RED] // Yellow
];

/// Constructs and returns an array such that for two colors a and b, iff arr[a.idx] == Some(b) then
/// a rotates to b in the specified rotation. The index of the color opposite face will be None in
/// the resulting array.
fn get_color_rotations(rotation: &Rotation) -> [Option<&'static Color>; NUM_COLORS] {
    let face = rotation.face;
    let adjacent = ADJACENT_COLORS[face.idx];
    let faces_in_order: [&'static Color; NUM_NEIGHBORS] = match rotation.direction {
        Direction::Clockwise => adjacent,
        Direction::CounterClockwise => {
            let mut result = [ &WHITE; NUM_NEIGHBORS ];
            for i in 0..NUM_NEIGHBORS {
                result[i] = adjacent[NUM_NEIGHBORS - i - 1];
            }
            result
        },
    };

    let mut result = [None; NUM_COLORS];
    result[face.idx] = Some(face);
    for i in 0..faces_in_order.len() - 1 {
        result[faces_in_order[i].idx] = Some(faces_in_order[i + 1]);
    }
    result[faces_in_order[faces_in_order.len() - 1].idx] = Some(faces_in_order[0]);
    result
}

#[derive(Clone)]
pub struct RubiksCube<'a> {
    blocks: Box<[Block<'a>; 20]>
}

impl <'a> RubiksCube<'a> {
    pub fn solved() -> Self {
        const DEFAULT_FACE: BlockFace = BlockFace { color: &BLUE, face: &BLUE };
        const DEFAULT: Block = Block::Edge(DEFAULT_FACE, DEFAULT_FACE);
        let mut blocks = [ DEFAULT; 20];
        let mut idx = 0;

        for color in [&WHITE, &YELLOW] {
            let neighbors = ADJACENT_COLORS[color.idx];

            for i in 0..neighbors.len() {
                blocks[idx] = Block::solved_corner(color, neighbors[i], neighbors[(i + 1) % neighbors.len()]);
                idx += 1;
                blocks[idx] = Block::solved_edge(color, neighbors[i]);
                idx += 1;
            }
        }

        for color in [&GREEN, &BLUE] {
            let neighbors = ADJACENT_COLORS[color.idx];
            for neighbor in neighbors {
                if neighbor != &WHITE && neighbor != &YELLOW {
                    blocks[idx] = Block::solved_edge(color, neighbor);
                    idx += 1;
                }
            }
        }

        assert!(idx == 20);
        Self { blocks: Box::new(blocks) }
    }

    pub fn is_solved(&self) -> bool {
        self.blocks.iter().all(|block| block.is_solved())
    }

    pub fn solve(&self) -> Vec<Rotation> {
        let copy = self.clone();

        let mut queue = Queue::new();

        if !copy.is_solved() {
            queue.add((copy, Vec::new())).expect("");
        }

        let mut all_rotations = Vec::new();
        for color in ALL_COLORS.iter() {
            all_rotations.push(Rotation { face: color, direction: Direction::Clockwise });
            all_rotations.push(Rotation { face: color, direction: Direction::CounterClockwise });
        }

        let mut i = 0;
        while queue.size() != 0 {
            i = i + 1;
            let (next, next_acc) = queue.remove().expect("Underflow");
            
            if next.is_solved() {
                println!("Solve iterated {} times", i);
                return next_acc;
            }

            for rotation in all_rotations.iter() {
                let mut next_copy = next.clone();
                let mut next_acc_copy = next_acc.clone();

                next_copy.turn(rotation);
                next_acc_copy.push(rotation.clone());
                queue.add((next_copy, next_acc_copy)).expect("");
            }
        }

        println!("Solve iterated {} times", i);

        Vec::new()
    }

    /// Uses the provided Rng to generate random Rotations and executes them on self.
    /// Returns a Vec of the rotations that were executed in order.
    pub fn scramble(&mut self, rng: &mut impl Rng, n: usize) -> Vec<Rotation> {
        let mut result = Vec::new();
        for _ in 0..n {
            let rotation = Rotation::random(rng);
            self.turn(&rotation);
            result.push(rotation);
        }

        result
    }

    /// Executes the specified rotation
    pub fn turn(&mut self, rotation: &Rotation) {
        let face = rotation.face;
        let rotations = get_color_rotations(&rotation);

        self.blocks.iter_mut()
            .filter(|block| block.get_face(face) != None)
            .for_each(|block| match block {
                Block::Edge(ref mut a, ref mut b) => {
                    a.face = rotations[a.face.idx].unwrap();
                    b.face = rotations[b.face.idx].unwrap();
                },
                Block::Corner(ref mut a, ref mut b, ref mut c) => {
                    a.face = rotations[a.face.idx].unwrap();
                    b.face = rotations[b.face.idx].unwrap();
                    c.face = rotations[c.face.idx].unwrap();
                }
            });
    }

    /// Finds the block that resides between the faces in colors.
    fn find_edge(&self, colors: &[&Color; 2]) -> Option<&Block> {
        for block in self.blocks.iter() {
            match block {
                Block::Edge(i, j) => {
                    let matches = colors.iter()
                        .all(|color| *color == i.face || *color == j.face);
                    if matches {
                        return Some(block);
                    }
                },
                Block::Corner(_, _, _) => ()
            }
        }

        None
    }


    /// Finds the block that resides between the faces in colors.
    fn find_corner(&self, colors: &[&Color; 3]) -> Option<&Block> {
        for block in self.blocks.iter() {
            match block {
                Block::Edge(_, _) => (),
                Block::Corner(i, j, k) => {
                    let matches = colors.iter()
                        .all(|color| *color == i.face || *color == j.face || *color == k.face);
                    if matches {
                        return Some(block);
                    }
                }
            }
        }

        None
    }

    fn get_face(&self, face: &Color) -> Option<[[&str; SIDE_LEN]; SIDE_LEN]> {
        let neighbors = ADJACENT_COLORS[face.idx];
        
        let mut result = [["X"; SIDE_LEN]; SIDE_LEN];

        // Both edge and corner indexes go Top, Right, Bottom, Left
        let edge_indexes = [(0, 1), (1, 2), (2, 1), (1, 0)];
        let corner_indexes = [(0, 2), (2, 2), (2, 0), (0, 0)];
        for i in 0..NUM_NEIGHBORS {
            let edge = self.find_edge(&[face, neighbors[i]])?;
            result[edge_indexes[i].0][edge_indexes[i].1] = edge.get_face(face)?.abrv;
            let corner = self.find_corner(&[face, neighbors[i], neighbors[(i + 1) % NUM_NEIGHBORS]])?;
            result[corner_indexes[i].0][corner_indexes[i].1] = corner.get_face(face)?.abrv;
        }
        result[1][1] = face.abrv;

        Some(result)
    }
}

fn write_face_row(
    face: &[[&str; SIDE_LEN]; SIDE_LEN],
    row: usize, f: &mut Formatter<'_>
) -> Result<(), Error> {
    write!(f, "| {} {} {} |", face[row][0], face[row][1], face[row][2])
}

fn write_multiple_face_rows(
    faces: &Vec<[[&str; SIDE_LEN]; SIDE_LEN]>, 
    row: usize, f: &mut Formatter<'_>
) -> Result<(), Error> {
    for face in faces {
        write_face_row(face, row, f)?;
    }
    writeln!(f, "")?;
    Ok(())
}

fn write_single_face(
    face: &[[&str; SIDE_LEN]; SIDE_LEN], 
    left_pad: &str, 
    f: &mut Formatter<'_>
) -> Result<(), Error> {
    write!  (f, "{}", left_pad)?;
    write_face_row(face, 0, f)?;
    writeln!(f, "")?;
    write!  (f, "{}", left_pad)?;
    write_face_row(face, 1, f)?;
    writeln!(f, "")?;
    write!  (f, "{}", left_pad)?;
    write_face_row(face, 2, f)?;
    writeln!(f, "")?;
    Ok(())
}

impl <'a> Display for RubiksCube<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut faces = Vec::new();
        for color in ALL_COLORS {
            faces.push(self.get_face(color).unwrap());
        }

        let blank = "         ";
        let dash = "---------";

        writeln!(f, "{}{}", blank, dash)?;
        write_single_face(&faces[0], blank, f)?;
        writeln!(f, "{}{}{}{}", dash, dash, dash, dash)?;
        let middle_faces = vec! [faces[1], faces[2], faces[3], faces[4]];
        write_multiple_face_rows(&middle_faces, 0, f)?;
        write_multiple_face_rows(&middle_faces, 1, f)?;
        write_multiple_face_rows(&middle_faces, 2, f)?;
        writeln!(f, "{}{}{}{}", dash, dash, dash, dash)?;
        write_single_face(&faces[5], blank, f)?;
        writeln!(f, "{}{}", blank, dash)?;

        Ok(())
    }
}
