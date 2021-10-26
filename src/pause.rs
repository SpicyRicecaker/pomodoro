use std::time;
use std::time::Instant;

use crate::large_break::LargeBreak;
use crate::normal_break::Break;
use crate::work::Work;
use crate::PomoState;
use crate::{alarm, PomoMachine};

pub struct Pause {
    start: time::Instant,
    // time remaining left in work / break or whatever
    paused: Box<PomoMachine>,
}

impl Pause {
    pub fn new(paused: PomoMachine) -> Self {
        Self {
            start: Instant::now(),
            paused: Box::new(paused),
        }
    }
}

impl PomoState<Pause> {
    pub fn tick(&mut self) {
        println!(
            "Mini Break {}/{}, START!",
            self.global.num_of_work,
            self.global.total_work - 1
        );
        alarm(Instant::now(), self.state.duration);
    }
}

impl From<PomoState<Work>> for PomoState<Pause> {
    fn from(wo: PomoState<Work>) -> Self {
        Self {
            state: Pause::new(PomoMachine::Work(wo)),
            global: wo.global,
        }
    }
}

impl From<PomoState<Break>> for PomoState<Pause> {
    fn from(wo: PomoState<Break>) -> Self {
        Self {
            state: Pause::new(PomoMachine::Break(wo)),
            global: wo.global,
        }
    }
}

impl From<PomoState<LargeBreak>> for PomoState<Pause> {
    fn from(wo: PomoState<LargeBreak>) -> Self {
        Self {
            state: Pause::new(PomoMachine::LargeBreak(wo)),
            global: wo.global,
        }
    }
}
