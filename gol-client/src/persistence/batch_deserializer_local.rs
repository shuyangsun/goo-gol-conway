use super::HISTORY_EXTENSION;
use bincode;
use flate2::read::GzDecoder;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::fs::{self, read_dir, File};
use std::io::Read;
use std::iter::FromIterator;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

struct MultiRowData<T> {
    start: usize,
    end: usize,
    data: Vec<Arc<T>>,
}

pub struct BatchDeserializerLocal<T, U> {
    path: String,
    idx_ranges: Vec<usize>,
    forward_buffer_size: usize,
    backward_buffer_size: usize,
    buffer: Arc<RwLock<Vec<(U, MultiRowData<T>)>>>, // U is header
    buffer_append_handles: Arc<RwLock<Vec<JoinHandle<MultiRowData<T>>>>>,
}

impl<T, U> BatchDeserializerLocal<T, U> {
    pub fn new(dir_path: &String) -> Self {
        let expanded = shellexpand::full(dir_path).unwrap();
        let path = Path::new(expanded.as_ref());
        let idx_ranges = match Self::construct_ranges(&path) {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };
        let res = Self {
            path: String::from(path.to_str().unwrap()),
            idx_ranges,
            forward_buffer_size: 1,
            backward_buffer_size: 1,
            buffer: Arc::new(RwLock::new(Vec::new())),
            buffer_append_handles: Arc::new(RwLock::new(Vec::new())),
        };

        res.with_forward_buffer_size(2).with_backward_buffer_size(1)
    }

    pub fn with_forward_buffer_size(self, size: usize) -> Self {
        let mut res = self;
        res.forward_buffer_size = size;
        let arr_len = 1 + res.backward_buffer_size + size;
        res.buffer = Arc::new(RwLock::new(Vec::with_capacity(arr_len)));
        res
    }

    pub fn with_backward_buffer_size(self, size: usize) -> Self {
        let mut res = self;
        res.backward_buffer_size = size;
        let arr_len = 1 + size + res.forward_buffer_size;
        res.buffer = Arc::new(RwLock::new(Vec::with_capacity(arr_len)));
        res
    }

    pub fn get(&self, idx: usize) -> Option<Arc<T>>
    where
        T: DeserializeOwned,
    {
        if &idx >= self.idx_ranges.last().unwrap() || &idx < self.idx_ranges.first().unwrap() {
            return None;
        }

        // Try to find index in current buffer.
        let (mut left_buffer_idx, mut left_data_idx): (Option<usize>, Option<usize>) = (None, None);
        loop {
            let read = self.buffer_ranges.try_read();
            if read.is_ok() {
                let buffer_ranges_ref: &Vec<usize> = read.unwrap().as_ref();
                left_buffer_idx = Self::find_range_left_idx(&idx, buffer_ranges_ref);
                if left_buffer_idx.is_some() {
                    left_data_idx = Some(buffer_ranges_ref[left_buffer_idx.unwrap()]);
                }
                break;
            }
        }

        match left_buffer_idx {
            // Cache hit
            Some(range_idx) => {
                let start_idx = self.buffer_ranges.read().unwrap()[buffer_idx];
                let buffer_for_file = &self.buffer.read().unwrap()[buffer_idx].1;
                let data = &buffer_for_file[idx - start_idx];
                assert_eq!(data.0, idx);

                let desired_indices = self.desired_buffer_indices(buffer_idx);

                Some(Arc::clone(&data.1))
                // TODO: preload data to buffer
            }
            // Cache miss
            None => None,
        }
    }
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
