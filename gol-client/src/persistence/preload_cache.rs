use super::preload_prediction::PreloadPrediction;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub trait PreLoadCacheDelegate<T, U>: Send + Sync {
    fn get(&self, key: &U) -> Option<T>;
}

pub struct PreLoadCache<T, U> {
    predictor: Arc<RwLock<Box<dyn PreloadPrediction<U>>>>,
    delegate: Box<dyn PreLoadCacheDelegate<T, U>>,
    cache: Arc<RwLock<(bool, HashMap<U, Arc<T>>)>>,
}

impl<T, U> PreLoadCache<T, U> {
    pub fn new(
        predictor: Box<dyn PreloadPrediction<U>>,
        delegate: Box<dyn PreLoadCacheDelegate<T, U>>,
    ) -> Self {
        let predictor = Arc::new(RwLock::new(predictor));
        Self {
            predictor,
            delegate,
            cache: Arc::new(RwLock::new((false, HashMap::new()))),
        }
    }

    pub fn get(&self, key: &U) -> Option<Arc<T>>
    where
        U: Clone + Eq + Hash,
    {
        let prediction = self.cur_prediction(key);

        let mut res = None;
        let mut is_updating = false;
        loop {
            let unlocked = self.cache.try_read();
            if unlocked.is_err() {
                continue;
            }
            let cache = unlocked.unwrap();
            is_updating = cache.0;
            if cache.1.is_empty() || !is_updating && cache.1.get(key).is_none() {
                let delegate_res = self.delegate.get(key);
                if delegate_res.is_some() {
                    res = Some(Arc::new(delegate_res.unwrap()));
                    cache.1.insert(key.clone(), Arc::clone(&res.unwrap()));
                }
            }
            break;
        }
        return res;
    }
}

impl<T, U> PreLoadCache<T, U> {
    fn cur_prediction(&self, key: &U) -> HashSet<U> {
        loop {
            let predictor_unlocked = self.predictor.try_write();
            if predictor_unlocked.is_err() {
                continue;
            }
            let mut predictor = predictor_unlocked.unwrap();
            predictor.register(key);
            return predictor.predict();
        }
    }
}
