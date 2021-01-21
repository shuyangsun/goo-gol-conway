use crate::{BoardCallback, IndexedDataOwned};
use rayon::prelude::*;
use tokio::sync::broadcast::{error::TryRecvError, Receiver};

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
        loop {
            match self.rx.try_recv() {
                Ok(val) => {
                    self.execute_user_input(val);
                    break;
                }
                Err(err) => match err {
                    TryRecvError::Empty => {
                        if should_block {
                            continue;
                        } else {
                            break;
                        }
                    }
                    TryRecvError::Closed => panic!("Error getting user input: {}", err),
                    TryRecvError::Lagged(_) => continue,
                },
            }
        }
    }

    fn execute_user_input(&mut self, ch: char) {
        if ch == ' ' {
            self.is_paused = !self.is_paused;
        }
    }
}
