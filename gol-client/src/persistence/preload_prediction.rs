use std::collections::HashSet;

pub trait PreloadPrediction<T> {
    fn register(&mut self, value: &T);
    fn predict(&self) -> HashSet<T>;
}
