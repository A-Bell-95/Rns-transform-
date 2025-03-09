use super::{Rns, PRIME_BASE_U64};
use std::ops::{Add, AddAssign};

impl<const N: usize> Add for Rns<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = self;
        sum += other;
        sum
    }
}

impl<const N: usize> AddAssign for Rns<N> {
    fn add_assign(&mut self, other: Self) {
        for (i, (&r, &p)) in other
            .remainders
            .iter()
            .zip(PRIME_BASE_U64.iter())
            .take(N)
            .enumerate()
        {
            self.remainders[i] = ((self.remainders[i] as u16 + r as u16) % (p as u16)) as u8;
        }
    }
}
