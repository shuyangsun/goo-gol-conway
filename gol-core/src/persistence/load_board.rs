use crate::{util::grid_util::Size2D, Board, DiscreteState, GridPoint2D};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    enable_contro: bool,
    save: Option<String>,
    visual: VisualConfig,
    neighbor_rule: NeighborRuleConfig,
    state: StateConfig,
    evolution_rule: EvolutionRuleConfig,
    board: BoardConfig,
}

impl CellularAutomatonConfig {
    fn try_gen_discrete_state<const N: u8>(&self) -> Result<DiscreteState<u8, N>, ()> {
        match self.state {
            StateConfig::UInt { count } => {
                if count == N as usize {
                    Ok(DiscreteState::new())
                } else {
                    Err(())
                }
            }
        }
    }
}
