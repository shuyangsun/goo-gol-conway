pub trait StateVisualMapping<T, U>: Send + Sync {
    fn to_visual(&self, state: &T) -> U;
}

#[derive(Clone)]
pub struct BinaryStateCharMap {}

#[derive(Clone)]
pub struct BinaryStateColorMap {}

#[derive(Clone)]
pub struct DiscreteStateCharMap {
    state_count: usize,
}

#[derive(Clone)]
pub struct DiscreteStateColorMap {
    state_count: usize,
    should_decay_alpha: bool,
}

impl BinaryStateCharMap {
    pub fn new() -> Self {
        Self {}
    }
}

impl BinaryStateColorMap {
    pub fn new() -> Self {
        Self {}
    }
}

impl DiscreteStateCharMap {
    pub fn new(state_count: usize) -> Self {
        Self { state_count }
    }

    pub fn state_count(&self) -> usize {
        self.state_count
    }
}

impl DiscreteStateColorMap {
    pub fn new(state_count: usize) -> Self {
        Self {
            state_count,
            should_decay_alpha: false,
        }
    }

    pub fn with_decaying_alpha(self) -> Self {
        let mut res = self;
        res.should_decay_alpha = true;
        res
    }

    pub fn state_count(&self) -> usize {
        self.state_count
    }

    pub fn should_decay_alpha(&self) -> bool {
        self.should_decay_alpha
    }
}
