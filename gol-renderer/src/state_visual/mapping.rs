use rgb;

pub trait CharMapping<T> {
    fn char_representation(&self, state: &T) -> char;
}

pub trait ColorMapping<T> {
    fn color_representation(&self, state: &T) -> rgb::RGBA16;
}

#[derive(Clone)]
pub struct DefaultCharMap {}

#[derive(Clone)]
pub struct DefaultColorMap {
    should_decay_alpha: bool,
}

impl DefaultCharMap {
    pub fn new() -> Self {
        Self {}
    }
}

impl DefaultColorMap {
    pub fn new() -> Self {
        Self {
            should_decay_alpha: false,
        }
    }

    pub fn new_decay_alpha() -> Self {
        Self {
            should_decay_alpha: true,
        }
    }

    pub fn should_decay_alpha(&self) -> bool {
        self.should_decay_alpha
    }
}
