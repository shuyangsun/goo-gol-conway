use super::preload_prediction::PreloadPrediction;
use std::collections::HashSet;

pub struct AdjacentIndexPrediction {
    forward_size: usize,
    backward_size: usize,
    history_size: usize,
    history: (usize, Vec<usize>), // Keeps track of avg index request, first value sum.
}

impl PreloadPrediction<usize> for AdjacentIndexPrediction {
    fn register(&mut self, value: &usize) {
        self.history.1.push(*value);
        self.history.0 += value;
        if self.history.1.len() > self.history_size {
            let pop = self.history.1.remove(0);
            self.history.0 -= pop;
        }
    }

    fn predict(&self) -> HashSet<usize> {
        let cur_avg = self.cur_avg();
        let sub = std::cmp::min(self.backward_size, cur_avg);
        let start = cur_avg - sub;
        let end = cur_avg + self.forward_size + 1;
        (start..end).collect()
    }
}

impl AdjacentIndexPrediction {
    pub fn new() -> Self {
        let default_history_size = 10;
        Self {
            forward_size: 0usize,
            backward_size: 0usize,
            history_size: default_history_size,
            history: (0, Vec::with_capacity(default_history_size)),
        }
    }

    pub fn with_history_size(self, size: usize) -> Self {
        assert!(self.history.1.is_empty(), "Expected empty history.");
        let mut res = self;
        res.history_size = size;
        res.history = (0, Vec::with_capacity(size));
        res
    }

    pub fn with_forward_size(self, size: usize) -> Self {
        assert!(self.history.1.is_empty(), "Expected empty buffer.");
        let mut res = self;
        res.forward_size = size;
        res
    }

    pub fn with_backward_size(self, size: usize) -> Self {
        assert!(self.history.1.is_empty(), "Expected empty buffer.");
        let mut res = self;
        res.backward_size = size;
        res
    }

    fn buffer_len(&self) -> usize {
        1 + self.forward_size + self.backward_size
    }

    fn cur_avg(&self) -> usize {
        if self.history.1.is_empty() {
            0
        } else {
            self.history.0 / self.history.1.len()
        }
    }
}
