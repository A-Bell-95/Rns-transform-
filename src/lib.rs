pub mod convert;
pub mod modulus;
pub mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rns<const N: usize> {
    remainders: [u8; N],
}
