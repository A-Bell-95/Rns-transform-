use super::{Rns, PRIME_BASE_U64};
use std::ops::{Add, AddAssign, Sub, SubAssign};

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
        for ((rem, &oth_rem), &p) in self
            .remainders
            .iter_mut()
            .zip(other.remainders.iter())
            .zip(PRIME_BASE_U64.iter())
            .take(N)
        {
            *rem = ((*rem as u16 + oth_rem as u16) % (p as u16)) as u8;
        }
    }
}

impl<const N: usize> Sub for Rns<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut sum = self;
        sum -= other;
        sum
    }
}

impl<const N: usize> SubAssign for Rns<N> {
    fn sub_assign(&mut self, other: Self) {
        for ((rem, &oth_rem), &p) in self
            .remainders
            .iter_mut()
            .zip(other.remainders.iter())
            .zip(PRIME_BASE_U64.iter())
            .take(N)
        {
            *rem = if *rem >= oth_rem {
                *rem as u16 - oth_rem as u16
            } else {
                p as u16 - oth_rem as u16 + *rem as u16
            } as u8
        }
    }
}
