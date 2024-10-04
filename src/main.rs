mod canvas;
mod color;
mod field;
mod frame;
mod point;

use crate::canvas::Canvas;
use crate::field::{Field};
use crate::frame::FrameScheduler;
use crate::point::Point;

fn main() {
    let field = Field::default();
    let mut canvas = Canvas::default();

    let frame_scheduler = FrameScheduler::new(60);
    let handle = frame_scheduler.start(move |fps| {
        canvas.draw(Point::new(0, 0), field.into());
        canvas.draw(
            Point::new(Field::WIDTH + 1, 0),
            vec![vec![format!("FPS: {}", fps.floor())]],
        );
        canvas.render();
    });

    handle.join().unwrap();
}
