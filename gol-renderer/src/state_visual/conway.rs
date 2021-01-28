#![feature(min_const_generics)]

use super::mapping::{CharMapping, ColorMapping, DefaultCharMap, DefaultColorMap};
use gol_core::{DiscreteState, ToPrimitive};
use num_traits::PrimInt;
use rgb::RGBA16;

const DEAD_STATE_CHAR: char = ' ';
const INT_STATE_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl<T> ColorMapping<DiscreteState<T, 2>> for DefaultColorMap
where
    T: PrimInt,
{
    fn color_representation(&self, state: &DiscreteState<T, 2>) -> RGBA16 {
        if state.val() <= T::zero() {
            RGBA16 {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }
        } else {
            RGBA16 {
                r: 0,
                g: u16::MAX,
                b: 0,
                a: u16::MAX,
            }
        }
    }
}

impl<T, const N: usize> CharMapping<DiscreteState<T, N>> for DefaultCharMap
where
    T: PrimInt + ToPrimitive,
{
    fn char_representation(&self, state: &DiscreteState<T, N>) -> char {
        assert!(N <= 11);
        if state.val() <= T::zero() {
            DEAD_STATE_CHAR
        } else {
            INT_STATE_CHARS[state.val().to_usize().unwrap() - 1]
        }
    }
}

impl<T, const N: usize> ColorMapping<DiscreteState<T, N>> for DefaultColorMap
where
    T: PrimInt + ToPrimitive,
{
    fn color_representation(&self, state: &DiscreteState<T, N>) -> RGBA16 {
        if state.val() <= T::zero() {
            RGBA16 {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }
        } else {
            let ratio = state.val().to_f64().unwrap() / (N - 1) as f64;
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
