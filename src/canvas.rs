use crate::color::Color;
use crate::point::Point;
use std::io::{stdout, Write};
use std::ops::Add;

pub struct Canvas {
    codes: Vec<Vec<String>>,
}

impl Canvas {
    pub fn draw(&mut self, point: Point, codes: Vec<Vec<String>>) {
        self.resize(point.x + codes[0].len(), point.y + codes.len());

        for (i, row) in codes.iter().enumerate() {
            for (j, code) in row.iter().enumerate() {
                self.codes[point.y + i][point.x + j] = code.clone();
            }
        }
    }

    pub fn render(&self) {
        let text = self
            .codes
            .iter()
            .map(|row| row.concat())
            .collect::<Vec<_>>()
            .join(&format!("{}\x1b[K\n", Color::Black))
            .add(&format!("{}\x1B[0J\n", Color::Black));

        print!("\x1B[H{}", text);
        stdout().flush().unwrap();
    }

    fn resize(&mut self, width: usize, height: usize) {
        let empty_code = format!("{}　", Color::Black);

        if self.codes.len() < height {
            self.codes.resize(height, vec![empty_code.clone(); width]);
        }

        if self.codes[0].len() < width {
            for row in &mut self.codes {
                row.resize(width, empty_code.clone());
            }
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self { codes: vec![] }
    }
}
