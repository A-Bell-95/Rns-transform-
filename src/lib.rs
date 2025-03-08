pub mod convert;
pub mod modulus;
mod ops;

#[cfg(test)]
pub mod tests;

use std::fmt;

const PRIME_BASE_U64: [u64; 18] = [
    3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rns<const N: usize> {
    remainders: [u8; N],
}

impl<const N: usize> fmt::Display for Rns<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.remainders[..N.min(18)])
    }
}

impl<const N: usize> Rns<N> {
    pub const fn zero() -> Self {
        Rns { remainders: [0; N] }
    }

    pub const fn one() -> Self {
        Rns { remainders: [1; N] }
    }
}
