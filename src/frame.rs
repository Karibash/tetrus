use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};

pub struct FrameScheduler {
    frame_rate: u8,
}

impl FrameScheduler {
    pub fn new(frame_rate: u8) -> Self {
        Self { frame_rate }
    }

    pub fn start<F>(&self, mut callback: F) -> thread::JoinHandle<()>
    where
        F: FnMut(f64) + Send + 'static,
    {
        let frame_duration = Duration::from_nanos(1_000_000_000 / self.frame_rate as u64);
        let mut frame_metrics = FrameMetrics::new(self.frame_rate);

        thread::spawn(move || {
            let mut frame_start = Instant::now();

            loop {
                frame_metrics.push(frame_start);

                callback(frame_metrics.current_frame_rate());

                frame_start += frame_duration;
                if let Some(remaining) = frame_start.checked_duration_since(Instant::now()) {
                    thread::sleep(remaining);
                }
            }
        })
    }
}

struct FrameMetrics {
    frame_times: VecDeque<Instant>,
}

impl FrameMetrics {
    fn new(frame_rate: u8) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(frame_rate as usize),
        }
    }

    fn push(&mut self, frame_time: Instant) {
        if self.frame_times.len() == self.frame_times.capacity() {
            self.frame_times.pop_front();
        }
        self.frame_times.push_back(frame_time);
    }

    fn current_frame_rate(&self) -> f64 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }

        let first_time = self.frame_times.front().unwrap();
        let last_time = self.frame_times.back().unwrap();
        let elapsed_time = last_time.duration_since(*first_time);
        self.frame_times.len() as f64 / elapsed_time.as_secs_f64()
    }
}
