use crate::util::sync_util::ReadOnlyLock;
use crate::{BoardCallbackWithStates, IndexedDataOwned};
use rayon::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{Arc, RwLock, RwLockReadGuard, TryLockResult};

pub struct BinaryStatesReadOnly<T, CI>
where
    CI: Hash,
{
    trivial_state: T,
    non_trivial_state: T,
    non_trivial_indices: ReadOnlyLock<(usize, HashSet<CI>)>,
}

impl<T, CI> Clone for BinaryStatesReadOnly<T, CI>
where
    CI: Hash,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new(
            self.trivial_state.clone(),
            self.non_trivial_state.clone(),
            self.non_trivial_indices.clone(),
        )
    }
}

impl<T, CI> BinaryStatesReadOnly<T, CI>
where
    CI: Hash,
{
    pub fn new(
        trivial_state: T,
        non_trivial_state: T,
        lock: ReadOnlyLock<(usize, HashSet<CI>)>,
    ) -> Self {
        Self {
            trivial_state,
            non_trivial_state,
            non_trivial_indices: lock,
        }
    }

    pub fn trivial_state(&self) -> &T {
        &self.trivial_state
    }

    pub fn non_trivial_state(&self) -> &T {
        &self.non_trivial_state
    }

    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<(usize, HashSet<CI>)>> {
        self.non_trivial_indices.try_read()
    }
}

pub struct BinaryStatesCallback<T, CI>
where
    CI: Hash,
{
    trivial_state: T,
    non_trivial_state: T,
    non_trivial_indices: Arc<RwLock<(usize, HashSet<CI>)>>,
}

impl<T, CI> BinaryStatesCallback<T, CI>
where
    CI: Hash,
{
    pub fn new(trivial_state: T, non_trivial_state: T) -> Self {
        Self::new_with_non_trivial_indices(trivial_state, non_trivial_state, HashSet::new())
    }

    pub fn new_with_non_trivial_indices(
        trivial_state: T,
        non_trivial_state: T,
        non_trivial_indices: HashSet<CI>,
    ) -> Self {
        Self {
            trivial_state,
            non_trivial_state,
            non_trivial_indices: Arc::new(RwLock::new((0, non_trivial_indices))),
        }
    }

    pub fn set_non_trivial_states(&mut self, indices: HashSet<CI>) {
        let mut indices_unlocked = self.non_trivial_indices.write().unwrap();
        let iter_count = indices_unlocked.0 + 1;
        *indices_unlocked = (iter_count, indices);
    }

    pub fn clone_read_only(&self) -> BinaryStatesReadOnly<T, CI>
    where
        T: Clone,
    {
        BinaryStatesReadOnly::new(
            self.trivial_state.clone(),
            self.non_trivial_state.clone(),
            ReadOnlyLock::new(Arc::clone(&self.non_trivial_indices)),
        )
    }
}

impl<T, CI, I> BoardCallbackWithStates<T, CI, I> for BinaryStatesCallback<T, CI>
where
    T: Send + Sync + Clone + Eq,
    CI: Send + Sync + Clone + Eq + Hash,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn execute(&mut self, states: I) {
        let new_indices: HashSet<CI> = states
            .filter(|ele| &ele.1 != &self.trivial_state)
            .map(|ele| ele.0)
            .collect();
        self.set_non_trivial_states(new_indices);
    }
}
