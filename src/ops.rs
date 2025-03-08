use super::{Rns, PRIME_BASE_U64};
use std::ops::Add;

impl<const N: usize> Add for Rns<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = self.remainders;
        for (i, (&r, &p)) in other
            .remainders
            .iter()
            .zip(PRIME_BASE_U64.iter())
            .take(N)
            .enumerate()
        {
            sum[i] = ((sum[i] as u16 + r as u16) % (p as u16)) as u8;
        }

        Rns { remainders: sum }
    }
}
