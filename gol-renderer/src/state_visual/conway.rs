use super::mapping::{CharMapping, ColorMapping, DefaultCharMap, DefaultColorMap};
use gol_core::ConwayState;
use rgb::RGBA16;

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
    fn color_representation(&self, state: &ConwayState) -> RGBA16 {
        match state {
            ConwayState::Alive => RGBA16 {
                r: 0,
                g: u16::MAX,
                b: 0,
                a: u16::MAX,
            },
            ConwayState::Dead => RGBA16 {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        }
    }
}
