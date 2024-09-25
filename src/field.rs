use crate::color::Color;

const WIDTH: usize = 13;

const HEIGHT: usize = 21;

#[derive(Clone, Copy)]
struct FieldCell {
    color: Color,
    has_collision: bool,
}

impl FieldCell {
    fn empty() -> Self {
        Self {
            color: Color::Black,
            has_collision: false,
        }
    }

    fn block(color: Color) -> Self {
        Self {
            color,
            has_collision: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Field {
    blocks: [[FieldCell; WIDTH]; HEIGHT],
}

impl Default for Field {
    fn default() -> Self {
        let row = [FieldCell::empty(); WIDTH - 2];
        let mut blocks = [[FieldCell::block(Color::Gray); WIDTH]; HEIGHT];
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
            .iter()
            .map(|row| row.iter().map(|cell| cell.color.into()).collect())
            .collect()
    }
}
