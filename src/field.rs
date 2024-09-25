use crate::color::Color;

const WIDTH: usize = 13;

const HEIGHT: usize = 21;

#[derive(Clone, Copy)]
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

impl From<Field> for Vec<Vec<&'static str>> {
    fn from(value: Field) -> Self {
        value
            .blocks
            .into_iter()
            .map(|row| row.into_iter().map(Into::into).collect())
            .collect()
    }
}
