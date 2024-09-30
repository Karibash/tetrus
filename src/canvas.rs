use std::io::{stdout, Write};
use std::ops::Add;
use crate::color::Color;

pub struct Canvas {
    codes: Vec<Vec<String>>,
}

impl Canvas {
    pub fn draw(&mut self, codes: Vec<Vec<String>>) {
        self.codes = codes;
    }

    pub fn render(&self) {
        let text = self
            .codes
            .iter()
            .map(|row| row.concat())
            .collect::<Vec<_>>()
            .join(format!("{}\x1b[K\n", Color::Black).as_str())
            .add(format!("{}\x1B[0J\n", Color::Black).as_str());

        print!("\x1B[H{}", text);
        stdout().flush().unwrap();
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self { codes: vec![] }
    }
}
