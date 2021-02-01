use super::batch_serializer::BatchIndexedSerializer;
use serde::Serialize;
use std::path::Path;

pub struct BatchSerializerLocal<T, U> {
    path: Box<Path>,
    header: U,
    serializer: BatchIndexedSerializer<T>,
}

impl<T, U> BatchSerializerLocal<T, U> {
    pub fn new(batch_size: usize, dir_path: String) -> Self {}
}
