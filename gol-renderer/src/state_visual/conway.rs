use super::mapping::{CharMapping, ColorMapping, DefaultCharMap, DefaultColorMap};
use gol_core::{ConwayState, DiscreteState};
use num_traits::{PrimInt, ToPrimitive, Unsigned};
use rgb::RGBA16;

const DEAD_STATE_CHAR: char = ' ';
const INT_STATE_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
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

impl<T, const N: usize> CharMapping<DiscreteState<T, N>> for DefaultCharMap
where
    T: PrimInt + ToPrimitive + Unsigned,
{
    fn char_representation(&self, state: &DiscreteState<T, N>) -> char {
        assert!(N <= 11);
        if state.val() <= &T::zero() {
            DEAD_STATE_CHAR
        } else {
            INT_STATE_CHARS[state.val().to_usize().unwrap() - 1]
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
