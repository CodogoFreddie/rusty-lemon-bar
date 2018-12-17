use std::fmt;

#[derive(Clone)]
pub enum Color {
    Black,
    Red,
    Green,
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
    Swap,
    SwapAt(f32),
}

impl Format {
    pub fn apply(&self, s: String) -> String {
        use Format::*;

        match *self {
            Foreground(ref col) => format!("%{{F{}}}{}%{{F-}}", col, s),
            Background(ref col) => format!("%{{B{}}}{}%{{B-}}", col, s),
            Swap => format!("%{{R}}{}%{{R}}", s),
            SwapAt(f) => {
                let length = s.len();
                let split_at = (f * (length as f32)) as usize;
                let start: String = s.chars().take(split_at).collect();
                let end: String = s.chars().skip(split_at).collect();

                return format!("{}{}", start, Format::Swap.apply(end));
            }
        }
    }
}
