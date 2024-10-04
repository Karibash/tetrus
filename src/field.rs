use crate::color::Color;

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
    blocks: [[FieldCell; Self::WIDTH]; Self::HEIGHT],
}

impl Field {
    pub const HEIGHT: usize = 21;

    pub const WIDTH: usize = 13;
}

impl Default for Field {
    fn default() -> Self {
        let row = [FieldCell::Empty; Self::WIDTH - 2];
        let mut blocks = [[FieldCell::Wall; Self::WIDTH]; Self::HEIGHT];
        for block in &mut blocks[..Self::HEIGHT - 1] {
            block[1..Self::WIDTH - 1].copy_from_slice(&row);
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
