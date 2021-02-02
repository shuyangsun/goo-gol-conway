use flate2::{write::GzEncoder, Compression};
use serde::Serialize;
use std::io::prelude::*;

pub struct IndexedBatchData {
    pub idx_beg: usize,
    pub idx_end: usize,
    pub data: Vec<u8>,
}

pub struct BatchIndexedSerializer<T, U> {
    batch_size: usize,
    iter_count: usize,
    history_buffer: Vec<(usize, T)>,
    header: Option<U>,
}

impl<T, U> BatchIndexedSerializer<T, U>
where
    T: Serialize,
    U: Serialize,
{
    pub fn new(batch_size: usize) -> Self {
        let batch_size = std::cmp::max(batch_size, 1);
        Self {
            batch_size,
            iter_count: 0,
            history_buffer: Vec::with_capacity(batch_size),
            header: None,
        }
    }

    pub fn with_header(self, header: U) -> Self {
        let mut res = self;
        res.header = Some(header);
        res
    }

    pub fn push(&mut self, item: T) -> Option<IndexedBatchData> {
        self.iter_count += 1;
        self.history_buffer.push((self.iter_count, item));
        if self.history_buffer.len() >= self.batch_size {
            let serialized = self.serialize_and_clear_buffer();
            Some(serialized)
        } else {
            None
        }
    }

    pub fn remaining(&mut self) -> Option<IndexedBatchData> {
        if self.history_buffer.is_empty() {
            None
        } else {
            Some(self.serialize_and_clear_buffer())
        }
    }

    fn serialize_and_clear_buffer(&mut self) -> IndexedBatchData {
        let idx_beg = self.iter_count - self.history_buffer.len();
        let idx_end = self.iter_count;
        let data = bincode::serialize(&(self.header.as_ref(), &self.history_buffer)).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&data).unwrap();
        let data = encoder.finish().unwrap();
        self.history_buffer = Vec::with_capacity(self.batch_size);
        IndexedBatchData {
            idx_beg,
            idx_end,
            data,
        }
    }
}

impl<T, U> Drop for BatchIndexedSerializer<T, U> {
    fn drop(&mut self) {
        if !self.history_buffer.is_empty() {
            eprintln!(
                "Dropping non-empty batch serializer with {} remaining items.",
                self.history_buffer.len()
            );
        }
    }
}
