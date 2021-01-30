use crate::{
    util::grid_util::Size2D, BinaryState, Board, BoardSpaceManager, BoardStateManager, Grid,
    GridFactory, GridPoint2D, IndexedDataOwned, SparseBinaryStates, SparseStates,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};

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
        initial_states: HashMap<String, Vec<GridPoint2D<i32>>>,
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

    fn gen_space_manager_grid_2d(
        &self,
    ) -> Result<
        Box<
            dyn BoardSpaceManager<
                GridPoint2D<i32>,
                std::vec::IntoIter<GridPoint2D<i32>>,
                rayon::vec::IntoIter<GridPoint2D<i32>>,
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
                let space_manager = Grid::<GridPoint2D<i32>>::new(shape_vec.into_iter());
                Ok(Box::new(space_manager))
            }
        }
    }

    fn gen_binary_state_manager_grid_2d(
        &self,
    ) -> Result<
        Box<
            dyn BoardStateManager<
                BinaryState,
                GridPoint2D<i32>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<i32>, BinaryState>>,
            >,
        >,
        (),
    > {
        match &self.state {
            StateConfig::UInt { count } => {
                assert!(count == &2);
                let res_init_states;
                match &self.board {
                    BoardConfig::Grid2D {
                        shape: _,
                        initial_states,
                    } => {
                        res_init_states = initial_states.clone();
                    }
                }
                let init_states: HashSet<GridPoint2D<i32>> = res_init_states
                    .get("1")
                    .unwrap()
                    .par_iter()
                    .cloned()
                    .collect();
                Ok(Box::new(SparseBinaryStates::new(
                    BinaryState::Dead,
                    BinaryState::Alive,
                    init_states,
                )))
            }
        }
    }

    fn gen_discrete_state_manager_grid_2d(
        &self,
    ) -> Result<
        Box<
            dyn BoardStateManager<
                u8,
                GridPoint2D<i32>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<i32>, u8>>,
            >,
        >,
        (),
    > {
        match &self.state {
            StateConfig::UInt { count } => {
                if count > &2 {
                    let init_states = HashMap::new();
                    Ok(Box::new(SparseStates::new(0u8, init_states)))
                } else {
                    Err(())
                }
            }
        }
    }
}
