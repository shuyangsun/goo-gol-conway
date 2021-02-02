use super::batch_serializer::{BatchIndexedSerializer, IndexedBatchData};
use gol_core::{BoardCallbackWithStates, IndexedDataOwned};
use serde::Serialize;
use shellexpand;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct BatchSerializerLocal<T, U>
where
    T: Serialize,
    U: Serialize,
{
    path: String,
    serializer: BatchIndexedSerializer<T, U>,
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
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&data.data).unwrap();
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
