mod convert;
mod modulus;

use crate::Rns;

#[test]
fn test_rns_zero() {
    assert_eq!(
        Rns::zero(),
        Rns {
            remainders: [0; 18]
        }
    )
}

#[test]
fn test_rns_one() {
    assert_eq!(
        Rns::one(),
        Rns {
            remainders: [1; 18]
        }
    )
}
