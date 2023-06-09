use super::color::Color;

/// Represents a single face of a Block which has it's own color and a pointer to the face that
/// color is on.
#[derive(Clone)]
pub struct BlockFace<'a> {
    pub color: &'a Color,
    pub face: &'a Color,
}

impl <'a> BlockFace<'a> {
    pub fn is_solved(&self) -> bool {
        self.color == self.face
    }
}

/// Represents a single piece of the larger rubiks cube.
#[derive(Clone)]
pub enum Block<'a> {
    Edge(BlockFace<'a>, BlockFace<'a>),
    Corner(BlockFace<'a>, BlockFace<'a>, BlockFace<'a>)
}

impl <'a> Block<'a> {
    /// Returns an Edge with the 2 specified colors where each color is on the correct face.
    pub fn solved_edge(a: &'a Color, b: &'a Color) -> Self {
        return Block::Edge(BlockFace { color: a, face: a}, BlockFace { color: b, face: b});
    }

    /// Returns a Corner with the 3 specified colors where each color is on the correct face.
    pub fn solved_corner(a: &'a Color, b: &'a Color, c: &'a Color) -> Self {
        Block::Corner(
            BlockFace { color: a, face: a }, 
            BlockFace { color: b, face: b },
            BlockFace { color: c, face: c }
        )
    }

    pub fn is_solved(&self) -> bool {
        match &self {
            Block::Edge(a, b) =>
                a.is_solved() && b.is_solved(),
            Block::Corner(a, b, c) =>
                a.is_solved() && b.is_solved() && c.is_solved()
        }
    }

    /// Returns the color associated with specified face or None if self does not touch the specified
    /// face.
    pub fn get_face(&self, face: &Color) -> Option<&Color> {
        let colors = match self {
            Block::Edge(a, b) => vec![a, b] ,
            Block::Corner(a, b, c) => vec![a, b, c]
        };

        for color in colors {
            if color.face == face {
                return Some(color.color)
            }
        }

        None
    }
}
