use crate::neighbors::util::{MarginPrimInt, PointPrimInt};
use crate::{
    util::grid_util::Size2D, Board, BoardCallback, BoardCallbackManager, BoardNeighborManager,
    BoardSpaceManager, BoardStateManager, BoardStrategyManager, EvolutionStrategy, Grid,
    GridFactory, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND, IndexedDataOwned,
    NeighborMoore, NeighborMooreDonut, NeighborsGridDonut, NeighborsGridSurround,
    SharedStrategyManager, SparseBinaryStates, SparseStates,
};
use num_traits::{CheckedDiv, FromPrimitive, PrimInt, Unsigned};
use rayon;
use std::collections::{HashMap, HashSet};
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
        &mut self,
    ) -> &mut BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>> {
        &mut self.callback_manager
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
        callbacks: Vec<BoardCallback<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
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
    pub fn new_standard_nd_grid<T, U, S, I>(
        shape: I,
        default_state: T,
        neighbor_margin: S,
        initial_states: HashMap<GridPointND<U>, T>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPointND<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPointND<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPointND<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPointND<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + FromPrimitive + MarginPrimInt,
        I: Iterator<Item = S>,
    {
        let shape: Vec<S> = shape.collect();
        let space_manager = Grid::<GridPointND<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                as Box<dyn BoardNeighborManager<GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>>
        } else {
            Box::new(NeighborsGridSurround::new(neighbor_margin))
                as Box<dyn BoardNeighborManager<GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>>
        };
        let state_manager = SparseStates::new(default_state, initial_states);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }

    pub fn new_binary_nd_grid<T, U, S, I>(
        shape: I,
        default_state: T,
        non_default_state: T,
        neighbor_margin: S,
        non_default_indices: HashSet<GridPointND<U>>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPointND<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPointND<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPointND<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPointND<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + Unsigned + FromPrimitive + MarginPrimInt,
        I: Iterator<Item = S>,
    {
        let shape: Vec<S> = shape.collect();
        let space_manager = Grid::<GridPointND<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                as Box<dyn BoardNeighborManager<GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>>
        } else {
            Box::new(NeighborsGridSurround::new(neighbor_margin))
                as Box<dyn BoardNeighborManager<GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>>
        };
        let state_manager =
            SparseBinaryStates::new(default_state, non_default_state, non_default_indices);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }

    pub fn new_standard_3d_grid<T, U, S>(
        shape: (S, S, S),
        default_state: T,
        neighbor_margin: S,
        initial_states: HashMap<GridPoint3D<U>, T>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPoint3D<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPoint3D<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPoint3D<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint3D<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + Unsigned + FromPrimitive + MarginPrimInt,
    {
        let shape = vec![shape.0, shape.1, shape.2];
        let space_manager = Grid::<GridPoint3D<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                as Box<dyn BoardNeighborManager<GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>>
        } else {
            Box::new(NeighborsGridSurround::new(neighbor_margin))
                as Box<dyn BoardNeighborManager<GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>>
        };
        let state_manager = SparseStates::new(default_state, initial_states);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }

    pub fn new_binary_3d_grid<T, U, S>(
        shape: (S, S, S),
        default_state: T,
        non_default_state: T,
        neighbor_margin: S,
        non_default_indices: HashSet<GridPoint3D<U>>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPoint3D<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPoint3D<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPoint3D<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint3D<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + Unsigned + FromPrimitive + MarginPrimInt,
    {
        let shape = vec![shape.0, shape.1, shape.2];
        let space_manager = Grid::<GridPoint3D<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                as Box<dyn BoardNeighborManager<GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>>
        } else {
            Box::new(NeighborsGridSurround::new(neighbor_margin))
                as Box<dyn BoardNeighborManager<GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>>
        };
        let state_manager =
            SparseBinaryStates::new(default_state, non_default_state, non_default_indices);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }

    pub fn new_standard_2d_grid<T, U, S>(
        shape: (S, S),
        default_state: T,
        neighbor_margin: S,
        initial_states: HashMap<GridPoint2D<U>, T>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPoint2D<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPoint2D<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPoint2D<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPoint2D<U>, std::vec::IntoIter<GridPoint2D<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + Unsigned + FromPrimitive + MarginPrimInt,
    {
        let (width, height) = shape;
        let shape = vec![width, height];
        let space_manager = Grid::<GridPoint2D<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            if neighbor_margin == S::one() {
                Box::new(NeighborMooreDonut::new(Size2D::new(
                    width.to_usize().unwrap(),
                    height.to_usize().unwrap(),
                )))
            } else {
                Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                    as Box<
                        dyn BoardNeighborManager<
                            GridPoint2D<U>,
                            std::vec::IntoIter<GridPoint2D<U>>,
                        >,
                    >
            }
        } else {
            if neighbor_margin == S::one() {
                Box::new(NeighborMoore::new())
            } else {
                Box::new(NeighborsGridSurround::new(neighbor_margin))
                    as Box<
                        dyn BoardNeighborManager<
                            GridPoint2D<U>,
                            std::vec::IntoIter<GridPoint2D<U>>,
                        >,
                    >
            }
        };
        let state_manager = SparseStates::new(default_state, initial_states);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }

    pub fn new_binary_2d_grid<T, U, S>(
        shape: (S, S),
        default_state: T,
        non_default_state: T,
        neighbor_margin: S,
        non_default_indices: HashSet<GridPoint2D<U>>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPoint2D<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPoint2D<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPoint2D<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPoint2D<U>, std::vec::IntoIter<GridPoint2D<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + Unsigned + FromPrimitive + MarginPrimInt,
    {
        let (width, height) = shape;
        let shape = vec![width, height];
        let space_manager = Grid::<GridPoint2D<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            if neighbor_margin == S::one() {
                Box::new(NeighborMooreDonut::new(Size2D::new(
                    width.to_usize().unwrap(),
                    height.to_usize().unwrap(),
                )))
            } else {
                Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                    as Box<
                        dyn BoardNeighborManager<
                            GridPoint2D<U>,
                            std::vec::IntoIter<GridPoint2D<U>>,
                        >,
                    >
            }
        } else {
            if neighbor_margin == S::one() {
                Box::new(NeighborMoore::new())
            } else {
                Box::new(NeighborsGridSurround::new(neighbor_margin))
                    as Box<
                        dyn BoardNeighborManager<
                            GridPoint2D<U>,
                            std::vec::IntoIter<GridPoint2D<U>>,
                        >,
                    >
            }
        };
        let state_manager =
            SparseBinaryStates::new(default_state, non_default_state, non_default_indices);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }

    pub fn new_standard_1d_grid<T, U, S>(
        shape: S,
        default_state: T,
        neighbor_margin: S,
        initial_states: HashMap<GridPoint1D<U>, T>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPoint1D<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPoint1D<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPoint1D<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint1D<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPoint1D<U>, std::vec::IntoIter<GridPoint1D<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + Unsigned + FromPrimitive + MarginPrimInt,
    {
        let shape = vec![shape];
        let space_manager = Grid::<GridPoint1D<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                as Box<dyn BoardNeighborManager<GridPoint1D<U>, std::vec::IntoIter<GridPoint1D<U>>>>
        } else {
            Box::new(NeighborsGridSurround::new(neighbor_margin))
                as Box<dyn BoardNeighborManager<GridPoint1D<U>, std::vec::IntoIter<GridPoint1D<U>>>>
        };
        let state_manager = SparseStates::new(default_state, initial_states);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }

    pub fn new_binary_1d_grid<T, U, S>(
        shape: S,
        default_state: T,
        non_default_state: T,
        neighbor_margin: S,
        non_default_indices: HashSet<GridPoint1D<U>>,
        strategy: Box<
            dyn EvolutionStrategy<
                GridPoint1D<U>,
                T,
                std::vec::IntoIter<IndexedDataOwned<GridPoint1D<U>, T>>,
            >,
        >,
        callbacks: Vec<
            BoardCallback<
                T,
                GridPoint1D<U>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint1D<U>, T>>,
            >,
        >,
        is_donut: bool,
    ) -> StandardBoard<T, GridPoint1D<U>, std::vec::IntoIter<GridPoint1D<U>>>
    where
        T: 'static + Send + Sync + Clone + PartialEq,
        U: 'static + Hash + PrimInt + CheckedDiv + std::convert::TryFrom<S> + PointPrimInt,
        S: 'static + Unsigned + FromPrimitive + MarginPrimInt,
    {
        let shape = vec![shape];
        let space_manager = Grid::<GridPoint1D<U>>::new(shape.clone().into_iter());
        let neighbor_manager = if is_donut {
            Box::new(NeighborsGridDonut::new(neighbor_margin, shape.into_iter()))
                as Box<dyn BoardNeighborManager<GridPoint1D<U>, std::vec::IntoIter<GridPoint1D<U>>>>
        } else {
            Box::new(NeighborsGridSurround::new(neighbor_margin))
                as Box<dyn BoardNeighborManager<GridPoint1D<U>, std::vec::IntoIter<GridPoint1D<U>>>>
        };
        let state_manager =
            SparseBinaryStates::new(default_state, non_default_state, non_default_indices);
        let strategy_manger = SharedStrategyManager::new(strategy);
        StandardBoard::new(
            Box::new(space_manager),
            neighbor_manager,
            Box::new(state_manager),
            Box::new(strategy_manger),
            callbacks,
        )
    }
}
