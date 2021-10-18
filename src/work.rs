use std::thread;
use std::time;

use crate::large_break::LargeBreak;
use crate::normal_break::Break;
use crate::Global;
use crate::PomoState;

pub struct Work {
    duration: time::Duration,
}

impl Work {
    pub fn new(duration: time::Duration) -> Self {
        Self { duration }
    }
}

impl PomoState<Work> {
    pub fn tick(&mut self) {
        // print message
        self.global.num_of_work += 1;
        println!(
            "Work {}/{}, START!",
            self.global.num_of_work, self.global.total_work
        );
        thread::sleep(self.state.duration);
    }
}

impl From<PomoState<Break>> for PomoState<Work> {
    fn from(br: PomoState<Break>) -> Self {
        Self {
            state: Work::new(br.global.work_time),
            global: br.global,
        }
    }
}

impl From<PomoState<LargeBreak>> for PomoState<Work> {
    fn from(br: PomoState<LargeBreak>) -> Self {
        Self {
            state: Work::new(br.global.work_time),
            global: Global {
                num_of_work: 0,
                ..br.global
            },
        }
    }
}
