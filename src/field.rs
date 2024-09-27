use crate::color::Color;

const WIDTH: usize = 13;

const HEIGHT: usize = 21;

#[derive(Clone, Copy)]
enum FieldCell {
    Empty,
    Wall,
    Block(Color),
}

impl FieldCell {
    fn get_color(&self) -> Color {
        match self {
            Self::Empty => Color::Black,
            Self::Wall => Color::Gray,
            Self::Block(color) => *color,
        }
    }

    fn has_collision(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Wall | Self::Block(_) => true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Field {
    blocks: [[FieldCell; WIDTH]; HEIGHT],
}

impl Default for Field {
    fn default() -> Self {
        let row = [FieldCell::Empty; WIDTH - 2];
        let mut blocks = [[FieldCell::Wall; WIDTH]; HEIGHT];
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
            .map(|row| row.iter().map(|cell| cell.get_color().into()).collect())
            .collect()
    }
}
