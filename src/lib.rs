use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    time,
};

use large_break::LargeBreak;
use normal_break::Break;
use rodio::{Decoder, OutputStream, Sink};
use work::Work;

pub mod large_break;
pub mod normal_break;
pub mod work;

pub enum PomoMachine {
    Work(PomoState<Work>),
    Break(PomoState<Break>),
    LargeBreak(PomoState<LargeBreak>),
}
impl PomoMachine {
    pub fn tick(&mut self) {
        match self {
            PomoMachine::Work(r) => r.tick(),
            PomoMachine::Break(r) => r.tick(),
            PomoMachine::LargeBreak(r) => r.tick(),
        }
    }
    pub fn step(self) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        #[cfg(debug_assertions)]
        let mut resource_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "res"].iter().collect();
        #[cfg(not(debug_assertions))]
        let mut resource_dir: PathBuf = "./res".into();

        resource_dir.push("hellnaw.wav");

        let file = BufReader::new(File::open(resource_dir).unwrap());
        let source = Decoder::new(file).unwrap();

        sink.append(source);

        println!("nice work! type `ok!` when you're ready!");
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        sink.stop();
        match self {
            PomoMachine::Break(val) => PomoMachine::Work(val.into()),
            PomoMachine::Work(val) => {
                if val.global.num_of_work == 4 {
                    PomoMachine::LargeBreak(val.into())
                } else {
                    PomoMachine::Break(val.into())
                }
            }
            PomoMachine::LargeBreak(val) => PomoMachine::Work(val.into()),
        }
    }
    pub fn debug() -> PomoMachine {
        let work_time = time::Duration::from_secs(1);
        PomoMachine::Work(PomoState {
            global: Global {
                num_of_work: 0,
                total_work: 4,
                work_time,
                break_time: time::Duration::from_secs(1),
                large_break_time: time::Duration::from_secs(2),
            },
            state: Work::new(work_time),
        })
    }
}

pub struct Global {
    num_of_work: u8,
    total_work: u8,
    work_time: time::Duration,
    break_time: time::Duration,
    large_break_time: time::Duration,
}
pub struct PomoState<T> {
    global: Global,
    state: T,
}

impl<T> PomoState<T> {
    pub fn new(
        work_time: time::Duration,
        break_time: time::Duration,
        large_break_time: time::Duration,
    ) -> PomoState<Work> {
        PomoState {
            global: Global {
                num_of_work: 0,
                total_work: 4,
                work_time,
                break_time,
                large_break_time,
            },
            state: Work::new(work_time),
        }
    }
}
