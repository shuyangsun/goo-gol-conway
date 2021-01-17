use crate::{BoardStateManager, IndexedDataOwned};
use rayon::prelude::*;
use std::collections::HashMap;

pub struct SparseStates<T, CI> {
    default_state: T,
    lookup: HashMap<CI, T>,
}

impl<T, CI> SparseStates<T, CI> {
    pub fn new(default_state: T, initial_states: &HashMap<CI, T>) -> Self
    where
        CI: Eq + std::hash::Hash + Clone,
        T: PartialEq + Clone,
    {
        let lookup = initial_states
            .iter()
            .filter(|ele| ele.1 != &default_state)
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        Self {
            default_state,
            lookup,
        }
    }
}

impl<T, CI> BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>
    for SparseStates<T, CI>
where
    T: Send + Sync + Clone + PartialEq,
    CI: Send + Sync + std::hash::Hash + Eq + Clone,
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

#[cfg(test)]
mod sparse_state_manager_test {
    use crate::{BoardStateManager, ConwayState, GridPoint2D, SparseStates};
    use std::collections::HashMap;

    #[test]
    fn sparse_state_test_1() {
        let mut initial_maps = HashMap::new();
        initial_maps.insert(GridPoint2D { x: 0, y: 0 }, ConwayState::Alive);
        let states = SparseStates::new(ConwayState::Dead, &initial_maps);
        assert_eq!(
            states.get_cell_state(&GridPoint2D { x: 0, y: 0 }),
            ConwayState::Alive
        );
        assert_eq!(
            states.get_cell_state(&GridPoint2D { x: 1, y: 0 }),
            ConwayState::Dead
        );
        assert_eq!(
            states.get_cell_state(&GridPoint2D { x: 1, y: -5 }),
            ConwayState::Dead
        );
    }
}
