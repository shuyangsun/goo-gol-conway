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
pub struct DefaultColorMap {}

impl DefaultCharMap {
    pub fn new() -> Self {
        Self {}
    }
}

impl DefaultColorMap {
    pub fn new() -> Self {
        Self {}
    }
}
