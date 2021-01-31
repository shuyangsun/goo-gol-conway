use gol_core::{
    util::grid_util::Shape2D, BinaryState, BinaryStatesCallback, BinaryStrategy, Board,
    BoardCallback, BoardNeighborManager, BoardSpaceManager, BoardStateManager,
    BoardStrategyManager, DiscreteStrategy, Grid, GridFactory, GridPoint2D, IndexedDataOwned,
    NeighborMoore, NeighborMooreDonut, NeighborsGridDonut, NeighborsGridSurround,
    SharedStrategyManager, SparseBinaryStates, SparseStates, StandardBoard,
};
use gol_renderer::{
    renderer::keyboard_control::KeyboardControl, BinaryStateCharMap, BinaryStateColorMap,
    CellularAutomatonRenderer, GraphicalRendererGrid2D,
};
use num_cpus;
use rand::Rng;
use rayon::prelude::*;
use rgb::RGBA16;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::thread;
use std::time::Duration;

type IntIdx = i32;
type IntState = u8;

// Visual

#[derive(Serialize, Deserialize)]
pub enum VisualStyle {
    Ascii,
    Graphical,
}

#[derive(Serialize, Deserialize)]
pub struct VisualConfig {
    on: bool,
    styles: Vec<VisualStyle>,
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
enum InitialStatesConfig {
    Deterministic {
        positions: HashMap<String, Vec<GridPoint2D<IntIdx>>>,
    },
    Random {
        alive_ratio: f32,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum BoardConfig {
    Grid2D {
        shape: Shape2D,
        initial_states: InitialStatesConfig,
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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn run_board(&self) {
        let max_iter = self.max_iter.clone();
        let (mut char_renderers, mut color_renderers) = match self.board {
            BoardConfig::Grid2D {
                shape: _,
                initial_states: _,
            } => {
                let space = self.gen_space_grid_2d().unwrap();
                let neighbor = self.gen_neighbor_grid_2d().unwrap();
                match self.state {
                    StateConfig::UInt { count } => {
                        if count == 2 {
                            let state = self.gen_state_manager_grid_2d_binary().unwrap();
                            let strat = self.gen_strat_grid_2d_binary().unwrap();
                            let (callbacks, char_renderers, color_renderers) =
                                self.gen_callback_grid_2d_binary_state();
                            let mut board =
                                StandardBoard::new(space, neighbor, state, strat, callbacks);
                            std::thread::spawn(move || {
                                board.advance(max_iter);
                            });
                            (char_renderers, color_renderers)
                        } else {
                            let _state = self.gen_state_manager_grid_2d_discrete().unwrap();
                            let _strat = self.gen_strat_grid_2d_discrete().unwrap();
                            panic!("Implement discrete state renderer");
                        }
                    }
                }
            }
        };

        let char_map = BinaryStateCharMap::new();
        let color_map = BinaryStateColorMap::new();

        if char_renderers.len() + color_renderers.len() == 1 {
            match char_renderers.first() {
                Some(_) => char_renderers[0].as_mut().run(Box::new(char_map.clone())),
                None => color_renderers[0].as_mut().run(Box::new(color_map.clone())),
            }
        } else {
            let mut main_renderer = None;
            let mut handles = Vec::with_capacity(char_renderers.len() + color_renderers.len());
            while !char_renderers.is_empty() {
                let mut cur = char_renderers.pop().unwrap();
                let char_map_clone = char_map.clone();
                handles.push(thread::spawn(move || cur.run(Box::new(char_map_clone))));
            }
            while !color_renderers.is_empty() {
                let mut cur = color_renderers.pop().unwrap();
                if cur.need_run_on_main() {
                    main_renderer = Some(cur);
                } else {
                    let color_map_clone = color_map.clone();
                    handles.push(thread::spawn(move || cur.run(Box::new(color_map_clone))));
                }
            }
            match main_renderer {
                Some(mut renderer) => renderer.run(Box::new(color_map)),
                None => {
                    for handle in handles {
                        handle.join().unwrap()
                    }
                }
            }
        }
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
                        shape,
                        initial_states,
                    } => match initial_states {
                        InitialStatesConfig::Deterministic { positions } => {
                            positions.get("1").unwrap().par_iter().cloned().collect()
                        }
                        InitialStatesConfig::Random { alive_ratio } => {
                            gen_2d_random_binary_states(shape, alive_ratio)
                        }
                    },
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
                        shape,
                        initial_states,
                    } => match initial_states {
                        InitialStatesConfig::Deterministic { positions } => positions
                            .par_iter()
                            .map(|(key, val)| {
                                let cur_map: HashMap<GridPoint2D<IntIdx>, IntState> = val
                                    .par_iter()
                                    .map(|ele| {
                                        (
                                            ele.clone(),
                                            key.parse::<IntState>().expect(
                                                "Discrete states must be unsigned integers.",
                                            ),
                                        )
                                    })
                                    .collect();
                                cur_map
                            })
                            .reduce(|| HashMap::new(), |a, b| a.into_iter().chain(b).collect()),
                        InitialStatesConfig::Random { alive_ratio } => {
                            gen_2d_random_discrete_states(shape, alive_ratio, count)
                        }
                    },
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
        Vec<
            BoardCallback<
                BinaryState,
                GridPoint2D<IntIdx>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<IntIdx>, BinaryState>>,
            >,
        >,
        Vec<Box<dyn CellularAutomatonRenderer<BinaryState, char>>>,
        Vec<Box<dyn CellularAutomatonRenderer<BinaryState, RGBA16>>>,
    ) {
        let mut callbacks = Vec::new();
        let mut char_renderers: Vec<Box<dyn CellularAutomatonRenderer<BinaryState, char>>> =
            Vec::new();
        let mut color_renderers: Vec<Box<dyn CellularAutomatonRenderer<BinaryState, RGBA16>>> =
            Vec::new();

        if self.visual.on && !self.visual.styles.is_empty() {
            let one_billion_nano_sec: f64 = 1_000_000_000f64;
            let interval_nano_sec = (self.delay * one_billion_nano_sec) as u64;
            let keyboard_control = if self.enable_control {
                let (control_callbacks, keyboard_control) =
                    crate::callback::standard_control_callbacks(
                        self.pause_at_start,
                        Duration::from_nanos(interval_nano_sec),
                    );
                callbacks = control_callbacks;
                Some(keyboard_control)
            } else {
                None
            };
            let board_shape = match &self.board {
                BoardConfig::Grid2D {
                    shape,
                    initial_states: _,
                } => shape.clone(),
            };
            let binary_states_callback: BinaryStatesCallback<GridPoint2D<IntIdx>, BinaryState> =
                BinaryStatesCallback::new(BinaryState::Dead, BinaryState::Alive);
            let states_read_only = binary_states_callback.clone_read_only();
            let binary_states_callback =
                BoardCallback::WithStates(Box::new(binary_states_callback));
            callbacks.push(binary_states_callback);

            for style in self.visual.styles.iter() {
                match style {
                    VisualStyle::Graphical => {
                        let graphical_renderer = GraphicalRendererGrid2D::new(
                            board_shape.width(),
                            board_shape.height(),
                            states_read_only.clone(),
                        );

                        match graphical_renderer {
                            Ok(val) => {
                                let real_gui_renderer = val.with_title(self.title.clone());
                                let res = match &keyboard_control {
                                    Some(control) => {
                                        real_gui_renderer.with_keyboard_control(control.clone())
                                    }
                                    None => real_gui_renderer,
                                };
                                color_renderers.push(Box::new(res));
                            }
                            Err(err) => eprintln!("Error creating graphical renderer: {:?}", err),
                        };
                    }
                    VisualStyle::Ascii => {
                        #[cfg(not(feature = "ascii"))]
                        eprintln!("Cannot create ASCII renderer, please recompile with \"--features ascii\",");
                        #[cfg(feature = "ascii")]
                        {
                            use gol_renderer::{BinaryStateCharMap, TextRendererGrid2D};

                            let text_renderer = TextRendererGrid2D::new(
                                board_shape.width(),
                                board_shape.height(),
                                states_read_only.clone(),
                            )
                            .with_title(self.title.clone());
                            let res = match &keyboard_control {
                                Some(control) => {
                                    text_renderer.with_keyboard_control(control.clone())
                                }
                                None => text_renderer,
                            };
                            char_renderers.push(Box::new(res));
                        }
                    }
                }
            }
        }

        let mut found_must_main_thread = false;

        for renderer in char_renderers.iter() {
            if renderer.need_run_on_main() {
                if found_must_main_thread {
                    panic!("More than one visual style need to be ran on main thread, try reducing the number of styles.");
                } else {
                    found_must_main_thread = true;
                }
            }
        }

        for renderer in color_renderers.iter() {
            if renderer.need_run_on_main() {
                if found_must_main_thread {
                    panic!("More than one visual style need to be ran on main thread, try reducing the number of styles.");
                } else {
                    found_must_main_thread = true;
                }
            }
        }

        (callbacks, char_renderers, color_renderers)
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

fn gen_random_usize(len: &usize, alive_ratio: &f32) -> HashSet<usize> {
    let core_count = num_cpus::get();
    let num_indices_per_thread = len / core_count + 1;
    let mut state_indices: Vec<HashSet<usize>> = vec![HashSet::new(); core_count];
    state_indices
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, ele)| {
            let mut rng = rand::thread_rng();
            for inner_idx in
                (i * num_indices_per_thread)..std::cmp::min((i + 1) * num_indices_per_thread, *len)
            {
                if &rng.gen::<f32>() <= alive_ratio {
                    ele.insert(inner_idx);
                }
            }
        });

    state_indices
        .into_par_iter()
        .reduce(|| HashSet::new(), |a, b| a.union(&b).cloned().collect())
}

fn gen_2d_random_binary_states(
    board_shape: &Shape2D,
    alive_ratio: &f32,
) -> HashSet<GridPoint2D<IntIdx>> {
    let res = gen_random_usize(&board_shape.volume(), alive_ratio);
    res.into_par_iter()
        .map(|ele| {
            let x = (ele % board_shape.width()) as i64 + board_shape.x_idx_min();
            let y = (ele / board_shape.width()) as i64 + board_shape.y_idx_min();
            GridPoint2D::new(x as IntIdx, y as IntIdx)
        })
        .collect()
}

fn gen_2d_random_discrete_states(
    board_shape: &Shape2D,
    alive_ratio: &f32,
    state_count: &usize,
) -> HashMap<GridPoint2D<IntIdx>, IntState> {
    let res = gen_random_usize(&board_shape.volume(), alive_ratio);
    res.into_par_iter()
        .map(|ele| {
            let x = (ele % board_shape.width()) as i64 + board_shape.x_idx_min();
            let y = (ele / board_shape.width()) as i64 + board_shape.y_idx_min();
            (
                GridPoint2D::new(x as IntIdx, y as IntIdx),
                (state_count - 1) as IntState,
            )
        })
        .collect()
}
