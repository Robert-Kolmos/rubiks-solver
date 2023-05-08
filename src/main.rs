use std::collections::{HashSet};
use std::fmt::{Display, Formatter, Error};
use std::result::Result;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Color {
    idx: usize,
    name: &'static str,
    abrv: &'static str,
}

const NUM_COLORS: usize = 6;
const NUM_NEIGHBORS: usize = 4;
const WHITE: Color = Color { idx: 0, name: "White", abrv: "w" };
const RED: Color = Color { idx: 1, name: "Red", abrv: "r" };
const BLUE: Color = Color { idx: 2, name: "Blue", abrv: "b" };
const ORANGE: Color = Color { idx: 3, name: "Orange", abrv: "o" };
const GREEN: Color = Color { idx: 4, name: "Green", abrv: "g" };
const YELLOW: Color = Color { idx: 5, name: "Yellow", abrv: "y" };
const ALL_COLORS: [&Color; NUM_COLORS] = [&WHITE, &RED, &BLUE, &ORANGE, &GREEN, &YELLOW];

// Stored in the order Top, Right, Bottom, Left. 
const ADJACENT_COLORS: [[&Color; NUM_NEIGHBORS]; NUM_COLORS] = [
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
fn get_color_rotations(face: &Color, direction: Direction) -> [Option<&Color>; NUM_COLORS] {
    let adjacent = ADJACENT_COLORS[face.idx];
    let faces_in_order = match direction {
        Direction::Clockwise => adjacent,
        Direction::CounterClockwise => {
            let mut result = adjacent.clone();
            result.reverse();
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

#[derive(Debug)]
pub struct ColorPointer<'a> {
    color: &'a Color,
    face: &'a Color,
}

pub enum Block<'a> {
    Middle(&'a Color),
    Edge(ColorPointer<'a>, ColorPointer<'a>),
    Corner(ColorPointer<'a>, ColorPointer<'a>, ColorPointer<'a>)
}

impl <'a> Block<'a> {
    fn solved_edge(a: &'a Color, b: &'a Color) -> Self {
        return Block::Edge(ColorPointer { color: a, face: a}, ColorPointer { color: b, face: b});
    }

    fn solved_corner(a: &'a Color, b: &'a Color, c: &'a Color) -> Self {
        Block::Corner(
            ColorPointer { color: a, face: a }, 
            ColorPointer { color: b, face: b },
            ColorPointer { color: c, face: c }
        )
    }

    fn get_face(&self, face: &Color) -> Option<&Color> {
        match self {
            Block::Middle(a) => if *a == face {
                Some(a)
            } else {
                None
            },
            Block::Edge(a, b) => if a.face == face { 
                Some(a.color) 
            } else if b.face == face {
                Some(b.color)
            } else {
                None
            },
            Block::Corner(a, b, c) =>  if a.face == face { 
                Some(a.color) 
            } else if b.face == face {
                Some(b.color)
            } else if c.face == face {
                Some(c.color)
            } else {
                None
            },
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Direction {
    Clockwise = 0,
    CounterClockwise = 1,
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

    pub fn turn(&mut self, face: &Color, direction: Direction) {
        let rotations = get_color_rotations(face, direction);

        for block in self.blocks.iter_mut() {
            if block.get_face(face) == None {
                continue;
            }

            match block {
                Block::Middle(_) => (),
                Block::Edge(ref mut a, ref mut b) => {
                    *a = ColorPointer  { color: a.color, face: ALL_COLORS[rotations[a.face.idx].unwrap().idx] };
                    *b = ColorPointer { color: b.color, face: ALL_COLORS[rotations[b.face.idx].unwrap().idx] };
                },
                Block::Corner(ref mut a, ref mut b, ref mut c) => {
                    *a = ColorPointer  { color: a.color, face: ALL_COLORS[rotations[a.face.idx].unwrap().idx] };
                    *b = ColorPointer { color: b.color, face: ALL_COLORS[rotations[b.face.idx].unwrap().idx] };
                    *c = ColorPointer { color: c.color, face: ALL_COLORS[rotations[c.face.idx].unwrap().idx] };
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
        for color in ALL_COLORS {
            self.write_face(color, f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}


fn main() {
    let mut solved = RubiksCube::solved();
    println!("Initial");
    println!("{}", solved);

    println!("-------------------");
    println!("Turned White Clockwise");
    solved.turn(&WHITE, Direction::Clockwise);
    println!("{}", solved);

    println!("-------------------");
    println!("Turned Red Clockwise");
    solved.turn(&RED, Direction::Clockwise);
    println!("{}", solved)
}
