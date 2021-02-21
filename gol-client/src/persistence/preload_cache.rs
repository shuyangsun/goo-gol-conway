use super::preload_prediction::PreloadPrediction;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

pub trait PreloadCacheDelegate<T, U>: Send + Sync {
    fn get(&self, key: &T) -> Option<U>;
}

pub struct PreloadCache<T, U> {
    predictor: Arc<RwLock<Box<dyn PreloadPrediction<T>>>>,
    delegate: Arc<Box<dyn PreloadCacheDelegate<T, U>>>,
    cache: Arc<RwLock<HashMap<T, Arc<U>>>>,
    update: Arc<RwLock<Option<JoinHandle<(HashSet<T>, HashMap<T, Arc<U>>)>>>>,
}

impl<T, U> PreloadCache<T, U> {
    pub fn new(
        predictor: Box<dyn PreloadPrediction<T>>,
        delegate: Box<dyn PreloadCacheDelegate<T, U>>,
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

    pub fn get(&self, key: &T) -> Option<Arc<U>>
    where
        T: 'static + Send + Sync + Clone + Eq + Hash,
        U: 'static + Send + Sync,
    {
        let prediction = self.cur_prediction(key);

        let mut res = None;
        let mut should_insert_res = false;
        let mut cur_keys: HashSet<T>;
        loop {
            let unlocked = self.cache.try_read();
            if unlocked.is_err() {
                continue;
            }
            let cache = unlocked.unwrap();

            let cache_res = cache.get(key);

            if cache_res.is_none() && self.is_updating() {
                drop(cache);
                self.wait_for_update();
                continue;
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

        if !self.is_updating() {
            let extra = cur_keys.difference(&prediction).cloned().collect();
            let to_be_added: HashSet<T> = prediction.difference(&cur_keys).cloned().collect();

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
                        match delegate.get(&key) {
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

impl<T, U> PreloadCache<T, U> {
    fn wait_for_update(&self)
    where
        T: Eq + Hash,
    {
        eprintln!();
        loop {
            eprintln!("{:?}: Waiting for update...", thread::current().id());
            let unlocked = self.update.try_write();
            if unlocked.is_err() {
                continue;
            }
            let mut unlocked = unlocked.unwrap();
            if unlocked.is_some() {
                let update = unlocked.take().unwrap();
                let update_res = update.join().unwrap();
                let (extra, appending) = update_res;
                loop {
                    eprintln!("{:?}: Waiting for cache...", thread::current().id());
                    let cache_unlocked = self.cache.try_write();
                    if cache_unlocked.is_err() {
                        continue;
                    }
                    let mut cache = cache_unlocked.unwrap();
                    eprintln!("{:?}: Unlocked cache...", thread::current().id());
                    for key in extra.iter() {
                        cache.remove_entry(key);
                    }
                    cache.extend(appending);
                    break;
                }
            }
            *unlocked = None;
            eprintln!("Update finished.");
            break;
        }
    }

    fn cur_prediction(&self, key: &T) -> HashSet<T> {
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

    fn is_updating(&self) -> bool {
        eprintln!();
        loop {
            eprintln!("{:?}: Reading update...", thread::current().id());
            let update_unlocked = self.update.try_read();
            if update_unlocked.is_err() {
                continue;
            }
            return update_unlocked.unwrap().is_some();
        }
    }
}

#[cfg(test)]
mod preload_cache_test {
    use super::super::adjacent_index_prediction::AdjacentIndexPrediction;
    use super::{PreloadCache, PreloadCacheDelegate};
    use std::sync::Arc;
    use std::thread;

    struct VecWrapper<T> {
        vec: Vec<T>,
    }

    impl PreloadCacheDelegate<usize, usize> for VecWrapper<usize> {
        fn get(&self, key: &usize) -> Option<usize> {
            self.vec.get(*key).cloned()
        }
    }

    #[test]
    fn preload_cache_test_1() {
        let predictor = Box::new(
            AdjacentIndexPrediction::new()
                .with_history_size(10)
                .with_forward_size(3)
                .with_backward_size(1),
        );
        let delegate = Box::new(VecWrapper {
            vec: (0..100).collect(),
        });
        let preload_cache = PreloadCache::new(predictor, delegate);
        for i in 0usize..100 {
            assert_eq!(*preload_cache.get(&i).unwrap(), i);
        }
        for i in 100usize..150 {
            assert!(preload_cache.get(&i).is_none());
        }
        for i in (0usize..100).rev() {
            assert_eq!(*preload_cache.get(&i).unwrap(), i);
        }
    }

    fn preload_cache_test_multithread_1() {
        let predictor = Box::new(
            AdjacentIndexPrediction::new()
                .with_history_size(10)
                .with_forward_size(5)
                .with_backward_size(2),
        );
        let delegate = Box::new(VecWrapper {
            vec: (0..100).collect(),
        });
        let preload_cache = Arc::new(PreloadCache::new(predictor, delegate));
        let mut handles = Vec::new();
        let cache_clone_1 = Arc::clone(&preload_cache);
        handles.push(thread::spawn(move || {
            for i in 0usize..100 {
                assert_eq!(*cache_clone_1.get(&i).unwrap(), i);
            }
        }));
        for i in 100usize..150 {
            assert!(preload_cache.get(&i).is_none());
        }
        for i in (0usize..100).rev() {
            assert_eq!(*preload_cache.get(&i).unwrap(), i);
        }
        for handle in handles.into_iter() {
            handle.join().unwrap();
        }
    }
}
