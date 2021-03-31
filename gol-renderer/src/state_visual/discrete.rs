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
            let (hue_low, hue_high) = (-0.5, 0.55);
            let (sat_low, sat_high) = (0.5, 1.0);
            let (lum_low, lum_high) = (0.2, 0.5);

            let hue = hue_low + (hue_high - hue_low) * ratio;
            let sat = sat_low + (sat_high - sat_low) * ratio;
            let lum = lum_low + (lum_high - lum_low) * ratio;

            hsl_to_rbg(hue, sat, lum, ratio)
        }
    }
}

fn hsl_to_rbg(h: f64, s: f64, l: f64, a: f64) -> RGBA16 {
    let (mut r, mut g, mut b) = (l, l, l);
    if s != 0. {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
        let p = 2. * l - q;
        r = hue_to_rgb(p, q, h + 0.33);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 0.33);
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
    } else if t > 1. {
        t -= 1.;
    }

    if t >= 0.66 {
        return p;
    } else if t >= 0.5 {
        return p + (q - p) * (0.66 - t) * 6.;
    } else if t >= 0.33 {
        return q;
    }
    return p + (q - p) * t * 6.;
}
