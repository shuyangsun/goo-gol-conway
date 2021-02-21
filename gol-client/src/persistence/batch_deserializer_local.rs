use super::HISTORY_EXTENSION;
use super::{
    adjacent_index_prediction::AdjacentIndexPrediction,
    preload_cache::{PreloadCache, PreloadCacheDelegate},
};
use bincode;
use flate2::read::GzDecoder;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::fs::{self, read_dir, File};
use std::io::Read;
use std::iter::FromIterator;
use std::path::Path;
use std::sync::Arc;

struct DirFileDelegate {
    path: String,
    idx_ranges: Vec<usize>,
}

impl DirFileDelegate {
    pub fn new(dir_path: &String) -> Self {
        let expanded = shellexpand::full(dir_path).unwrap();
        let path = Path::new(expanded.as_ref());
        let idx_ranges = match Self::construct_ranges(&path) {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };

        Self {
            path: String::from(path.to_str().unwrap()),
            idx_ranges,
        }
    }
}

impl<T> PreloadCacheDelegate<usize, T> for DirFileDelegate
where
    T: DeserializeOwned,
{
    fn get(&self, key: &usize) -> Option<T> {
        if key >= &(self.idx_ranges.len() - 1) {
            return None;
        }
        let (start, end) = (self.idx_ranges[*key], self.idx_ranges[key + 1]);
        let file_name = format!("{}_{}.{}", start, end, HISTORY_EXTENSION);
        let file_path = Path::new(&self.path).join(&file_name);

        let mut file = File::open(&file_path).expect("File not found.");
        let metadata = fs::metadata(&file_path).expect("Cannot read file metadata.");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("Cannot read file.");
        let mut decoder = GzDecoder::new(&buffer[..]);

        // Uncompressed data should be larger, but good starting size.
        let mut byte_data = Vec::with_capacity(metadata.len() as usize);
        decoder.read(&mut byte_data[..]);

        let res = bincode::deserialize(&byte_data[..]).expect("Cannot deserialize data.");

        Some(res)
    }
}

impl DirFileDelegate {
    fn construct_ranges(path: &Path) -> Result<Vec<usize>, &'static str> {
        if !path.is_dir() {
            return Err("Path specified for deserialization is not a directory.");
        }
        let mut res_set = HashSet::new();
        for ele in read_dir(path).unwrap() {
            let entry = ele.unwrap();
            let cur = entry.path();
            if !path.is_dir() && path.extension().unwrap() == HISTORY_EXTENSION {
                let file_name = cur.file_name().unwrap().to_str().unwrap();
                let split: Vec<&str> = file_name.split("_").collect();
                let start: usize = split[0].parse().expect("Expected integer in file name.");
                let end: usize = split[1].parse().expect("Expected integer in file name.");
                res_set.insert(start);
                res_set.insert(end);
            }
        }
        let mut res_vec: Vec<usize> = res_set.into_iter().collect();
        res_vec.sort();
        Ok(res_vec)
    }
}

pub struct BatchDeserializerLocal<T, U> {
    cache: PreloadCache<usize, (T, Vec<(usize, U)>)>,
    idx_ranges: Vec<usize>,
}

impl<T, U> BatchDeserializerLocal<T, U> {
    fn desired_buffer_indices(&self, buffer_idx: &usize) -> Vec<usize> {
        let idx = *buffer_idx;

        let mut start = 0usize;
        if idx > self.buffer_size {
            start = idx - self.buffer_size;
        }
        let end = std::cmp::min(self.idx_ranges.len() - 1, idx + self.buffer_size);
        Vec::from_iter(start..end)
    }

    fn data_for_idx(&self, idx: &usize) -> Option<(U, MultiRowData<T>)>
    where
        T: Send + Sync + DeserializeOwned,
        U: DeserializeOwned,
    {
        let byte_data = self.byte_data_for_idx(idx);
        if byte_data.is_none() {
            return None;
        }
        let byte_data = byte_data.unwrap();
        let deserialized: (U, Vec<(usize, T)>) =
            bincode::deserialize(&byte_data[..]).expect("Cannot deserialize data.");
        let (header, history) = deserialized;
        let history = history
            .into_par_iter()
            .map(|ele| (ele.0, Arc::new(ele.1)))
            .collect();
        Some((header, history))
    }

    fn byte_data_for_idx(&self, idx: &usize) -> Option<Vec<u8>> {
        let file_path = self.path_for_idx(idx);
        if file_path.is_none() {
            return None;
        }
        let file_path = file_path.unwrap();
        let mut file = File::open(&file_path).expect("File not found.");
        let metadata = fs::metadata(&file_path).expect("Cannot read file metadata.");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("Cannot read file.");
        let mut decoder = GzDecoder::new(&buffer[..]);

        // Uncompressed data should be larger, but good starting size.
        let mut res = Vec::with_capacity(metadata.len() as usize);
        decoder.read(&mut res[..]);

        Some(res)
    }

    fn path_for_idx(&self, idx: &usize) -> Option<String> {
        let range_idx = Self::find_range_left_idx(idx, &self.idx_ranges);
        match range_idx {
            Some(idx) => {
                let (start, end) = (self.idx_ranges[idx], self.idx_ranges[idx + 1]);
                let file_name = format!("{}_{}.{}", start, end, HISTORY_EXTENSION);
                let path_to_file = Path::new(&self.path).join(&file_name);
                Some(String::from(path_to_file.to_str().unwrap()))
            }
            None => None,
        }
    }
}
