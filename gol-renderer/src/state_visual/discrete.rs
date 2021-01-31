use super::mapping::{
    BinaryStateCharMap, BinaryStateColorMap, DiscreteStateCharMap, DiscreteStateColorMap,
    StateVisualMapping,
};
use gol_core::BinaryState;
use num_traits::{PrimInt, ToPrimitive, Unsigned};
use rgb::RGBA16;

const DEAD_STATE_CHAR: char = ' ';
const INT_STATE_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const BINARY_STATE_ALIVE_CHAR: char = '0';
const BINARY_STATE_DEAD_CHAR: char = ' ';

impl StateVisualMapping<BinaryState, char> for BinaryStateCharMap {
    fn to_visual(&self, state: &BinaryState) -> char {
        match state {
            BinaryState::Alive => BINARY_STATE_ALIVE_CHAR,
            BinaryState::Dead => BINARY_STATE_DEAD_CHAR,
        }
    }
}

impl<T> StateVisualMapping<T, char> for DiscreteStateCharMap
where
    T: PrimInt + ToPrimitive + Unsigned,
{
    fn to_visual(&self, state: &T) -> char {
        assert!(self.state_count() <= 11);
        if state <= &T::zero() {
            DEAD_STATE_CHAR
        } else {
            INT_STATE_CHARS[state.to_usize().unwrap() - 1]
        }
    }
}

impl StateVisualMapping<BinaryState, RGBA16> for BinaryStateColorMap {
    fn to_visual(&self, state: &BinaryState) -> RGBA16 {
        match state {
            BinaryState::Alive => RGBA16 {
                r: 0,
                g: u16::MAX,
                b: 0,
                a: u16::MAX,
            },
            BinaryState::Dead => RGBA16 {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        }
    }
}

impl<T> StateVisualMapping<T, RGBA16> for DiscreteStateColorMap
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    fn to_visual(&self, state: &T) -> RGBA16 {
        if state <= &T::zero() {
            RGBA16 {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }
        } else {
            let ratio = state.to_f64().unwrap() / (self.state_count() - 1) as f64;
            let green = (u16::MAX as f64 * ratio).ceil() as u16;
            let red = (u16::MAX as f64 * (1.0 - ratio)).floor() as u16;
            RGBA16 {
                r: red,
                g: green,
                b: 0,
                a: green,
            }
        }
    }
}
