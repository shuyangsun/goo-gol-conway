use super::batch_serializer::{BatchIndexedSerializer, IndexedBatchData};
use super::HISTORY_EXTENSION;
use gol_core::{BoardCallbackWithStates, IndexedDataOwned};
use rayon::prelude::*;
use serde::Serialize;
use shellexpand;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::thread::{self, JoinHandle};

pub struct BatchSerializerLocal<T, U>
where
    T: Serialize,
    U: Serialize,
{
    path: String,
    serializer: BatchIndexedSerializer<T, U>,
    last_handle: Option<JoinHandle<()>>,
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
            last_handle: None,
        }
    }

    pub fn push(&mut self, content: T) {
        let bytes = self.serializer.push(content);
        self.save_bytes(bytes);
    }

    fn save_bytes(&mut self, bytes: Option<IndexedBatchData>) {
        if bytes.is_none() {
            return;
        }
        let data = bytes.unwrap();
        let file_name = format!("{}_{}.{}", data.idx_beg, data.idx_end, HISTORY_EXTENSION);
        let file_path = Path::new(&self.path).join(&file_name);
        let file = File::create(&file_path).unwrap();
        let mut buffer = BufWriter::new(file);
        self.wait_on_last_handle();
        self.last_handle = Some(thread::spawn(move || {
            buffer.write_all(&data.data).unwrap();
            buffer.flush().unwrap();
        }));
    }

    fn wait_on_last_handle(&mut self) {
        if self.last_handle.is_none() {
            return;
        }
        self.last_handle.take().unwrap().join().unwrap();
        self.last_handle = None;
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
        self.wait_on_last_handle();
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
