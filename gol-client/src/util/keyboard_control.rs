use std::char;
use std::io::Read;
use std::thread;
use tokio::sync::broadcast::{self, Receiver, Sender};

pub struct KeyboardControl {
    tx: Sender<char>,
}

impl KeyboardControl {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
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
        Self { tx }
    }

    pub fn get_receiver(&self) -> Receiver<char> {
        self.tx.subscribe()
    }
}
