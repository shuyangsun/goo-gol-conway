use crate::state_visual::mapping::StateVisualMapping;

pub trait CellularAutomatonRenderer: Send + Sync {
    fn need_run_on_main(&self) -> bool;
    fn run(&mut self);
}
