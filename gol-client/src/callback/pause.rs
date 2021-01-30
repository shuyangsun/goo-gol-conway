use gol_core::BoardCallbackWithoutStates;
use gol_renderer::renderer::keyboard_control::KeyboardControl;

pub struct Pause {
    is_paused: bool,
    control: KeyboardControl,
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
    pub fn new(is_paused: bool, control: KeyboardControl) -> Self {
        Self { is_paused, control }
    }

    fn check_user_input(&mut self, should_block: bool) {
        if should_block {
            let ch = self.control.receive();
            self.execute_user_input(ch);
        }
        loop {
            match self.control.try_receive() {
                Some(val) => {
                    self.execute_user_input(val);
                    break;
                }
                None => break,
            }
        }
    }

    fn execute_user_input(&mut self, ch: char) {
        if ch == ' ' {
            self.is_paused = !self.is_paused;
        }
    }
}
