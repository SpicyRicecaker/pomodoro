use std::time;

use pomodoro::{work::Work, PomoMachine, PomoState};

const FIVE_MIN: time::Duration = time::Duration::from_secs(60 * 5);
const TWENTY_FIVE_MIN: time::Duration = time::Duration::from_secs(60 * 25);
const THIRTY_MIN: time::Duration = time::Duration::from_secs(60 * 30);

fn main() {
    let mut state = PomoMachine::Work(PomoState::<Work>::new(
        TWENTY_FIVE_MIN,
        FIVE_MIN,
        THIRTY_MIN,
    ));

    // let mut state = PomoMachine::debug();

    loop {
        state.tick();
        state = state.step();
    }
}
