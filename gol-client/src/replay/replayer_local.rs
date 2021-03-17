use super::super::persistence::batch_deserializer_local::BatchDeserializerLocal;
use gol_core::{BoardCallbackManager, IndexedDataOwned, StatesCallback, StatesReadOnly};
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

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
    idx: usize,
    delay: Duration,
    is_paused: bool,
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
        // TODO: create thread to update model.
        Self {
            idx: 0,
            delay: Duration::new(1, 0),
            is_paused: true,
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
}

impl<T, CI, U> Replay for ReplayerLocal<T, CI, U>
where
    T: 'static + Send + Sync + Clone + DeserializeOwned,
    CI: 'static + Send + Sync + Clone + Eq + Hash + DeserializeOwned,
    U: 'static + Send + Sync + DeserializeOwned,
{
    fn play(&mut self) {
        self.is_paused = false;
    }

    fn pause(&mut self) {
        self.is_paused = true;
    }

    fn get_delay(&self) -> Duration {
        self.delay
    }

    fn set_delay(&mut self, delay: Duration) {
        self.delay = delay;
    }

    fn get_idx(&self) -> usize {
        self.idx
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
            self.idx = idx;
        } else {
            if idx > 0 {
                self.set_idx(0);
            }
            self.is_paused = true;
        }
    }
}
