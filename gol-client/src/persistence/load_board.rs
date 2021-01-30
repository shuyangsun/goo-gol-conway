use gol_core::{
    util::grid_util::Size2D, BinaryState, BinaryStatesReadOnly, BinaryStrategy,
    BoardCallbackManager, BoardNeighborManager, BoardSpaceManager, BoardStateManager,
    BoardStrategyManager, DiscreteStrategy, Grid, GridFactory, GridPoint2D, IndexedDataOwned,
    NeighborMoore, NeighborMooreDonut, NeighborsGridDonut, NeighborsGridSurround,
    SharedStrategyManager, SparseBinaryStates, SparseStates, StatesReadOnly,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type IntIdx = i32;
type IntState = u8;

// Visual

#[derive(Serialize, Deserialize)]
pub enum VisualKind {
    Ascii,
    Graphical,
}

#[derive(Serialize, Deserialize)]
pub struct VisualConfig {
    on: bool,
    kind: Vec<VisualKind>,
}

// Neighbor

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum NeighborRuleConfig {
    Moore { margin: usize },
    MooreWrap { margin: usize },
}

// State

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum StateConfig {
    UInt { count: usize },
}

// State

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CellCount {
    Integer(usize),
    Range(Vec<usize>),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum EvolutionRuleConfig {
    AliveCount {
        survive: Vec<CellCount>,
        born: Vec<CellCount>,
    },
}

// Board

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum BoardConfig {
    Grid2D {
        shape: Size2D,
        initial_states: HashMap<String, Vec<GridPoint2D<IntIdx>>>,
    },
}

// Cellular Automaton

#[derive(Serialize, Deserialize)]
pub struct CellularAutomatonConfig {
    title: String,
    max_iter: Option<usize>,
    delay: f64,
    pause_at_start: bool,
    enable_control: bool,
    save: Option<String>,
    visual: VisualConfig,
    neighbor_rule: NeighborRuleConfig,
    state: StateConfig,
    evolution_rule: EvolutionRuleConfig,
    board: BoardConfig,
}

impl CellularAutomatonConfig {
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    fn gen_space_grid_2d(
        &self,
    ) -> Result<
        Box<
            dyn BoardSpaceManager<
                GridPoint2D<IntIdx>,
                std::vec::IntoIter<GridPoint2D<IntIdx>>,
                rayon::vec::IntoIter<GridPoint2D<IntIdx>>,
            >,
        >,
        (),
    > {
        match &self.board {
            BoardConfig::Grid2D {
                shape,
                initial_states: _,
            } => {
                let shape_vec = vec![shape.width(), shape.height()];
                let space_manager = Grid::<GridPoint2D<IntIdx>>::new(shape_vec.into_iter());
                Ok(Box::new(space_manager))
            }
        }
    }

    fn gen_neighbor_grid_2d(
        &self,
    ) -> Result<
        Box<dyn BoardNeighborManager<GridPoint2D<IntIdx>, std::vec::IntoIter<GridPoint2D<IntIdx>>>>,
        (),
    > {
        match &self.neighbor_rule {
            NeighborRuleConfig::Moore { margin } => {
                if margin == &1 {
                    Ok(Box::new(NeighborMoore::new()))
                } else {
                    Ok(Box::new(NeighborsGridSurround::new(margin.clone())))
                }
            }
            NeighborRuleConfig::MooreWrap { margin } => {
                let shape = match &self.board {
                    BoardConfig::Grid2D {
                        shape,
                        initial_states: _,
                    } => shape,
                };
                if margin == &1 {
                    Ok(Box::new(NeighborMooreDonut::new(shape.clone())))
                } else {
                    Ok(Box::new(NeighborsGridDonut::new(
                        margin.clone(),
                        [shape.width(), shape.height()].iter().cloned(),
                    )))
                }
            }
        }
    }

    fn gen_state_manager_grid_2d_binary(
        &self,
    ) -> Result<
        Box<
            dyn BoardStateManager<
                BinaryState,
                GridPoint2D<IntIdx>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<IntIdx>, BinaryState>>,
            >,
        >,
        (),
    > {
        match &self.state {
            StateConfig::UInt { count } => {
                assert!(count == &2);
                let init_states = match &self.board {
                    BoardConfig::Grid2D {
                        shape: _,
                        initial_states,
                    } => initial_states
                        .get("1")
                        .unwrap()
                        .par_iter()
                        .cloned()
                        .collect(),
                };
                Ok(Box::new(SparseBinaryStates::new(
                    BinaryState::Dead,
                    BinaryState::Alive,
                    init_states,
                )))
            }
        }
    }

    fn gen_state_manager_grid_2d_discrete(
        &self,
    ) -> Result<
        Box<
            dyn BoardStateManager<
                IntState,
                GridPoint2D<IntIdx>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<IntIdx>, IntState>>,
            >,
        >,
        (),
    > {
        match &self.state {
            StateConfig::UInt { count } => {
                assert!(count > &2);
                let init_states = match &self.board {
                    BoardConfig::Grid2D {
                        shape: _,
                        initial_states,
                    } => initial_states
                        .par_iter()
                        .map(|(key, val)| {
                            let cur_map: HashMap<GridPoint2D<IntIdx>, IntState> = val
                                .par_iter()
                                .map(|ele| {
                                    (
                                        ele.clone(),
                                        key.parse::<IntState>()
                                            .expect("Discrete states must be unsigned integers."),
                                    )
                                })
                                .collect();
                            cur_map
                        })
                        .reduce(|| HashMap::new(), |a, b| a.into_iter().chain(b).collect()),
                };
                Ok(Box::new(SparseStates::new(0, init_states)))
            }
        }
    }

    fn gen_strat_grid_2d_binary(
        &self,
    ) -> Result<
        Box<
            dyn BoardStrategyManager<
                GridPoint2D<IntIdx>,
                BinaryState,
                std::vec::IntoIter<IndexedDataOwned<GridPoint2D<IntIdx>, BinaryState>>,
            >,
        >,
        (),
    > {
        match &self.evolution_rule {
            EvolutionRuleConfig::AliveCount { survive, born } => {
                Ok(Box::new(SharedStrategyManager::new(Box::new(
                    BinaryStrategy::new(collect_cell_counts(&survive), collect_cell_counts(&born)),
                ))))
            }
        }
    }

    fn gen_strat_grid_2d_discrete(
        &self,
    ) -> Result<
        Box<
            dyn BoardStrategyManager<
                GridPoint2D<IntIdx>,
                IntState,
                std::vec::IntoIter<IndexedDataOwned<GridPoint2D<IntIdx>, IntState>>,
            >,
        >,
        (),
    > {
        let state_count = match &self.state {
            StateConfig::UInt { count } => count,
        };
        match &self.evolution_rule {
            EvolutionRuleConfig::AliveCount { survive, born } => Ok(Box::new(
                SharedStrategyManager::new(Box::new(DiscreteStrategy::new(
                    state_count.clone(),
                    collect_cell_counts(&survive),
                    collect_cell_counts(&born),
                ))),
            )),
        }
    }

    fn gen_callback_grid_2d_binary_state(
        &self,
    ) -> (
        BoardCallbackManager<
            BinaryState,
            GridPoint2D<IntIdx>,
            rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<IntIdx>, BinaryState>>,
        >,
        Option<BinaryStatesReadOnly<GridPoint2D<IntIdx>, BinaryState>>,
    ) {
        let mut res = Vec::new();
        // TODO: implementation
        (BoardCallbackManager::new(res), None)
    }
}

fn collect_cell_counts(counts: &Vec<CellCount>) -> HashSet<usize> {
    counts
        .par_iter()
        .map(|ele| match ele {
            CellCount::Integer(val) => HashSet::from_iter([val.clone()].iter().cloned()),
            CellCount::Range(range) => {
                HashSet::from_iter(range.first().unwrap().clone()..=range.last().unwrap().clone())
            }
        })
        .reduce(|| HashSet::new(), |a, b| a.union(&b).cloned().collect())
}
