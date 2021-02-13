use super::preload_prediction::PreloadPrediction;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

pub trait PreLoadCacheDelegate<T, U>: Send + Sync {
    fn get(&self, key: &U) -> Option<T>;
}

pub struct PreLoadCache<T, U> {
    predictor: Arc<RwLock<Box<dyn PreloadPrediction<U>>>>,
    delegate: Arc<Box<dyn PreLoadCacheDelegate<T, U>>>,
    cache: Arc<RwLock<HashMap<U, Arc<T>>>>,
    update: Arc<RwLock<Option<JoinHandle<(HashSet<U>, HashMap<U, Arc<T>>)>>>>,
}

impl<T, U> PreLoadCache<T, U> {
    pub fn new(
        predictor: Box<dyn PreloadPrediction<U>>,
        delegate: Box<dyn PreLoadCacheDelegate<T, U>>,
    ) -> Self {
        let predictor = Arc::new(RwLock::new(predictor));
        let delegate = Arc::new(delegate);
        Self {
            predictor,
            delegate,
            cache: Arc::new(RwLock::new(HashMap::new())),
            update: Arc::new(RwLock::new(None)),
        }
    }

    pub fn get(&self, key: &U) -> Option<Arc<T>>
    where
        T: 'static + Send + Sync,
        U: 'static + Send + Sync + Clone + Eq + Hash,
    {
        let prediction = self.cur_prediction(key);

        let mut res = None;
        let mut should_insert_res = false;
        let mut cur_keys = HashSet::new();
        loop {
            let unlocked = self.cache.try_read();
            if unlocked.is_err() {
                continue;
            }
            let cache = unlocked.unwrap();

            let cache_res = cache.get(key);

            let is_updating = self.update.read().unwrap().is_some();
            if is_updating && cache_res.is_none() {
                self.wait_for_update();
                return self.get(key);
            } else if cache_res.is_some() {
                res = Some(Arc::clone(cache_res.unwrap()));
            } else {
                let delegate_res = self.delegate.get(key);
                if delegate_res.is_some() {
                    res = Some(Arc::new(delegate_res.unwrap()));
                    should_insert_res = true;
                }
            }

            cur_keys = cache.keys().cloned().collect();

            break;
        }

        if res.is_some() && should_insert_res {
            loop {
                let unlocked = self.cache.try_write();
                if unlocked.is_err() {
                    continue;
                }
                let mut cache = unlocked.unwrap();
                if cache.get(key).is_none() {
                    cache.insert(key.clone(), Arc::clone(&(res.as_ref().unwrap())));
                    cur_keys.insert(key.clone());
                }
                break;
            }
        }

        let is_updating = self.update.read().unwrap().is_some();
        if !is_updating {
            let extra = cur_keys.difference(&prediction).cloned().collect();
            let to_be_added = prediction.difference(&cur_keys);

            loop {
                let unlocked = self.update.try_write();
                if unlocked.is_err() {
                    continue;
                }
                let mut update = unlocked.unwrap();
                let delegate = Arc::clone(&self.delegate);
                *update = Some(thread::spawn(move || {
                    let mut appending = HashMap::new();
                    for key in to_be_added {
                        match delegate.get(key) {
                            Some(val) => {
                                appending.insert(key.clone(), Arc::new(val));
                            }
                            None => continue,
                        }
                    }
                    (extra, appending)
                }));
                break;
            }
        }

        res
    }
}

impl<T, U> PreLoadCache<T, U> {
    fn wait_for_update(&self) {
        loop {
            let unlocked = self.update.try_read();
            if unlocked.is_err() {
                continue;
            }
            if unlocked.unwrap().is_some() {
                continue;
            }
            break;
        }
    }

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
