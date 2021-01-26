use gol_core::BoardCallbackWithoutStates;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::{error::TryRecvError, Receiver};

pub struct Delay {
    last_execution: Instant,
    duration: Duration,
    rx: Option<Receiver<char>>,
}

impl<T, U> BoardCallbackWithoutStates<T, U> for Delay
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
{
    fn execute(&mut self) {
        let old_execution = self.last_execution;
        self.check_user_input();
        // duration.is_zero() is unstable
        while Instant::now() < old_execution + self.duration {
            self.check_user_input();
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

    pub fn new_with_ch_receiver(duration: Duration, rx: Receiver<char>) -> Self {
        Self {
            last_execution: Instant::now(),
            duration,
            rx: Some(rx),
        }
    }

    fn check_user_input(&mut self) {
        if self.rx.is_some() {
            loop {
                match self.rx.as_mut().unwrap().try_recv() {
                    Ok(val) => {
                        if val == 'k' {
                            self.duration =
                                Duration::from_nanos(self.duration.as_nanos() as u64 / 2);
                        } else if val == 'j' {
                            self.duration =
                                Duration::from_nanos(self.duration.as_nanos() as u64 * 2);
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
}
