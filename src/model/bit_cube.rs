use crate::model;
use model::rotation::Direction;
use std::fmt::{Display, Formatter, Result as FormatResult};
use anyhow::{bail, Result};
use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub struct Face {
    ///
    /// Representation:
    ///
    /// Squares are indexed according to the following diagram:
    /// | 0 | 1 | 2 |
    /// | 7 | 8 | 3 |
    /// | 6 | 5 | 4 |
    ///
    /// Each square is then represented using 3 bits according to the following diagram:
    /// [    ] [8] [7] [6] [5] [4] [3] [2] [1] [0]
    /// -- --- --- --- --- --- --- --- --- --- --- 
    values: u32,
}

const LEAST_MASK: u32 = 0b111;
const TOP_EDGE_MASK: u32 = 0b111_111_111;
const CENTER_MASK: u32 = LEAST_MASK << 24;
const EDGE_MASK: u32 = !CENTER_MASK ^ (0b11111 << 27);
const OVER_UNDER_FLOW_SHIFT: u8 = 18;
const UNDERFLOW_MASK: u32 = 0b111_111;
const OVERFLOW_MASK: u32 = UNDERFLOW_MASK << OVER_UNDER_FLOW_SHIFT;
const BLOCK_SIZE: u8 = 3;
const ROTATE_SHIFT: u8 = 2 * BLOCK_SIZE;
const INTERNAL_TO_ROW_MAJOR: [usize; 9] = [0, 1, 2, 5, 8, 7, 6, 3, 4];
const ROW_MAJOR_TO_INTERNAL: [usize; 9] = [0, 1, 2, 7, 8, 3, 6, 5, 4];
const SIDE_LEN: usize = 3;


const ROTATION_INDEXES: [[(usize, [usize; 3]); 4]; 6] = [
    [ (1, [0, 1, 2]), (2, [0, 1, 2]), (3, [0, 1, 2]), (4, [0, 1, 2]) ],
    [ (0, [0, 6, 7]), (4, [4, 2, 3]), (5, [0, 6, 7]), (2, [0, 6, 7]) ],
    [ (0, [6, 5, 4]), (1, [4, 3, 2]), (5, [2, 1, 0]), (3, [0, 7, 6]) ],
    [ (0, [2, 3, 4]), (2, [2, 3, 4]), (5, [2, 3, 4]), (4, [6, 7, 0]) ],
    [ (0, [0, 1, 2]), (3, [2, 3, 4]), (5, [4, 5, 6]), (1, [6, 7, 0]) ],
    [ (1, [4, 5, 6]), (4, [4, 5, 6]), (3, [4, 5, 6]), (2, [4, 5, 6]) ],
];

#[derive(PartialEq, Eq)]
pub enum Ordinal {
    North,
    South,
    East,
    West
}


impl Face {
    pub fn solved(value: u32) -> Result<Self> {
        Self::validate(value)?;

        let mut values = 0;
        for i in 0..INTERNAL_TO_ROW_MAJOR.len() {
            let shift = i * BLOCK_SIZE as usize;
            values += value << shift;
        }
        Ok(Self { values: values })
    }

    pub fn rotate(&self, direction: Direction) -> Face {
        let mut copy = self.clone();
        copy.rotate_mut(direction);
        copy
    }

    pub fn rotate_mut(&mut self, direction: Direction) {
        let edges = self.values;
        let center = self.values & CENTER_MASK;
    
        if direction == Direction::Clockwise {
            let rotated_edges = (edges << ROTATE_SHIFT) & EDGE_MASK;
            let overflow = (self.values & OVERFLOW_MASK) >> OVER_UNDER_FLOW_SHIFT;
            self.values = overflow | rotated_edges | center
        } else {
            let rotated_edges = (edges & EDGE_MASK) >> ROTATE_SHIFT;
            let underflow = (self.values & UNDERFLOW_MASK) << OVER_UNDER_FLOW_SHIFT;
            self.values = underflow | rotated_edges | center
        }
    }

    pub fn get_edge(&self, ordinal: &Ordinal, reverse: bool) -> u32 {
        let unreversed_raw = match ordinal {
            Ordinal::North => self.values,
            Ordinal::South => self.values >> (ROTATE_SHIFT << 1),
            Ordinal::East => self.values >> ROTATE_SHIFT,
            Ordinal::West => {
                let first_two = (self.values >> (ROTATE_SHIFT * 3)) & UNDERFLOW_MASK;
                let last = self.values << ROTATE_SHIFT;
                first_two | last
            }
        };
        let unreversed = unreversed_raw & TOP_EDGE_MASK;

        if reverse {
            let first = unreversed >> (BLOCK_SIZE * 2);
            let middle = unreversed & (LEAST_MASK << BLOCK_SIZE);
            let last = (unreversed << (BLOCK_SIZE * 2)) & TOP_EDGE_MASK;
            first | middle | last
        } else {
            unreversed
        }
    }

    pub fn set_edge(&mut self, values: u32, ordinal: &Ordinal) {
        let shift = match ordinal {
            Ordinal::North => 0,
            Ordinal::South => ROTATE_SHIFT << 1,
            Ordinal::East => ROTATE_SHIFT,
            Ordinal::West => {
                // The memory for the west face is not continous so we have to do bespoke logic
                let first = 0o7 & values;
                let last_two = 0o770 & values;
                let shift = ROTATE_SHIFT * 3 - BLOCK_SIZE;
                let mask = !0o77_000_007;
                self.values = (self.values & mask) | first | (last_two << shift);
                return;
            },
        };

        let shifted_values = values << shift;
        let mask = !(0o777 << shift);
        self.values = (self.values & mask) | shifted_values;
    }

    fn validate(value: u32) -> Result<()> {
        if value > 7 || value == 0 {
            return bail!("Encountered value: {}, which must be in (0, 7]", value);
        }

        Ok(())
    }

    /// Constructs a face form the given array of values, All values in the array must be less
    /// than 6. The provided array is expected to be ordered like the following diagram
    /// 
    /// | 0 | 1 | 2 |
    /// | 3 | 4 | 5 |
    /// | 6 | 7 | 8 |
    pub fn from(values: [u32; 9]) -> Result<Self, > {
        let mut shift = 0;
        let mut representation: u32 = 0;
        for i in 0..values.len() {
            let value = values[ROW_MAJOR_TO_INTERNAL[i]];
            Face::validate(value)?;
            representation |= value << shift;
            shift += BLOCK_SIZE;
        }

        Ok(Face { values: representation })
    }

    /// Returns the value at index. Indexing is done according to the following diagram
    /// 
    /// | 0 | 1 | 2 |
    /// | 7 | 8 | 3 |
    /// | 6 | 5 | 4 |
    pub fn get(&self, index: usize) -> u8 {
        if index > 8 {
            panic!("overflow when indexing Face with index: {}", index);
        }

        let shift = index * (BLOCK_SIZE as usize);
        let mask = LEAST_MASK << shift;
        ((self.values & mask) >> shift) as u8
    }

    /// Sets the value at index to the provided value. Indexing is done according to the following diagram
    /// 
    /// | 0 | 1 | 2 |
    /// | 7 | 8 | 3 |
    /// | 6 | 5 | 4 |
    pub fn set(&mut self, index: usize, value: u32) {
        Self::validate(value).expect("Invalid value passed to set");

        let shift = index * (BLOCK_SIZE as usize);
        let mask = LEAST_MASK << shift;
        self.values = (self.values & (!mask)) | (value << shift);
    }

    pub fn pretty_fmt(
        &self, 
        f: &mut Formatter<'_>,
        l_pad: Option<&str>,
        r_pad: Option<&str>,
        delimiter: Option<&str>
    ) -> FormatResult {
        for i in 0..3 {
            self.pretty_fmt_row(i, f, l_pad, r_pad, delimiter)?;
        }
        Ok(())
    }

    pub fn pretty_fmt_row(
        &self, 
        row: usize,
        f: &mut Formatter<'_>,
        l_pad: Option<&str>,
        r_pad: Option<&str>,
        delimiter: Option<&str>,
    ) -> FormatResult {
        let l_pad_str = l_pad.unwrap_or("");
        let r_pad_str = r_pad.unwrap_or("");
        let delimiter_str = delimiter.unwrap_or(" ");
        let lower = row * SIDE_LEN;
        let upper = (row + 1) * SIDE_LEN;
        write!(f, "{}", l_pad_str)?;
        write!(f, "{}", &self.get(ROW_MAJOR_TO_INTERNAL[lower]))?;
        for i in lower + 1..upper {
            write!(f, "{}{}", delimiter_str, &self.get(ROW_MAJOR_TO_INTERNAL[i]))?;
        }
        write!(f, "{}", r_pad_str)?;
        Ok(())
    }

}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        self.pretty_fmt(f, Some("|"), Some("|\n"), None)
    }
}

#[derive(Clone)]
pub struct BitCube {
    faces: [Face; 6]
}

impl BitCube {
    pub fn solved() -> Self {
        const DEFAULT: Face = Face { values: 0 };  
        let mut faces = [ DEFAULT; 6 ];
        for i in 1..=faces.len() {
            faces[i - 1] = Face::solved(i as u32).expect(&format!("Error constructing solved face with value {}", i));
        }

        BitCube { faces: faces }
    }

    fn copy_values(&self, face: usize, indexes: [usize; 3]) -> [u8; 3] {
        let face = &self.faces[face];
        let mut result = [0; 3];
        for i in 0..result.len() {
            result[i] = face.get(indexes[i]);
        }
        result
    }

    fn overwrite_values(&mut self, face_index: usize, indexes: [usize; 3], values: [u8; 3]) {
        let face = &mut self.faces[face_index];
        for i in 0..indexes.len() {
            face.set(indexes[i], values[i] as u32);
        }
    }

    pub fn turn(&mut self, face: usize, direction: Direction) {
        let mut indexes = ROTATION_INDEXES[face].clone();
        if direction == Direction::CounterClockwise {
            indexes.reverse();
        }
 
        let (tmp_face, tmp_indexes) = indexes[0];
        let tmp_values = self.copy_values(tmp_face, tmp_indexes);
        for i in 0..indexes.len() - 1 {
            let (src_face, src_indexes) = indexes[i + 1];
            let src_values = self.copy_values(src_face, src_indexes);

            let (dest_face, dest_indexes) = indexes[i];
            self.overwrite_values(dest_face, dest_indexes, src_values);
        }
        let (last_face, last_face_indexes) = indexes[indexes.len() - 1];
        self.overwrite_values(last_face, last_face_indexes, tmp_values);

        self.faces[face].rotate_mut(direction);
    }

    pub fn from(src: [u32; 6]) -> Self {
        const DEFAULT: Face = Face { values: 0 };
        let mut faces = [ DEFAULT; 6 ];
        for i in 0..src.len() {
            faces[i] = Face { values: src[i] };
        }
        Self { faces: faces }
    }

    pub fn pretty_to_string(&self, color_mapping: &[&str; 6]) -> String {
        let mut str = self.to_string();
        for i in 1..=6 {
            str = str.replace(&format!("{}", i), color_mapping[i - 1]);
        }

        str
    }
}

impl Display for BitCube {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let space = "        ";
        let blank = &format!("{}| ", space);
        let dash = "--------";
        let delimiter = Some(" ");

        writeln!(f, "{}{}-", space, dash)?;
        self.faces[0].pretty_fmt(f, Some(blank), Some(" |\n"), delimiter)?;
        writeln!(f, "{}{}{}{}-", dash, dash, dash, dash)?;
        for i in 0..SIDE_LEN {
            for j in 1..5 {
                let l_pad = if j == 1 {
                    Some("| ") 
                } else {
                    None
                };
                let r_pad = if j == 4 {
                    Some(" |\n")
                } else {
                    Some(" | ")
                };
                self.faces[j].pretty_fmt_row(i, f, l_pad, r_pad, delimiter)?;
            }
        }
        writeln!(f, "{}{}{}{}-", dash, dash, dash, dash)?;
        self.faces[5].pretty_fmt(f, Some(blank), Some(" |\n"), delimiter)?;
        writeln!(f, "{}{}-", space, dash)?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Move {
    pub face: usize,
    pub direction: Direction,
}

impl Move {
    pub fn random(rng: &mut impl Rng) -> Self {
        let i: usize = rng.gen_range(0..6 * 2);

        let direction = match i {
            i if i % 2 == 0 => Direction::Clockwise,
            _ => Direction::CounterClockwise
        };
        Move { face: i / 2, direction: direction }
    }

    pub fn pretty_fmt(&self, color_mapping: &[&str; 6]) -> String {
        let mut printed = self.to_string();
        for i in 0..color_mapping.len() {
            printed = printed.replace(&format!("{}", i), color_mapping[i]);
        }

        printed
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let direction_str = match self.direction {
            Direction::Clockwise => "",
            Direction::CounterClockwise => "'",
        };
        write!(f, "{}{}", self.face, direction_str)?;
        Ok(())
    }
}
