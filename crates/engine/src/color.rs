pub struct ColorPair {
    pub fg: Color,
    pub bg: Color,
}

pub enum Color {
    Black,
    White,
    Green,
    Red,
}

impl Color {
    pub fn fg_code(&self) -> &'static str {
        use Color::*;
        match self {
            Black => "30",
            White => "37",
            Green => "32",
            Red => "31",
        }
    }

    pub fn bg_code(&self) -> &'static str {
        use Color::*;
        match self {
            Black => "40",
            White => "47",
            Green => "42",
            Red => "41",
        }
    }
}
