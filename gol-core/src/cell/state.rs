use num_traits::{FromPrimitive, PrimInt, Unsigned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscreteState<T, const N: usize>
where
    T: PrimInt + Unsigned,
{
    val: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConwayState {
    Alive,
    Dead,
}

impl<T, const N: usize> DiscreteState<T, N>
where
    T: PrimInt + Unsigned,
{
    pub fn new() -> Self
    where
        T: FromPrimitive,
    {
        Self {
            val: T::from_usize(N - 1).unwrap(),
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn decay(&self) -> Self {
        if self.val() <= &T::zero() {
            self.clone()
        } else {
            Self {
                val: *self.val() - T::one(),
            }
        }
    }
}
