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
    fn has_collision(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Wall | Self::Block(_) => true,
        }
    }
}

impl From<&FieldCell> for String {
    fn from(color: &FieldCell) -> String {
        match color {
            FieldCell::Empty => format!("{}　", Color::Black),
            FieldCell::Wall => format!("{}　", Color::Gray),
            FieldCell::Block(color) => format!("{}　", color),
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

impl From<Field> for Vec<Vec<String>> {
    fn from(value: Field) -> Self {
        value
            .blocks
            .iter()
            .map(|row| row.iter().map(|cell| cell.into()).collect())
            .collect()
    }
}
