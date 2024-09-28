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

    let frame_scheduler = FrameScheduler::new(60);
    let handle = frame_scheduler.start(move |fps| {
        canvas.draw(field.into());
        canvas.render();
    });

    handle.join().unwrap();
}
