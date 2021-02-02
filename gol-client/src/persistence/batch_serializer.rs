use serde::Serialize;

pub struct IndexedBatchData {
    pub idx_beg: usize,
    pub idx_end: usize,
    pub data: Vec<u8>,
}

pub struct BatchIndexedSerializer<T, U, W> {
    batch_size: usize,
    iter_count: usize,
    history_buffer: Vec<(usize, T)>,
    header: Option<U>,
    footer: Option<W>,
}

impl<T, U, W> BatchIndexedSerializer<T, U, W>
where
    T: Serialize,
    U: Serialize,
    W: Serialize,
{
    pub fn new(batch_size: usize) -> Self {
        let batch_size = std::cmp::max(batch_size, 1);
        Self {
            batch_size,
            iter_count: 0,
            history_buffer: Vec::with_capacity(batch_size),
            header: None,
            footer: None,
        }
    }

    pub fn with_header(self, header: U) -> Self {
        let mut res = self;
        res.header = Some(header);
        res
    }

    pub fn with_footer(self, footer: W) -> Self {
        let mut res = self;
        res.footer = Some(footer);
        res
    }

    pub fn push(&mut self, item: T) -> Option<IndexedBatchData> {
        self.iter_count += 1;
        self.history_buffer.push((self.iter_count, item));
        if self.history_buffer.len() >= self.batch_size {
            let serialized = self.serialize_history();
            self.history_buffer = Vec::with_capacity(self.batch_size);
            Some(serialized)
        } else {
            None
        }
    }

    pub fn remaining(&mut self) -> Option<IndexedBatchData> {
        if self.history_buffer.is_empty() {
            None
        } else {
            Some(self.serialize_history())
        }
    }

    fn serialize_history(&self) -> IndexedBatchData {
        let idx_beg = self.iter_count - self.history_buffer.len();
        let idx_end = self.iter_count;
        let data = bincode::serialize(&(
            self.header.as_ref(),
            &self.history_buffer,
            self.footer.as_ref(),
        ))
        .unwrap();
        IndexedBatchData {
            idx_beg,
            idx_end,
            data,
        }
    }
}

impl<T, U, W> Drop for BatchIndexedSerializer<T, U, W> {
    fn drop(&mut self) {
        if !self.history_buffer.is_empty() {
            eprintln!(
                "Dropping non-empty batch serializer with {} remaining items.",
                self.history_buffer.len()
            );
        }
    }
}
