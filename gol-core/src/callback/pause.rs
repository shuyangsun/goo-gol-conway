use crate::{BoardCallback, IndexedDataOwned};
use crossbeam_channel::Receiver;
use rayon::prelude::*;

pub struct Pause {
    is_paused: bool,
    rx: Receiver<char>,
}

impl<T, U, I> BoardCallback<T, U, I> for Pause
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: ParallelIterator<Item = IndexedDataOwned<U, T>>,
{
    fn execute(&mut self, _: I) {
        self.check_user_input(false);
        if self.is_paused {
            loop {
                self.check_user_input(true);
                if !self.is_paused {
                    break;
                }
            }
        }
    }
}

impl Pause {
    pub fn new(receiver: Receiver<char>) -> Self {
        Self {
            is_paused: true,
            rx: receiver,
        }
    }

    fn check_user_input(&mut self, should_block: bool) {
        if should_block {
            match self.rx.recv() {
                Ok(val) => self.execute_user_input(val),
                Err(err) => panic!("Error getting user input: {}", err),
            }
        } else {
            eprintln!("Checking...");
            match self.rx.try_recv() {
                Ok(val) => self.execute_user_input(val),
                Err(err) => {
                    if err != crossbeam_channel::TryRecvError::Empty {
                        panic!("Error getting user input: {}", err)
                    }
                }
            }
        }
    }

    fn execute_user_input(&mut self, ch: char) {
        eprintln!("Got {}", ch);
        if ch == ' ' {
            self.is_paused = !self.is_paused;
        }
    }
}
