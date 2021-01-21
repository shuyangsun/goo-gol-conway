use crate::{BoardCallback, IndexedDataOwned};
use crossbeam_channel::{Receiver, Sender};
use rayon::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

pub struct Delay {
    last_execution: Instant,
    duration: Duration,
    tx: Option<Sender<char>>,
    rx: Option<Receiver<char>>,
}

impl<T, U, I> BoardCallback<T, U, I> for Delay
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: ParallelIterator<Item = IndexedDataOwned<U, T>>,
{
    fn execute(&mut self, _: I) {
        let old_execution = self.last_execution;
        self.last_execution = Instant::now();
        self.check_user_input();
        // duration.is_zero() is unstable
        if self.duration.as_nanos() > 0 {
            let diff = Instant::now() - old_execution;
            if diff < self.duration {
                thread::sleep(self.duration - diff);
            }
        }
    }
}

impl Delay {
    pub fn new(duration: Duration) -> Self {
        Self {
            last_execution: Instant::now(),
            duration,
            rx: None,
        }
    }

    pub fn new_with_ch_receiver(duration: Duration, tx: Sender<char>, rx: Receiver<char>) -> Self {
        Self {
            last_execution: Instant::now(),
            duration,
            rx: Some(receiver),
        }
    }

    fn check_user_input(&mut self) {
        if self.rx.is_some() {
            match self.rx.as_ref().unwrap().try_recv() {
                Ok(val) => {
                    if val == 'k' {
                        self.duration = Duration::from_nanos(self.duration.as_nanos() as u64 / 2);
                    } else if val == 'j' {
                        self.duration = Duration::from_nanos(self.duration.as_nanos() as u64 * 2);
                    }
                }
                Err(err) => {
                    if err != crossbeam_channel::TryRecvError::Empty {
                        panic!("Error getting user input: {}", err)
                    }
                }
            }
        }
    }
}
