use super::HISTORY_EXTENSION;
use bincode;
use flate2::read::GzDecoder;
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::{self, read_dir, File};
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct BatchDeserializerLocal<T, U> {
    path: String,
    idx_ranges: Vec<usize>,
    buffer_size: usize,
    buffer: Arc<Mutex<Vec<(U, Vec<(usize, Arc<T>)>)>>>, // U is header
    buffer_ranges: Arc<Mutex<Vec<usize>>>,
}

impl<T, U> BatchDeserializerLocal<T, U> {
    pub fn new(dir_path: &String) -> Self {
        let expanded = shellexpand::full(dir_path).unwrap();
        let path = Path::new(expanded.as_ref());
        let idx_ranges = match Self::construct_ranges(&path) {
            Ok(val) => val,
            Err(err) => panic!(err),
        };
        let default_buffer_size = 1;
        Self {
            path: String::from(path.to_str().unwrap()),
            idx_ranges,
            buffer_size: default_buffer_size,
            buffer: Arc::new(Mutex::new(Vec::with_capacity(default_buffer_size + 1))),
            buffer_ranges: Arc::new(Mutex::new(Vec::with_capacity(default_buffer_size + 1))),
        }
    }

    pub fn with_buffer_size(self, buffer_size: usize) -> Self {
        let mut res = self;
        res.buffer_size = buffer_size;
        res.buffer = Arc::new(Mutex::new(Vec::with_capacity(buffer_size + 1)));
        res.buffer_ranges = Arc::new(Mutex::new(Vec::with_capacity(buffer_size + 1)));
        res
    }

    pub fn get<'de>(&self, idx: usize) -> Option<Arc<T>>
    where
        T: Deserialize<'de>,
    {
        let left_idx = Self::find_range_left_idx(&idx, &self.idx_ranges);
        match left_idx {
            Some(buffer_idx) => {
                let start_idx = self.buffer_ranges.lock().unwrap()[buffer_idx];
                let buffer_for_file = &self.buffer.lock().unwrap()[buffer_idx].1;
                let data = &buffer_for_file[idx - start_idx];
                assert_eq!(data.0, idx);
                Some(Arc::clone(&data.1))
            }
            None => {
                // TODO: replace the whole array
                None
            }
        }
    }
}

impl<T, U> BatchDeserializerLocal<T, U> {
    fn data_for_idx<'de>(&self, idx: &usize) -> Option<(U, Vec<(usize, Arc<T>)>)>
    where
        T: Send + Sync + Deserialize<'de>,
        U: Deserialize<'de>,
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

    fn find_range_left_idx(idx: &usize, ranges: &Vec<usize>) -> Option<usize> {
        match ranges.binary_search(idx) {
            Ok(val) => {
                if val >= ranges.len() - 1 {
                    None
                } else {
                    Some(val)
                }
            }
            Err(val) => {
                if val > 0 && val < ranges.len() - 1 {
                    Some(val - 1)
                } else {
                    None
                }
            }
        }
    }
}
