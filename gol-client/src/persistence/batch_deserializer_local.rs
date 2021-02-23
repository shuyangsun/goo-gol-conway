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
        let idx_ranges = match construct_ranges(&path) {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };

        Self {
            path: String::from(path.to_str().unwrap()),
            idx_ranges,
        }
    }
}

impl<T, U> PreloadCacheDelegate<usize, (Arc<Option<T>>, Vec<Arc<(usize, U)>>)> for DirFileDelegate
where
    T: Send + Sync + DeserializeOwned,
    U: Send + Sync + DeserializeOwned,
{
    fn get(&self, key: &usize) -> Option<(Arc<Option<T>>, Vec<Arc<(usize, U)>>)> {
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
        decoder.read(&mut byte_data[..]).unwrap();

        let res: (Option<T>, Vec<(usize, U)>) =
            bincode::deserialize(&byte_data[..]).expect("Cannot deserialize data.");
        let res_arc: Vec<Arc<(usize, U)>> = res
            .1
            .into_par_iter()
            .map(|ele| Arc::new((ele.0, ele.1)))
            .collect();

        Some((Arc::new(res.0), res_arc))
    }
}

pub struct BatchDeserializerLocal<T, U> {
    cache: PreloadCache<usize, (Arc<Option<T>>, Vec<Arc<(usize, U)>>)>,
    idx_ranges: Vec<usize>,
}

impl<T, U> BatchDeserializerLocal<T, U> {
    pub fn new(path: &String) -> Self
    where
        T: Send + Sync + DeserializeOwned,
        U: Send + Sync + DeserializeOwned,
    {
        let file_get_delegate = DirFileDelegate::new(path);
        let predictor = AdjacentIndexPrediction::new()
            .with_history_size(10)
            .with_backward_size(1)
            .with_forward_size(2);
        let cache = PreloadCache::new(Box::new(predictor), Box::new(file_get_delegate));

        let expanded = shellexpand::full(path).unwrap();
        let path = Path::new(expanded.as_ref());
        let idx_ranges = match construct_ranges(&path) {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };

        Self { cache, idx_ranges }
    }

    pub fn get(&self, idx: usize) -> Option<(Arc<Option<T>>, Arc<(usize, U)>)>
    where
        T: 'static + Send + Sync + DeserializeOwned,
        U: 'static + Send + Sync + DeserializeOwned,
    {
        let file_idx = Self::find_range_left_idx(&idx, &self.idx_ranges);
        match file_idx {
            Some(file_idx) => {
                let start = self.idx_ranges[file_idx];
                let inner_idx = idx - start;
                let file_res = self.cache.get(&file_idx);
                match file_res {
                    Some(file_res) => {
                        Some((Arc::clone(&file_res.0), Arc::clone(&file_res.1[inner_idx])))
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}

impl<T, U> BatchDeserializerLocal<T, U> {
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

fn construct_ranges(path: &Path) -> Result<Vec<usize>, &'static str> {
    if !path.is_dir() {
        return Err("Path specified for deserialization is not a directory.");
    }
    let mut res_set = HashSet::new();
    for ele in read_dir(path).unwrap() {
        let entry = ele.unwrap();
        let cur = entry.path();
        if !cur.is_dir() && cur.extension().unwrap() == HISTORY_EXTENSION {
            let file_name = cur.file_stem().unwrap().to_str().unwrap();
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
