use crate::{
    Board, BoardCallbackManager, BoardNeighborManager, BoardSpaceManager, BoardStateManager,
    BoardStrategyManager, IndexedDataOwned,
};
use rayon;

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
