use gol_core::BoardCallbackWithoutStates;
use gol_renderer::renderer::keyboard_control::KeyboardControl;
use std::time::{Duration, Instant};

pub struct Delay {
    last_execution: Instant,
    duration: Duration,
    control: Option<KeyboardControl>,
}

impl<T, U> BoardCallbackWithoutStates<T, U> for Delay
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
{
    fn execute(&mut self) {
        let old_execution = self.last_execution;
        self.check_user_input();
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
            control: None,
        }
    }

    pub fn with_ch_receiver(self, control: KeyboardControl) -> Self {
        let mut res = self;
        res.control = Some(control);
        res
    }

    fn check_user_input(&mut self) {
        if self.control.is_none() {
            return;
        }
        loop {
            match self.control.as_mut().unwrap().try_receive() {
                Some(ch) => {
                    let duration = if self.duration.as_nanos() == 0 {
                        Duration::from_millis(1)
                    } else {
                        self.duration
                    };
                    if ch == 'k' {
                        self.duration = Duration::from_nanos(duration.as_nanos() as u64 / 2);
                        println!("K: {:?}", self.duration);
                    } else if ch == 'j' {
                        self.duration = Duration::from_nanos(duration.as_nanos() as u64 * 2);
                        println!("J: {:?}", self.duration);
                    }
                    break;
                }
                None => break,
            }
        }
    }
}
