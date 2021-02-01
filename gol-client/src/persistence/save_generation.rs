use bincode;
use gol_core::{BoardCallbackWithStates, IndexedDataOwned};
use rayon::prelude::*;
use serde::Serialize;

// impl<T, CI, I> BoardCallbackWithStates<T, CI, I> for BatchIndexedSerializer<Vec<(CI, T)>>
// where
//     T: Send + Sync + Serialize,
//     CI: Send + Sync + Serialize,
//     I: rayon::iter::ParallelIterator<Item = IndexedDataOwned<CI, T>>,
// {
//     fn execute(&mut self, states: I) {
//         let res: Vec<(CI, T)> = states.collect();
//         let binary_states: Vec<u8> = bincode::serialize(&res).unwrap();
//     }
// }
