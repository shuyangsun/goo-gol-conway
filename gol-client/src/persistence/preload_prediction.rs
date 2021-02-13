use std::collections::HashSet;

pub trait PreloadPrediction<T>: Send + Sync {
    fn register(&mut self, value: &T);
    fn predict(&self) -> HashSet<T>;
}
