mod canvas;
mod color;
mod field;

use crate::canvas::Canvas;
use crate::field::Field;
use std::thread;
use std::time::{Duration, Instant};

const FRAME_DURATION: Duration = Duration::from_millis(1000 / 60);

fn main() {
    let field = Field::default();
    let mut canvas = Canvas::default();

    Canvas::clear();
    loop {
        let frame_start = Instant::now();

        canvas.draw(field.into());
        canvas.render();

        let frame_time = Instant::now().duration_since(frame_start);
        if frame_time < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - frame_time);
        }
    }
}
