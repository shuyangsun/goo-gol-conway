use crate::{BoardCallback, IndexedDataOwned};
use crossbeam_channel::Receiver;
use rayon::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

pub struct Delay {
    last_execution: Instant,
    duration: Duration,
    rx: Option<Receiver<char>>,
}

impl<T, U, I> BoardCallback<T, U, I> for Delay
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: ParallelIterator<Item = IndexedDataOwned<U, T>>,
{
    fn execute(&mut self, _: I) {
        self.check_user_input();
        // duration.is_zero() is unstable
        if self.duration.as_nanos() > 0 {
            let diff = Instant::now() - self.last_execution;
            if diff < self.duration {
                thread::sleep(self.duration - diff);
            }
        }
        self.last_execution = Instant::now();
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

    pub fn new_with_ch_receiver(duration: Duration, receiver: Receiver<char>) -> Self {
        Self {
            last_execution: Instant::now(),
            duration,
            rx: Some(receiver),
        }
    }

    fn check_user_input(&mut self) {
        if self.rx.is_some() {
            let rx_new = self.rx.as_ref().unwrap().clone();
            match rx_new.try_recv() {
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
