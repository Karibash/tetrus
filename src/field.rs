use crate::color::Color;

const WIDTH: usize = 13;

const HEIGHT: usize = 21;

pub struct Field {
    blocks: [[Color; WIDTH]; HEIGHT],
}

impl Default for Field {
    fn default() -> Self {
        let row = [Color::Black; WIDTH - 2];
        let mut blocks = [[Color::Gray; WIDTH]; HEIGHT];
        for block in &mut blocks[..HEIGHT - 1] {
            block[1..WIDTH - 1].copy_from_slice(&row);
        }
        Self { blocks }
    }
}
