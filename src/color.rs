use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Gray,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Orange,
    Yellow,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Color::Black => "\x1b[48;2;000;000;000m",
            Color::Gray => "\x1b[48;2;127;127;127m",
            Color::Blue => "\x1b[48;2;000;000;255m",
            Color::Green => "\x1b[48;2;000;255;000m",
            Color::Cyan => "\x1b[48;2;000;255;255m",
            Color::Red => "\x1b[48;2;255;000;000m",
            Color::Purple => "\x1b[48;2;255;000;255m",
            Color::Orange => "\x1b[48;2;255;127;000m",
            Color::Yellow => "\x1b[48;2;255;255;000m",
        };
        write!(f, "{}", str)
    }
}
