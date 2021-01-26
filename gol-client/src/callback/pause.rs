use gol_core::BoardCallbackWithoutStates;
use tokio::sync::broadcast::{error::TryRecvError, Receiver};

pub struct Pause {
    is_paused: bool,
    rx: Receiver<char>,
}

impl<T, U> BoardCallbackWithoutStates<T, U> for Pause
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
{
    fn execute(&mut self) {
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
    pub fn new(is_paused: bool, receiver: Receiver<char>) -> Self {
        Self {
            is_paused,
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
