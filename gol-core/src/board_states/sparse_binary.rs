use crate::{BoardStateManager, IndexedDataOwned};
use rayon::prelude::*;
use std::collections::HashSet;

pub struct SparseBinaryStates<T, CI> {
    default_state: T,
    non_default_state: T,
    non_default_indices: HashSet<CI>,
}

impl<T, CI> SparseBinaryStates<T, CI> {
    pub fn new(default_state: T, non_default_state: T, initial_non_defaults: &HashSet<CI>) -> Self
    where
        CI: Eq + std::hash::Hash + Clone,
        T: PartialEq,
    {
        Self {
            default_state,
            non_default_state,
            non_default_indices: initial_non_defaults.clone(),
        }
    }
}

impl<T, CI> BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>
    for SparseBinaryStates<T, CI>
where
    T: Send + Sync + Clone + std::cmp::PartialEq,
    CI: Send + Sync + std::hash::Hash + std::cmp::Eq + Clone,
{
    fn get_cell_state(&self, idx: &CI) -> T {
        if self.non_default_indices.contains(idx) {
            self.non_default_state.clone()
        } else {
            self.default_state.clone()
        }
    }

    fn update_cell_states_from_par_iter(
        &mut self,
        new_states: rayon::vec::IntoIter<IndexedDataOwned<CI, T>>,
    ) {
        self.non_default_indices = new_states
            .filter(|ele| ele.1 != self.default_state)
            .map(|ele| ele.0.clone())
            .collect();
    }

    fn get_non_trivial_states(&self) -> Result<rayon::vec::IntoIter<IndexedDataOwned<CI, T>>, ()> {
        let res: Vec<IndexedDataOwned<CI, T>> = self
            .non_default_indices
            .par_iter()
            .map(|ele| (ele.clone(), self.non_default_state.clone()))
            .collect();
        Ok(res.into_par_iter())
    }
}

#[cfg(test)]
mod sparse_binary_state_manager_test {
    use crate::{BoardStateManager, ConwayState, GridPoint2D, SparseBinaryStates};
    use std::collections::HashSet;

    #[test]
    fn sparse_state_test_1() {
        let mut alive_cells = HashSet::new();
        alive_cells.insert(GridPoint2D { x: 0, y: 0 });
        let states = SparseBinaryStates::new(ConwayState::Dead, ConwayState::Alive, &alive_cells);
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
