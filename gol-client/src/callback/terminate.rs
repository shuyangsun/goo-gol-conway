use gol_core::BoardCallbackWithoutStates;
use gol_renderer::renderer::keyboard_control::KeyboardControl;

pub struct Terminate {
    control: KeyboardControl,
}

impl<T, U> BoardCallbackWithoutStates<T, U> for Terminate
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
{
    fn execute(&mut self) {
        loop {
            match self.control.try_receive() {
                Some(val) => {
                    if val == 'q' {
                        // TODO: Hacky solution to give other callbacks a time buffer to do cleanup.
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        std::process::exit(0);
                    }
                    break;
                }
                None => break,
            }
        }
    }
}

impl Terminate {
    pub fn new(control: KeyboardControl) -> Self {
        Self { control }
    }
}
