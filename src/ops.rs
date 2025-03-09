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
        for (i, (&r, &p)) in other
            .remainders
            .iter()
            .zip(PRIME_BASE_U64.iter())
            .take(N)
            .enumerate()
        {
            let new_r = if self.remainders[i] >= r {
                self.remainders[i] as u16 - r as u16
            } else {
                p as u16 - r as u16 + self.remainders[i] as u16
            };
            self.remainders[i] = new_r as u8;
        }
    }
}
