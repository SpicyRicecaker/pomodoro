use std::time;
use std::time::Instant;

use crate::Global;
use crate::PomoState;
use crate::alarm;
use crate::work::Work;

pub struct LargeBreak {
    duration: time::Duration,
}

impl LargeBreak {
    fn new(duration: time::Duration) -> Self {
        Self { duration }
    }
}

impl PomoState<LargeBreak> {
    pub fn tick(&mut self) {
        println!("Large Break, START!",);
        alarm(Instant::now(), self.state.duration);
    }
}

impl From<PomoState<Work>> for PomoState<LargeBreak> {
    fn from(wo: PomoState<Work>) -> Self {
            Self {
                global: Global {
                    num_of_work: 0,
                    ..wo.global
                },
                state: LargeBreak::new(wo.global.large_break_time),
            }
    }
}

