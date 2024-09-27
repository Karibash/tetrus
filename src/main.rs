mod canvas;
mod color;
mod field;
mod frame;

use crate::canvas::Canvas;
use crate::field::Field;
use crate::frame::FrameScheduler;

fn main() {
    let field = Field::default();
    let mut canvas = Canvas::default();

    Canvas::clear();

    let frame_scheduler = FrameScheduler::of_fps(60);
    let handle = frame_scheduler.start(move || {
        canvas.draw(field.into());
        canvas.render();
    });

    handle.join().unwrap();
}
