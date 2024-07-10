use std::ops::Sub;
use std::time::{Duration, Instant};

pub struct Timer {
    current: Instant,
    time_paused: Duration,
    paused: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            current: Instant::now(),
            time_paused: Duration::new(0, 0),
            paused: false,
        }
    }

    pub fn restart(&mut self) {
        self.current = Instant::now();
        self.time_paused = Duration::new(0, 0);
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn time(&mut self) -> Instant {
        if self.paused {
            self.time_paused = Instant::now().duration_since(self.current);
        } else {
            self.current = Instant::now().sub(self.time_paused);
        }

        self.current
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }
}
