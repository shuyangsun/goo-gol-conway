use serde::Serialize;

pub struct BatchIndexedSerializer<T> {
    batch_size: usize,
    iter_count: usize,
    history_buffer: Vec<(usize, T)>,
}

impl<T> BatchIndexedSerializer<T> {
    pub fn new(batch_size: usize) -> Self {
        let batch_size = std::cmp::max(batch_size, 1);
        Self {
            batch_size,
            iter_count: 0,
            history_buffer: Vec::with_capacity(batch_size),
        }
    }

    pub fn push(&mut self, item: T) -> Option<Vec<u8>>
    where
        T: Serialize,
    {
        self.history_buffer.push((self.iter_count, item));
        if self.history_buffer.len() >= self.batch_size {
            let serialized = self.serialize_history();
            self.history_buffer = Vec::with_capacity(self.batch_size);
            Some(serialized)
        } else {
            None
        }
    }

    pub fn remaining(&mut self) -> Option<Vec<u8>>
    where
        T: Serialize,
    {
        if self.history_buffer.is_empty() {
            None
        } else {
            Some(self.serialize_history())
        }
    }

    fn serialize_history(&self) -> Vec<u8>
    where
        T: Serialize,
    {
        bincode::serialize(&self.history_buffer).unwrap()
    }
}

impl<T> Drop for BatchIndexedSerializer<T> {
    fn drop(&mut self) {
        if !self.history_buffer.is_empty() {
            eprintln!(
                "Dropping non-empty batch serializer with {} remaining items.",
                self.history_buffer.len()
            );
        }
    }
}
