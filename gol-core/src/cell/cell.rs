pub trait Cell<T>: Sync + Send {
    fn new_from_state(state: T) -> Self
    where
        Self: Sized;
    fn state(&self) -> &T;
    fn set_state(&mut self, new_state: T);
}
