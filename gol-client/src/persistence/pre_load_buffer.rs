use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

pub struct PreLoadBuffer<T> {
    forward_size: usize,
    backward_size: usize,
    buffer: Arc<RwLock<Vec<(usize, Arc<T>)>>>,
    buffer_update: Arc<RwLock<Vec<(usize, JoinHandle<T>)>>>,
    history_size: usize,
    history: Arc<RwLock<(usize, Vec<usize>)>>, // Keeps track of avg index request, first value sum.
}

pub trait PreLoadBufferDelegate<T> {
    fn raw_get(&self, index: usize) -> Option<T>;
}

impl<T> PreLoadBuffer<T> {
    pub fn new() -> Self {
        let default_history_size = 10;
        Self {
            forward_size: 0usize,
            backward_size: 0usize,
            buffer: Arc::new(RwLock::new(Vec::with_capacity(1))),
            buffer_update: Arc::new(RwLock::new(Vec::new())),
            history_size: default_history_size,
            history: Arc::new(RwLock::new((0, Vec::with_capacity(default_history_size)))),
        }
    }

    pub fn with_history_size(self, size: usize) -> Self {
        assert!(
            self.history.read().unwrap().1.is_empty(),
            "Expected empty history."
        );
        let mut res = self;
        res.history_size = size;
        res.history = Arc::new(RwLock::new((0, Vec::with_capacity(size))));
        res
    }

    pub fn with_forward_size(self, size: usize) -> Self {
        assert!(
            self.buffer.read().unwrap().is_empty(),
            "Expected empty buffer."
        );
        let mut res = self;
        res.forward_size = size;
        let buf_len = 1 + size + res.backward_size;
        res.buffer = Arc::new(RwLock::new(Vec::with_capacity(buf_len)));
        res
    }

    pub fn with_backward_size(self, size: usize) -> Self {
        assert!(
            self.buffer.read().unwrap().is_empty(),
            "Expected empty buffer."
        );
        let mut res = self;
        res.backward_size = size;
        let buf_len = 1 + size + res.forward_size;
        res.buffer = Arc::new(RwLock::new(Vec::with_capacity(buf_len)));
        res
    }

    pub fn get(&self, idx: usize) -> Option<Arc<T>> {
        let avg_idx = self.updated_avg(idx);
        loop {
            let unlocked = self.buffer.try_read();
            if unlocked.is_err() {
                continue;
            }
            let buffer = unlocked.unwrap();
            if buffer.is_empty() {}
            let buf_beg_idx = buffer.first().unwrap();
        }
    }
}

impl<T> PreLoadBuffer<T> {
    fn updated_avg(&self, idx: usize) -> usize {
        loop {
            match self.history.try_write() {
                Ok(mut guard) => {
                    guard.1.push(idx);
                    guard.0 += idx;
                    if guard.1.len() > self.history_size {
                        let pop = guard.1.remove(0);
                        guard.0 -= pop;
                    }
                    return guard.0 / guard.1.len();
                }
                Err(_) => continue,
            }
        }
    }

    fn schedule_buffer_update(
        &self,
        delegate: &dyn PreLoadBufferDelegate<T>,
        start: usize,
        end: usize,
    ) {
    }

    fn buffer_len(&self) -> usize {
        1 + self.forward_size + self.backward_size
    }

    fn find_range_left_idx(idx: &usize, ranges: &Vec<usize>) -> Option<usize> {
        match ranges.binary_search(idx) {
            Ok(val) => {
                if val >= ranges.len() - 1 {
                    None
                } else {
                    Some(val)
                }
            }
            Err(val) => {
                if val > 0 && val < ranges.len() - 1 {
                    Some(val - 1)
                } else {
                    None
                }
            }
        }
    }
}
