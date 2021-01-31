use crate::state_visual::mapping::StateVisualMapping;

pub trait CellularAutomatonRenderer<T, U>: Send + Sync {
    fn need_run_on_main(&self) -> bool;
    fn run(&mut self, visual_mapping: Box<dyn StateVisualMapping<T, U>>);
}
