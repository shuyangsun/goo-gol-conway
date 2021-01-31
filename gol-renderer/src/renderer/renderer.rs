pub trait CellularAutomatonRenderer {
    fn need_run_on_main(&self) -> bool;
    fn run(&mut self);
}
