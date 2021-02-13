use super::preload_prediction::PreloadPrediction;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

pub struct PreLoadBuffer<T> {
    predictor: Arc<RwLock<Box<dyn PreloadPrediction<usize>>>>,
    buffer: Arc<RwLock<HashMap<usize, Arc<T>>>>,
}

pub trait PreLoadBufferDelegate<T> {
    fn raw_get(&self, index: usize) -> Option<T>;
}

impl<T> PreLoadBuffer<T> {
    pub fn new(predictor: Box<dyn PreloadPrediction<usize>>) -> Self {
        let predictor = Arc::new(RwLock::new(predictor));
        Self {
            predictor,
            buffer: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, idx: usize) -> Option<Arc<T>> {
        let mut prediction: HashSet<usize>;
        loop {
            let predictor_unlocked = self.predictor.try_read();
            if predictor_unlocked.is_err() {
                continue;
            }
            let predictor = predictor_unlocked.unwrap();
            predictor.as_mut().register(&idx);
            prediction = predictor.as_ref().predict();
            break;
        }

        loop {
            let unlocked = self.buffer.try_read();
            if unlocked.is_err() {
                continue;
            }
            let buffer = unlocked.unwrap();
            if buffer.is_empty() {}
        }
    }
}

impl<T> PreLoadBuffer<T> {}
