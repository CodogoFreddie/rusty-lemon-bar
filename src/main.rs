use std::{thread, time, fmt};


enum Movement {
    Left,
    Center,
    Right,
    Window(u32),
}

enum Color {
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
    fn hex(&self) -> &str {
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

enum Element {
    Foreground(Color, Box<Element>),
    Background(Color, Box<Element>),
    Command(String, Box<Element>),
    Underline(Box<Element>),
    Overline(Box<Element>),
    Swap(Box<Element>),
    SwapAt(u32, Box<Element>),
    Move(Movement, Box<Element>),
    Raw(String),
}

fn render_element(el: Element) -> String {
    use Element::*;

    match el {
        Foreground(col, child) => format!("%{{F{}}}{}%{{F-}}", col.hex(), render_element(*child)),
        Background(col, child) => format!("%{{B{}}}{}%{{B-}}", col.hex(), render_element(*child)),
        Command(com, child) => format!("%{{A:{}:}}{}%{{A}}", com, render_element(*child)),
        Underline(child) => format!("%{{+u}}{}%{{-u}}", render_element(*child)),
        Overline(child) => format!("%{{+o}}{}%{{-o}}", render_element(*child)),
        Swap(child) => format!("%{{R}}{}%{{R}}", render_element(*child)),
        SwapAt(_, child) => render_element(Swap(child)),
        Move(_, child) => render_element(*child),
        Raw(s) => s,
    }
}

enum Format {
    Foreground(Color),
    Background(Color),
    Underline,
    Overline,
    Swap,
    SwapAt(f32),
}

impl Format {
    fn apply(&self, s: String) -> String {
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

fn main() {

    loop {
        let formatters = [
            Format::Foreground(Color::Black),
            Format::Background(Color::White),
            Format::Underline,
            Format::Swap
        ];

        let output = formatters.iter().fold( String::from("foo"), |acc, f| { f.apply(acc) } );
        println!("output = {}", output);

        thread::sleep(
            time::Duration::from_millis(1000)
        );
    }
}
