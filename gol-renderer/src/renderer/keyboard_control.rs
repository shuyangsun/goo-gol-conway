use std::thread;
use tokio::sync::broadcast::{self, error::TryRecvError, Receiver, Sender};

pub struct KeyboardControl {
    sender: Sender<char>,
    receiver: Receiver<char>,
    is_receive_only: bool,
}

impl Clone for KeyboardControl {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = sender.subscribe();
        Self {
            sender,
            receiver,
            is_receive_only: self.is_receive_only,
        }
    }
}

impl KeyboardControl {
    pub fn new() -> Self {
        let (sender, receiver) = broadcast::channel(1);
        Self {
            sender,
            receiver,
            is_receive_only: false,
        }
    }

    pub fn start_monitoring<F>(&self, func: F)
    where
        F: 'static + Send + Sync + Fn() -> char,
    {
        let sender_clone = self.sender.clone();
        thread::spawn(move || loop {
            sender_clone.send(func()).unwrap();
        });
    }

    pub fn is_receive_only(&self) -> bool {
        self.is_receive_only
    }

    pub fn broadcast(&self, ch: char) {
        if self.is_receive_only() {
            panic!("Cannot broadcast keyboard input on receive only control.");
        }
        self.sender.send(ch).unwrap();
    }

    pub fn receive(&mut self) -> char {
        loop {
            let res = self.try_receive();
            match res {
                Some(val) => return val,
                None => continue,
            }
        }
    }

    pub fn try_receive(&mut self) -> Option<char> {
        match self.receiver.try_recv() {
            Ok(val) => Some(val),
            Err(err) => match err {
                TryRecvError::Empty => None,
                TryRecvError::Closed => {
                    eprintln!("Error processing keyboard input: {}", err);
                    None
                }
                TryRecvError::Lagged(_) => None,
            },
        }
    }

    pub fn clone_receive_only(&self) -> Self {
        let sender = self.sender.clone();
        let receiver = sender.subscribe();
        Self {
            sender,
            receiver,
            is_receive_only: true,
        }
    }
}
