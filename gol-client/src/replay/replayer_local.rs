use super::super::persistence::batch_deserializer_local::BatchDeserializerLocal;
use gol_core::{IndexedDataOwned, StatesCallback, StatesReadOnly};
use gol_renderer::renderer::keyboard_control::KeyboardControl;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub trait Replay {
    fn play(&self);
    fn pause(&self);

    fn get_delay(&self) -> Duration;
    fn set_delay(&mut self, delay: Duration);

    fn get_idx(&self) -> usize;
    fn set_idx(&self, idx: usize);

    fn forward(&self) {
        self.set_idx(self.get_idx() + 1);
    }

    fn backward(&self) {
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
    header: U,
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
        let header = initial_states
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
            header,
        }
    }

    pub fn get_header(&self) -> &U {
        &self.header
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
                            if delay.is_err() {
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

    pub fn with_keyboard_control(self, keyboard_control: KeyboardControl) -> Self
    where
        T: 'static + Clone + DeserializeOwned,
        CI: 'static + Clone + Eq + DeserializeOwned,
        U: 'static + Send + Sync + Clone + DeserializeOwned,
    {
        let res = self;

        let states = Arc::clone(&res.states);
        let pause = Arc::clone(&res.is_paused);
        let delay = Arc::clone(&res.delay);
        let mut control = keyboard_control.clone_receive_only();

        std::thread::spawn(move || loop {
            let ch = control.receive();
            if ch == ' ' || ch == 'h' || ch == 'l' {
                loop {
                    let unlocked = pause.try_write();
                    if unlocked.is_err() {
                        continue;
                    }
                    let mut is_paused = unlocked.ok().unwrap();
                    *is_paused = if ch == ' ' { !*is_paused } else { true };

                    if ch != ' ' {
                        loop {
                            let states = states.try_write();
                            if states.is_err() {
                                continue;
                            }
                            let mut states = states.ok().unwrap();
                            let mut new_idx = states.get_idx();
                            if ch == 'h' && new_idx > 0 {
                                new_idx -= 1;
                            } else {
                                new_idx += 1;
                            }
                            states.set_idx(new_idx);
                            break;
                        }
                    }
                    break;
                }
            } else if ch == 'j' || ch == 'k' {
                loop {
                    let unlocked = delay.try_write();
                    if unlocked.is_err() {
                        continue;
                    }
                    let mut delay = unlocked.ok().unwrap();
                    if ch == 'j' {
                        *delay = *delay * 2;
                    } else {
                        *delay = *delay / 2;
                    }
                    break;
                }
            } else if ch == 'q' {
                // TODO: Hacky solution to give other callbacks a time buffer to do cleanup.
                std::thread::sleep(std::time::Duration::from_millis(500));
                std::process::exit(0);
            }
        });

        res
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

    pub fn get_header(&self) -> U
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
            return unlocked.unwrap().get_header().clone();
        }
    }

    fn set_paused(&self, is_paused: bool) {
        loop {
            let unlocked = self.is_paused.try_write();
            if unlocked.is_err() {
                continue;
            }
            *unlocked.unwrap() = is_paused;
            break;
        }
    }
}

impl<T, CI, U> Replay for ReplayerLocal<T, CI, U>
where
    T: 'static + Send + Sync + Clone + DeserializeOwned,
    CI: 'static + Send + Sync + Clone + Eq + Hash + DeserializeOwned,
    U: 'static + Send + Sync + Clone + DeserializeOwned,
{
    fn play(&self) {
        self.set_paused(false);
    }

    fn pause(&self) {
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
            break;
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

    fn set_idx(&self, idx: usize) {
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
