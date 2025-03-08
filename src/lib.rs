pub mod convert;
pub mod modulus;

#[cfg(test)]
pub mod tests;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rns<const N: usize> {
    remainders: [u8; N],
}

impl<const N: usize> Rns<N> {
    pub const fn zero() -> Self {
        Rns { remainders: [0; N] }
    }

    pub const fn one() -> Self {
        Rns { remainders: [1; N] }
    }
}

impl<const N: usize> fmt::Display for Rns<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.remainders[..N.min(18)])
    }
}
