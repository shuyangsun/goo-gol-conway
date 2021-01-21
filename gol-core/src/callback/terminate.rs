use crate::{BoardCallback, IndexedDataOwned};
use crossbeam_channel::Receiver;
use rayon::prelude::*;

pub struct Terminate {
    rx: Receiver<char>,
}

impl<T, U, I> BoardCallback<T, U, I> for Terminate
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: ParallelIterator<Item = IndexedDataOwned<U, T>>,
{
    fn execute(&mut self, _: I) {
        match self.rx.try_recv() {
            Ok(val) => {
                if val == 'q' {
                    std::process::exit(0);
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

impl Terminate {
    pub fn new(receiver: Receiver<char>) -> Self {
        Self { rx: receiver }
    }
}
