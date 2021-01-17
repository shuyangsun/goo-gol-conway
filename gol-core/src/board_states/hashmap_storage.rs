use crate::{BoardStateManager, IndexedDataOwned};
use rayon::prelude::*;
use std::collections::HashMap;

pub struct StateLookup<T, CI> {
    default_state: T,
    lookup: HashMap<CI, T>,
}

impl<T, CI> BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>
    for StateLookup<T, CI>
where
    T: Send + Sync + Clone + std::cmp::PartialEq,
    CI: Send + Sync + std::hash::Hash + std::cmp::Eq + Clone,
{
    fn get_cell_state(&self, idx: &CI) -> T {
        match self.lookup.get(idx) {
            Some(val) => val.clone(),
            None => self.default_state.clone(),
        }
    }

    fn update_cell_states_from_par_iter(
        &mut self,
        new_states: rayon::vec::IntoIter<IndexedDataOwned<CI, T>>,
    ) {
        self.lookup = new_states
            .filter(|ele| ele.1 != self.default_state)
            .map(|ele| (ele.0.clone(), ele.1.clone()))
            .collect();
    }
}
