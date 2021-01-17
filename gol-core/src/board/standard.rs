use crate::neighbors::grid_surround::{MarginPrimInt, PointPrimInt};
use crate::{
    Board, BoardCallback, BoardCallbackManager, BoardNeighborManager, BoardSpaceManager,
    BoardStateManager, BoardStrategyManager, EvolutionStrategy, Grid, GridFactory, GridPointND,
    IndexedDataOwned, NeighborsGridSurround, SharedStrategyManager, SparseStates,
};
use num_traits::{CheckedDiv, FromPrimitive, PrimInt, Unsigned};
use rayon;
use std::collections::HashMap;
use std::hash::Hash;

pub struct StandardBoardFactory {}

pub struct StandardBoard<T, CI, I>
where
    T: Send + Sync,
    CI: Send + Sync,
    I: Iterator<Item = CI>,
{
    space_manager: Box<dyn BoardSpaceManager<CI, I, rayon::vec::IntoIter<CI>>>,
    neighbor_manager: Box<dyn BoardNeighborManager<CI, I>>,
    state_manager: Box<dyn BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
    strategy_manager:
        Box<dyn BoardStrategyManager<CI, T, std::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
    callback_manager: BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>,
}

impl<T, CI, I> Board<T, CI, I> for StandardBoard<T, CI, I>
where
    T: 'static + Send + Sync + Clone,
    CI: 'static + Send + Sync + Clone,
    I: Iterator<Item = CI>,
{
    fn space_manager(&self) -> &dyn BoardSpaceManager<CI, I, rayon::vec::IntoIter<CI>> {
        &*self.space_manager
    }

    fn neighbor_manager(&self) -> &dyn BoardNeighborManager<CI, I> {
        &*self.neighbor_manager
    }

    fn state_manager(
        &self,
    ) -> &dyn BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>> {
        &*self.state_manager
    }

    fn state_manager_mut(
        &mut self,
    ) -> &mut dyn BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>> {
        &mut *self.state_manager
    }

    fn strategy_manager(
        &self,
    ) -> &dyn BoardStrategyManager<CI, T, std::vec::IntoIter<IndexedDataOwned<CI, T>>> {
        &*self.strategy_manager
    }

    fn callback_manager(
        &self,
    ) -> &BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>> {
        &self.callback_manager
    }
}

impl<T, CI, I> StandardBoard<T, CI, I>
where
    T: 'static + Send + Sync + Clone,
    CI: 'static + Send + Sync + Clone,
    I: Iterator<Item = CI>,
{
    pub fn new(
        space_manager: Box<dyn BoardSpaceManager<CI, I, rayon::vec::IntoIter<CI>>>,
        neighbor_manager: Box<dyn BoardNeighborManager<CI, I>>,
        state_manager: Box<
            dyn BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>,
        >,
        strategy_manager: Box<
            dyn BoardStrategyManager<CI, T, std::vec::IntoIter<IndexedDataOwned<CI, T>>>,
        >,
        callbacks: Vec<
            Box<dyn BoardCallback<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
        >,
    ) -> Self {
        Self {
            space_manager,
            neighbor_manager,
            state_manager,
            strategy_manager,
            callback_manager: BoardCallbackManager::new(callbacks),
        }
    }
}

impl StandardBoardFactory {
    pub fn new_standard_nd_grid<T, U, K, I>(
        shape: I,
        default_state: &T,
        neighbor_margin: &K,
        initial_states: &HashMap<GridPointND<U>, T>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPointND<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPointND<U>, T>>,
            >,
        >,
        callbacks: Vec<
            Box<
                dyn BoardCallback<
                    T,
                    GridPointND<U>,
                    rayon::vec::IntoIter<IndexedDataOwned<GridPointND<U>, T>>,
                >,
            >,
        >,
    ) -> StandardBoard<T, GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<K> + PointPrimInt,
        K: 'static + Unsigned + FromPrimitive + MarginPrimInt,
        I: Iterator<Item = K>,
    {
        let space_manager = Grid::<GridPointND<U>>::new(shape);
        let neighbor_manager = NeighborsGridSurround::new(*neighbor_margin);
        let state_manager = SparseStates::new(default_state.clone(), initial_states);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            Box::new(neighbor_manager),
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }
}
