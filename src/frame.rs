use std::thread;
use std::time::{Duration, Instant};

pub struct FrameScheduler {
    frame_duration: Duration,
}

impl FrameScheduler {
    pub fn of_fps(fps: u64) -> Self {
        Self {
            frame_duration: Duration::from_millis(1000 / fps),
        }
    }

    pub fn start<F>(&self, mut callback: F) -> thread::JoinHandle<()>
    where
        F: FnMut() + Send + 'static,
    {
        let frame_duration = self.frame_duration;

        thread::spawn(move || loop {
            let frame_start = Instant::now();

            callback();

            if let Some(remaining) = frame_duration.checked_sub(frame_start.elapsed()) {
                thread::sleep(remaining);
            }
        })
    }
}
