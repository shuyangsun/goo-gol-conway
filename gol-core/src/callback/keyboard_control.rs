use crate::{BoardCallback, IndexedDataOwned};
use crossbeam_channel::{unbounded, Receiver};
use rayon::prelude::*;
use std::char;
use std::io::Read;
use std::thread;

pub struct KeyboardControl {
    rx: Receiver<char>,
}

impl<T, U, I> BoardCallback<T, U, I> for KeyboardControl
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: ParallelIterator<Item = IndexedDataOwned<U, T>>,
{
    fn execute(&mut self, _: I) {}
}

impl KeyboardControl {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        thread::spawn(move || {
            let mut reader = std::io::stdin();
            let mut buffer = [0u8; 1];

            loop {
                reader.read_exact(&mut buffer).unwrap();
                let ch = char::from_u32(buffer[0] as u32).unwrap();
                tx.send(ch).unwrap();
            }
        });
        Self { rx }
    }

    pub fn get_receiver(&self) -> Receiver<char> {
        self.rx.clone()
    }
}
