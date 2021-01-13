pub trait EvolutionStrategy<'a, 't, T, I>: Send + Sync
where
    't: 'a,
    T: 't,
    I: Iterator<Item = (usize, &'a T)>,
{
    fn next_state(&self, idx: usize, cur_state: &'a T, neighbors: I) -> T;
}
