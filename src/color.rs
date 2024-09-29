#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Gray,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Orange,
    Yellow,
}

impl From<Color> for String {
    fn from(color: Color) -> String {
        match color {
            Color::Black => "\x1b[48;2;000;000;000m　".to_string(),
            Color::Gray => "\x1b[48;2;127;127;127m　".to_string(),
            Color::Blue => "\x1b[48;2;000;000;255m　".to_string(),
            Color::Green => "\x1b[48;2;000;255;000m　".to_string(),
            Color::Cyan => "\x1b[48;2;000;255;255m　".to_string(),
            Color::Red => "\x1b[48;2;255;000;000m　".to_string(),
            Color::Magenta => "\x1b[48;2;255;000;255m　".to_string(),
            Color::Orange => "\x1b[48;2;255;127;000m　".to_string(),
            Color::Yellow => "\x1b[48;2;255;255;000m　".to_string(),
        }
    }
}
