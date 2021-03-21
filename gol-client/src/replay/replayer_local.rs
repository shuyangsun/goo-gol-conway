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

struct IndexedStates<T, CI, U>
where
    T: Send + Sync,
    CI: Send + Sync + Hash,
{
    idx: usize,
    deserializer: BatchDeserializerLocal<U, Vec<IndexedDataOwned<CI, T>>>,
    states: StatesCallback<CI, T>,
    board_shape: U,
}

impl<T, CI, U> IndexedStates<T, CI, U>
where
    T: 'static + Send + Sync + Clone + DeserializeOwned,
    CI: 'static + Send + Sync + Clone + Eq + Hash + DeserializeOwned,
    U: 'static + Send + Sync + Clone + DeserializeOwned,
{
    pub fn new(trivial_state: T, deserializer_path: &String) -> Self {
        let deserializer: BatchDeserializerLocal<U, Vec<IndexedDataOwned<CI, T>>> =
            BatchDeserializerLocal::new(deserializer_path);
        let initial_states = deserializer.get(0).expect("No data at index 0.");
        let board_shape = initial_states
            .0
            .as_ref()
            .clone()
            .expect("No board shape at index 0.");
        let initial_states = initial_states
            .1
             .1
            .par_iter()
            .map(|ele| (ele.0.clone(), ele.1.clone()))
            .collect();

        let mut states = StatesCallback::new(trivial_state);
        states.set_non_trivial_lookup(initial_states);

        Self {
            idx: 0,
            deserializer,
            states,
            board_shape,
        }
    }

    pub fn get_board_shape(&self) -> &U {
        &self.board_shape
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }

    pub fn set_idx(&mut self, idx: usize) -> bool {
        if self.get_idx() == idx {
            return true;
        }

        if let Some(data) = self.deserializer.get(idx) {
            let (_, idx_states) = data;
            assert_eq!(idx_states.0, idx);
            let new_states: HashMap<CI, T> = idx_states
                .1
                .par_iter()
                .map(|ele| (ele.0.clone(), ele.1.clone()))
                .collect();
            self.states.set_non_trivial_lookup(new_states);
            self.idx = idx;
            return true;
        }

        return false;
    }

    pub fn get_readonly_states(&self) -> StatesReadOnly<CI, T>
    where
        T: Clone,
    {
        self.states.clone_read_only()
    }
}

pub struct ReplayerLocal<T, CI, U>
where
    T: Send + Sync,
    CI: Send + Sync + Hash,
{
    states: Arc<RwLock<IndexedStates<T, CI, U>>>,
    delay: Arc<RwLock<Duration>>,
    is_paused: Arc<RwLock<bool>>,
}

impl<T, CI, U> ReplayerLocal<T, CI, U>
where
    T: Send + Sync,
    CI: Send + Sync + Hash,
{
    pub fn new(trivial_state: T, history_path: &String) -> Self
    where
        T: 'static + Clone + DeserializeOwned,
        CI: 'static + Clone + Eq + DeserializeOwned,
        U: 'static + Send + Sync + Clone + DeserializeOwned,
    {
        let states = Arc::new(RwLock::new(IndexedStates::new(trivial_state, history_path)));
        let delay = Arc::new(RwLock::new(Duration::new(1, 0)));
        let is_paused = Arc::new(RwLock::new(true));

        let states_clone = Arc::clone(&states);
        let delay_clone = Arc::clone(&delay);
        let is_paused_clone = Arc::clone(&is_paused);

        std::thread::spawn(move || {
            let mut last_update = Instant::now();

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

                    let cur_idx;
                    loop {
                        let unlocked = states_clone.try_read();
                        if unlocked.is_err() {
                            continue;
                        }
                        cur_idx = unlocked.ok().unwrap().get_idx();
                        break;
                    }

                    loop {
                        let unlocked = states_clone.try_write();
                        if unlocked.is_err() {
                            continue;
                        }

                        loop {
                            let delay = delay_clone.try_read();
                            if unlocked.is_err() {
                                continue;
                            }
                            let delay = delay.ok().unwrap();
                            if Instant::now() - last_update >= delay.clone() {
                                unlocked.ok().unwrap().set_idx(cur_idx + 1);
                                last_update = Instant::now();
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        });

        Self {
            states,
            delay,
            is_paused,
        }
    }

    pub fn get_readonly_states(&self) -> StatesReadOnly<CI, T>
    where
        T: 'static + Clone + DeserializeOwned,
        CI: 'static + Clone + Eq + DeserializeOwned,
        U: 'static + Send + Sync + Clone + DeserializeOwned,
    {
        loop {
            let unlocked = self.states.try_read();
            if unlocked.is_err() {
                continue;
            }
            return unlocked.unwrap().get_readonly_states();
        }
    }

    pub fn get_board_shape(&self) -> U
    where
        T: 'static + Clone + DeserializeOwned,
        CI: 'static + Clone + Eq + DeserializeOwned,
        U: 'static + Send + Sync + Clone + DeserializeOwned,
    {
        loop {
            let unlocked = self.states.try_read();
            if unlocked.is_err() {
                continue;
            }
            return unlocked.unwrap().get_board_shape().clone();
        }
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
    U: 'static + Send + Sync + Clone + DeserializeOwned,
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
            let unlocked = self.states.try_read();
            if unlocked.is_err() {
                continue;
            }
            return unlocked.ok().unwrap().get_idx();
        }
    }

    fn set_idx(&mut self, idx: usize) {
        loop {
            let unlocked = self.states.try_write();
            if unlocked.is_err() {
                continue;
            }
            let succeed = unlocked.ok().unwrap().set_idx(idx);
            if !succeed {
                self.set_paused(true);
            }
            return;
        }
    }
}
