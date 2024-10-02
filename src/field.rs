use crate::color::Color;

pub const FIELD_WIDTH: usize = 13;

pub const FIELD_HEIGHT: usize = 21;

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
    blocks: [[FieldCell; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl Default for Field {
    fn default() -> Self {
        let row = [FieldCell::Empty; FIELD_WIDTH - 2];
        let mut blocks = [[FieldCell::Wall; FIELD_WIDTH]; FIELD_HEIGHT];
        for block in &mut blocks[..FIELD_HEIGHT - 1] {
            block[1..FIELD_WIDTH - 1].copy_from_slice(&row);
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
