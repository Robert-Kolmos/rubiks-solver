#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Color {
    pub idx: usize,
    pub name: &'static str,
    pub abrv: &'static str,
}

pub const WHITE: Color = Color { idx: 0, name: "White", abrv: "w" };
pub const RED: Color = Color { idx: 1, name: "Red", abrv: "r" };
pub const BLUE: Color = Color { idx: 2, name: "Blue", abrv: "b" };
pub const ORANGE: Color = Color { idx: 3, name: "Orange", abrv: "o" };
pub const GREEN: Color = Color { idx: 4, name: "Green", abrv: "g" };
pub const YELLOW: Color = Color { idx: 5, name: "Yellow", abrv: "y" };
pub const NUM_COLORS: usize = 6;
pub const ALL_COLORS: [&Color; NUM_COLORS] = [&WHITE, &RED, &BLUE, &ORANGE, &GREEN, &YELLOW];
