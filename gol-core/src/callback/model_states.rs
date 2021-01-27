use crate::util::sync_util::ReadOnlyLock;
use crate::{BoardCallbackWithStates, IndexedDataOwned};
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock, RwLockReadGuard, TryLockResult};

pub struct StatesReadOnly<T, CI>
where
    CI: Hash,
{
    trivial_state: T,
    lookup: ReadOnlyLock<(usize, HashMap<CI, T>)>,
}

impl<T, CI> Clone for StatesReadOnly<T, CI>
where
    CI: Hash,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.trivial_state.clone(), self.lookup.clone())
    }
}

impl<T, CI> StatesReadOnly<T, CI>
where
    CI: Hash,
{
    pub fn new(trivial_state: T, lock: ReadOnlyLock<(usize, HashMap<CI, T>)>) -> Self {
        Self {
            trivial_state,
            lookup: lock,
        }
    }

    pub fn trivial_state(&self) -> &T {
        &self.trivial_state
    }

    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<(usize, HashMap<CI, T>)>> {
        self.lookup.try_read()
    }
}

pub struct StatesCallback<T, CI>
where
    CI: Hash,
{
    trivial_state: T,
    non_trivial_lookup: Arc<RwLock<(usize, HashMap<CI, T>)>>,
}

impl<T, CI> StatesCallback<T, CI>
where
    CI: Hash,
{
    pub fn new(trivial_state: T) -> Self {
        Self::new_with_non_trivial_lookup(trivial_state, HashMap::new())
    }

    pub fn new_with_non_trivial_lookup(
        trivial_state: T,
        non_trivial_lookup: HashMap<CI, T>,
    ) -> Self {
        Self {
            trivial_state,
            non_trivial_lookup: Arc::new(RwLock::new((0, non_trivial_lookup))),
        }
    }

    pub fn set_non_trivial_lookup(&mut self, lookup: HashMap<CI, T>) {
        let mut lookup_unlocked = self.non_trivial_lookup.write().unwrap();
        let iter_count = lookup_unlocked.0 + 1;
        *lookup_unlocked = (iter_count, lookup);
    }

    pub fn clone_read_only(&self) -> StatesReadOnly<T, CI>
    where
        T: Clone,
    {
        StatesReadOnly::new(
            self.trivial_state.clone(),
            ReadOnlyLock::new(Arc::clone(&self.non_trivial_lookup)),
        )
    }
}

impl<T, CI, I> BoardCallbackWithStates<T, CI, I> for StatesCallback<T, CI>
where
    T: Send + Sync + Clone + Eq,
    CI: Send + Sync + Clone + Eq + Hash,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn execute(&mut self, states: I) {
        let new_lookup: HashMap<CI, T> = states
            .filter(|ele| &ele.1 != &self.trivial_state)
            .map(|ele| (ele.0, ele.1))
            .collect();
        self.set_non_trivial_lookup(new_lookup);
    }
}
