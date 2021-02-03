use super::batch_serializer::{BatchIndexedSerializer, IndexedBatchData};
use gol_core::{BoardCallbackWithStates, IndexedDataOwned};
use rayon::prelude::*;
use serde::Serialize;
use shellexpand;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

pub struct BatchSerializerLocal<T, U>
where
    T: Serialize,
    U: Serialize,
{
    path: String,
    serializer: BatchIndexedSerializer<T, U>,
}

pub struct StateSerializerLocal<T, U, S>
where
    T: Serialize,
    U: Serialize,
{
    serializer: BatchSerializerLocal<T, U>,
    trivial_state: S,
}

impl<T, U> BatchSerializerLocal<T, U>
where
    T: Serialize,
    U: Serialize,
{
    pub fn new(dir_path: &String, serializer: BatchIndexedSerializer<T, U>) -> Self {
        let expanded = shellexpand::full(dir_path).unwrap();
        let dir_path = Path::new(expanded.as_ref());
        let exists = dir_path.exists();
        if exists {
            let is_empty = dir_path.read_dir().unwrap().next().is_none();
            if !is_empty {
                panic!("Directory \"{}\" not empty.", dir_path.to_str().unwrap());
            }
        } else {
            fs::create_dir_all(dir_path).unwrap();
        }
        Self {
            path: String::from(dir_path.to_str().unwrap()),
            serializer,
        }
    }

    pub fn push(&mut self, content: T) {
        let bytes = self.serializer.push(content);
        self.save_bytes(bytes);
    }

    fn save_bytes(&self, bytes: Option<IndexedBatchData>) {
        if bytes.is_none() {
            return;
        }
        let file_extension = "cahist";
        let data = bytes.unwrap();
        let file_name = format!("{}_{}.{}", data.idx_beg, data.idx_end, file_extension);
        let file_path = Path::new(&self.path).join(&file_name);
        let file = File::create(&file_path).unwrap();
        let mut buffer = BufWriter::new(file);
        buffer.write_all(&data.data).unwrap();
        buffer.flush().unwrap();
    }
}

impl<T, U, S> StateSerializerLocal<T, U, S>
where
    T: Serialize,
    U: Serialize,
{
    pub fn new(serializer: BatchSerializerLocal<T, U>, trivial_state: S) -> Self {
        Self {
            serializer,
            trivial_state,
        }
    }
}

impl<T, U> Drop for BatchSerializerLocal<T, U>
where
    T: Serialize,
    U: Serialize,
{
    fn drop(&mut self) {
        let remaining = self.serializer.remaining();
        self.save_bytes(remaining);
    }
}

impl<T, CI, I, S> BoardCallbackWithStates<T, CI, I> for BatchSerializerLocal<Vec<(CI, T)>, S>
where
    T: Send + Sync + Serialize,
    CI: Send + Sync + Serialize,
    S: Send + Sync + Serialize,
    I: rayon::iter::ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn execute(&mut self, states: I) {
        self.push(states.collect());
    }
}

impl<T, CI, I, S> BoardCallbackWithStates<T, CI, I> for StateSerializerLocal<Vec<(CI, T)>, S, T>
where
    T: Send + Sync + Serialize + std::cmp::PartialEq,
    CI: Send + Sync + Serialize,
    S: Send + Sync + Serialize,
    I: rayon::iter::ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn execute(&mut self, states: I) {
        self.serializer
            .push(states.filter(|ele| ele.1 != self.trivial_state).collect());
    }
}