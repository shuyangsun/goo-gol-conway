use std::sync::{Arc, RwLock, RwLockReadGuard, TryLockResult};

pub struct ReadOnlyLock<T> {
    value: Arc<RwLock<T>>,
}

impl<T> ReadOnlyLock<T> {
    pub fn new(value: Arc<RwLock<T>>) -> Self {
        Self { value }
    }

    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<T>> {
        self.value.try_read()
    }
}

impl<T> Clone for ReadOnlyLock<T> {
    fn clone(&self) -> Self {
        Self::new(Arc::clone(&self.value))
    }
}
