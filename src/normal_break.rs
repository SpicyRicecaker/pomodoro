use std::thread;
use std::time;

use crate::work::Work;
use crate::PomoState;

pub struct Break {
    duration: time::Duration,
}

impl Break {
    pub fn new(duration: time::Duration) -> Self {
        Self { duration }
    }
}

impl PomoState<Break> {
    pub fn tick(&mut self) {
        println!(
            "Mini Break {}/{}, START!",
            self.global.num_of_work,
            self.global.total_work - 1
        );
        thread::sleep(self.state.duration);
    }
}

impl From<PomoState<Work>> for PomoState<Break> {
    fn from(wo: PomoState<Work>) -> Self {
        Self {
            state: Break::new(wo.global.break_time),
            global: wo.global,
        }
    }
}
