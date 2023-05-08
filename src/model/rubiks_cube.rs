use std::collections::{HashSet};
use std::fmt::{Display, Formatter, Error};
use std::result::Result;

use super::color::{Color, WHITE, RED, BLUE, ORANGE, GREEN, YELLOW, NUM_COLORS, ALL_COLORS};
use super::rotation::{Rotation, Direction};
use super::block::{Block, BlockFace};

const NUM_NEIGHBORS: usize = 4;

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
/// a rotates to b in the rotation specified by face and direction. The index of the color opposite
/// face will be None in the resulting array.
fn get_color_rotations(rotation: Rotation) -> [Option<&'static Color>; NUM_COLORS] {
    let face = rotation.face;
    let adjacent = ADJACENT_COLORS[face.idx];
    let faces_in_order: [&'static Color; NUM_NEIGHBORS] = match rotation.direction {
        Direction::Clockwise => adjacent,
        Direction::CounterClockwise => {
            let mut result = [ &WHITE; NUM_NEIGHBORS ];
            for i in NUM_NEIGHBORS-1..=0 {
                result[NUM_NEIGHBORS - i - 1] = adjacent[i];
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


pub struct RubiksCube<'a> {
    blocks: [Block<'a>; 26]
}

impl <'a> RubiksCube<'a> {
    pub fn solved() -> Self {
        const DEFAULT: Block = Block::Middle(&WHITE);
        let mut blocks = [ DEFAULT; 26];
        let mut idx = 0;

        for color in ALL_COLORS {
            blocks[idx] = Block::Middle(color);
            idx += 1;
        }

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

        assert!(idx == 26);
        Self { blocks: blocks }
    }

    pub fn turn(&mut self, rotation: Rotation) {
        let face = rotation.face;
        let rotations = get_color_rotations(rotation);
        for block in self.blocks.iter_mut() {
            if block.get_face(face) == None {
                continue;
            }

            match block {
                Block::Middle(_) => (),
                Block::Edge(ref mut a, ref mut b) => {
                    a.face = rotations[a.face.idx].unwrap();
                    b.face = rotations[b.face.idx].unwrap();
                },
                Block::Corner(ref mut a, ref mut b, ref mut c) => {
                    a.face = rotations[a.face.idx].unwrap();
                    b.face = rotations[b.face.idx].unwrap();
                    c.face = rotations[c.face.idx].unwrap();
                },
            }
        }
    }

    fn find_edge(&self, a: &Color, b: &Color) -> Option<&Block> {
        for block in self.blocks.iter() {
            match block {
                Block::Middle(_) => (),
                Block::Edge(i, j) => {
                    if (i.face == a || i.face == b) && (j.face == a || j.face == b) {
                        return Some(block);
                    }
                },
                Block::Corner(_, _, _) => ()
            }
        }

        None
    }

    fn find_corner(&self, a: &Color, b: &Color, c: &Color) -> Option<&Block> {
        // TODO find a way to do this that doesn't allocate a set every time.
        let mut colors = HashSet::new();
        colors.insert(a);
        colors.insert(b);
        colors.insert(c);

        for block in self.blocks.iter() {
            match block {
                Block::Middle(_) => (),
                Block::Edge(_, _) => (),
                Block::Corner(i, j, k) => {
                    if colors.contains(i.face) && colors.contains(j.face) && colors.contains(k.face) {
                        return Some(block);
                    }
                }
            }
        }

        None
    }

    fn write_face(&self, face: &Color, f: &mut Formatter<'_>) -> Result<(), Error> {
        let neighbors = ADJACENT_COLORS[face.idx];
        
        // TODO rewrite this to be more maintainable.
        let top_left = self.find_corner(face, neighbors[0], neighbors[3]).unwrap().get_face(face).unwrap().abrv;
        let top_middle = self.find_edge(face, neighbors[0]).unwrap().get_face(face).unwrap().abrv;
        let top_right = self.find_corner(face, neighbors[0], neighbors[1]).unwrap().get_face(face).unwrap().abrv;
        let left_middle = self.find_edge(face, neighbors[3]).unwrap().get_face(face).unwrap().abrv;
        let right_middle = self.find_edge(face, neighbors[1]).unwrap().get_face(face).unwrap().abrv;
        let bottom_left = self.find_corner(face, neighbors[3], neighbors[2]).unwrap().get_face(face).unwrap().abrv;
        let bottom_middle = self.find_edge(face, neighbors[2]).unwrap().get_face(face).unwrap().abrv;
        let bottom_right = self.find_corner(face, neighbors[2], neighbors[1]).unwrap().get_face(face).unwrap().abrv;
        
        return write!(f, "{} {} {}\n{} {} {}\n{} {} {}", top_left, top_middle, top_right, left_middle, face.abrv, right_middle, bottom_left, bottom_middle, bottom_right);
    }
}

impl <'a> Display for RubiksCube<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO print in the orange peel format rather than a vertical column
        for color in ALL_COLORS {
            self.write_face(color, f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}
