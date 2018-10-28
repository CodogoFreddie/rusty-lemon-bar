use std::fmt;

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Orange,
    White,
}

impl Color {
    pub fn hex(&self) -> &str {
        use Color::*;

        match *self {
            Black => "#2c292d",
            Red => "#ff6188",
            Green => "#a9dc76",
            Yellow => "#ffd866",
            Blue => "#78dce8",
            Purple => "#ab9df2",
            Orange => "#fc9867",
            White => "#fdf9f3",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hex())
    }
}

pub enum Format {
    Foreground(Color),
    Background(Color),
    Underline,
    Overline,
    Swap,
    SwapAt(f32),
}

impl Format {
    pub fn apply(&self, s: String) -> String {
        use Format::*;

        match *self {
            Foreground(ref col) => format!("%{{F{}}}{}%{{F-}}", col, s),
            Background(ref col) => format!("%{{B{}}}{}%{{B-}}", col, s),
            Underline => format!("%{{+u}}{}%{{-u}}", s),
            Overline => format!("%{{+o}}{}%{{-o}}", s),
            SwapAt(_) => Format::Swap.apply(s),
            Swap => format!("%{{R}}{}%{{R}}", s), 
        }
    }
}
