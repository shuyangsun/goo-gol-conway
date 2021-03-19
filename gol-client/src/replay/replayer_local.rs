use super::super::persistence::batch_deserializer_local::BatchDeserializerLocal;
use gol_core::{BoardCallbackManager, IndexedDataOwned, StatesCallback, StatesReadOnly};
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub trait Replay {
    fn play(&mut self);
    fn pause(&mut self);

    fn get_delay(&self) -> Duration;
    fn set_delay(&mut self, delay: Duration);

    fn get_idx(&self) -> usize;
    fn set_idx(&mut self, idx: usize);

    fn forward(&mut self) {
        self.set_idx(self.get_idx() + 1);
    }

    fn backward(&mut self) {
        let cur_idx = self.get_idx();
        if cur_idx > 0 {
            self.set_idx(cur_idx - 1);
        }
    }
}

pub struct ReplayerLocal<T, CI, U>
where
    T: Send + Sync,
    CI: Send + Sync + Hash,
{
    idx: Arc<RwLock<usize>>,
    delay: Arc<RwLock<Duration>>,
    is_paused: Arc<RwLock<bool>>,
    deserializer: BatchDeserializerLocal<U, Vec<IndexedDataOwned<CI, T>>>,
    states: StatesCallback<CI, T>,
}

impl<T, CI, U> ReplayerLocal<T, CI, U>
where
    T: Send + Sync,
    CI: Send + Sync + Hash,
{
    pub fn new(trivial_state: T, history_path: &String) -> Self
    where
        T: 'static + Clone + DeserializeOwned,
        CI: 'static + Clone + DeserializeOwned,
        U: Send + Sync + DeserializeOwned,
    {
        let deserializer = BatchDeserializerLocal::new(history_path);
        let states = StatesCallback::new(trivial_state);
        let idx = Arc::new(RwLock::new(0));
        let delay = Arc::new(RwLock::new(Duration::new(1, 0)));
        let is_paused = Arc::new(RwLock::new(true));

        let idx_clone = Arc::clone(&idx);
        let delay_clone = Arc::clone(&delay);
        let is_paused_clone = Arc::clone(&is_paused);

        std::thread::spawn(move || {
            let last_update = Instant::now();

            loop {
                let is_paused;
                loop {
                    let unlocked = is_paused_clone.try_read();
                    if unlocked.is_err() {
                        continue;
                    }
                    is_paused = *unlocked.unwrap();
                    break;
                }

                if !is_paused {
                    let unlocked = delay_clone.try_read();
                    if unlocked.is_err() {
                        continue;
                    }
                    let cur_delay = *unlocked.unwrap();
                    let now = Instant::now();
                    if now - last_update < cur_delay {
                        continue;
                    }
                }
            }
        });

        Self {
            idx,
            delay,
            is_paused,
            deserializer,
            states,
        }
    }

    pub fn get_readonly_states(&self) -> StatesReadOnly<CI, T>
    where
        T: Clone,
    {
        self.states.clone_read_only()
    }

    pub fn get_board_shape(&self) -> U
    where
        T: 'static + Send + Sync + DeserializeOwned,
        CI: 'static + Send + Sync + DeserializeOwned,
        U: 'static + Send + Sync + Clone + DeserializeOwned,
    {
        if let Some(data) = self.deserializer.get(0) {
            return data
                .0
                .as_ref()
                .clone()
                .expect("Cannot find board shape header at index 0.");
        }
        panic!("Cannot find board history at index 0.");
    }

    fn update_states(&mut self, states: HashMap<CI, T>) {
        self.states.set_non_trivial_lookup(states)
    }

    fn set_paused(&mut self, is_paused: bool) {
        loop {
            let unlocked = self.is_paused.try_write();
            if unlocked.is_err() {
                continue;
            }
            *unlocked.unwrap() = is_paused;
        }
    }
}

impl<T, CI, U> Replay for ReplayerLocal<T, CI, U>
where
    T: 'static + Send + Sync + Clone + DeserializeOwned,
    CI: 'static + Send + Sync + Clone + Eq + Hash + DeserializeOwned,
    U: 'static + Send + Sync + DeserializeOwned,
{
    fn play(&mut self) {
        self.set_paused(false);
    }

    fn pause(&mut self) {
        self.set_paused(true);
    }

    fn get_delay(&self) -> Duration {
        loop {
            let unlocked = self.delay.try_read();
            if unlocked.is_err() {
                continue;
            }
            return unlocked.unwrap().clone();
        }
    }

    fn set_delay(&mut self, delay: Duration) {
        loop {
            let unlocked = self.delay.try_write();
            if unlocked.is_err() {
                continue;
            }
            *unlocked.unwrap() = delay;
        }
    }

    fn get_idx(&self) -> usize {
        loop {
            let unlocked = self.idx.try_read();
            if unlocked.is_err() {
                continue;
            }
            return unlocked.unwrap().clone();
        }
    }

    fn set_idx(&mut self, idx: usize) {
        if self.get_idx() == idx {
            return;
        }

        if let Some(data) = self.deserializer.get(idx) {
            let (_, idx_states) = data;
            assert_eq!(idx_states.0, idx);
            let new_states: HashMap<CI, T> = idx_states
                .1
                .par_iter()
                .map(|ele| (ele.0.clone(), ele.1.clone()))
                .collect();
            self.update_states(new_states);
            loop {
                let unlocked = self.idx.try_write();
                if unlocked.is_err() {
                    continue;
                }
                *unlocked.unwrap() = idx;
            }
        } else {
            if idx > 0 {
                self.set_idx(0);
            }
            self.set_paused(true);
        }
    }
}
