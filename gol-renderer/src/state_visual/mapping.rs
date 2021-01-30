use rgb;

pub trait CharMapping<T> {
    fn char_representation(&self, state: &T) -> char;
}

pub trait ColorMapping<T> {
    fn color_representation(&self, state: &T) -> rgb::RGBA16;
}

#[derive(Clone)]
pub struct ConwayStateCharMap {}

#[derive(Clone)]
pub struct ConwayStateColorMap {}

#[derive(Clone)]
pub struct DiscreteStateCharMap {
    state_count: usize,
}

#[derive(Clone)]
pub struct DiscreteStateColorMap {
    state_count: usize,
    should_decay_alpha: bool,
}

impl ConwayStateCharMap {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConwayStateColorMap {
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
