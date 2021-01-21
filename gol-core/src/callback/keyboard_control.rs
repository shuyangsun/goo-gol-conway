use crate::{BoardCallback, IndexedDataOwned};
use crossbeam_channel::{bounded, Receiver, Sender};
use rayon::prelude::*;
use std::char;
use std::io::Read;
use std::thread;

pub struct KeyboardControl {
    tx: Sender<char>,
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
        let (tx, rx) = bounded(0);
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let mut reader = std::io::stdin();
            let mut buffer = [0u8; 1];

            loop {
                reader.read_exact(&mut buffer).unwrap();
                let ch = char::from_u32(buffer[0] as u32).unwrap();
                tx_clone.send(ch).unwrap();
            }
        });
        Self { tx, rx }
    }

    pub fn get_channel(&self) -> (Sender<char>, Receiver<char>) {
        (self.tx.clone(), self.rx.clone())
    }
}
