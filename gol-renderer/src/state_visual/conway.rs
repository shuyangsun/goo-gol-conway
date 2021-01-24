use super::mapping::{CharMapping, ColorMapping, DefaultCharMap, DefaultColorMap};
use gol_core::ConwayState;
use rgb::RGBA8;

const CONWAY_STATE_ALIVE_CHAR: char = '0';
const CONWAY_STATE_DEAD_CHAR: char = ' ';

impl CharMapping<ConwayState> for DefaultCharMap {
    fn char_representation(&self, state: &ConwayState) -> char {
        match state {
            ConwayState::Alive => CONWAY_STATE_ALIVE_CHAR,
            ConwayState::Dead => CONWAY_STATE_DEAD_CHAR,
        }
    }
}

impl ColorMapping<ConwayState> for DefaultColorMap {
    fn color_representation(&self, state: &ConwayState) -> RGBA8 {
        match state {
            ConwayState::Alive => RGBA8 {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            },
            ConwayState::Dead => RGBA8 {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        }
    }
}
