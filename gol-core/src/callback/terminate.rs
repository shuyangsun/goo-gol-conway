use crate::{BoardCallback, IndexedDataOwned};
use rayon::prelude::*;
use tokio::sync::broadcast::{error::TryRecvError, Receiver};

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
        loop {
            match self.rx.try_recv() {
                Ok(val) => {
                    if val == 'q' {
                        // TODO: Hacky solution to give other callbacks a time buffer to do cleanup.
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        std::process::exit(0);
                    }
                    break;
                }
                Err(err) => match err {
                    TryRecvError::Empty => break,
                    TryRecvError::Closed => panic!("Error getting user input: {}", err),
                    TryRecvError::Lagged(_) => continue,
                },
            }
        }
    }
}

impl Terminate {
    pub fn new(receiver: Receiver<char>) -> Self {
        Self { rx: receiver }
    }
}
