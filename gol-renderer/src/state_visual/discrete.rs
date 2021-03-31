use super::mapping::{DiscreteStateCharMap, DiscreteStateColorMap, StateVisualMapping};
use num_traits::{PrimInt, ToPrimitive, Unsigned};
use rgb::RGBA16;

const DEAD_STATE_CHAR: char = ' ';
const INT_STATE_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

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
            let hue = 1. - ratio;
            hsl_to_rbg(hue, 1.0, 0.5, ratio)
        }
    }
}

fn hsl_to_rbg(h: f64, s: f64, l: f64, a: f64) -> RGBA16 {
    let (mut r, mut g, mut b) = (l, l, l);
    if s > 0. {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
        let p = 2. * l - q;
        r = hue_to_rgb(p, q, h + 1. / 3.);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1. / 3.);
    }

    let max = u16::MAX as f64;
    RGBA16 {
        r: (r * max) as u16,
        g: (g * max) as u16,
        b: (b * max) as u16,
        a: (a * max) as u16,
    }
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0. {
        t += 1.;
    }
    if t > 1. {
        t -= 1.;
    }
    if t < 1. / 6. {
        return p + (q - p) * t * 6.;
    }
    if t < 1. / 2. {
        return q;
    }
    if t < 2. / 3. {
        return p + (q - p) * (2. / 3. - t) * 6.;
    }
    return p;
}
