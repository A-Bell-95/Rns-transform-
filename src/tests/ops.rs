use crate::Rns;
use rand::RngCore;

#[test]
fn test_add() {
    let a = Rns::<18>::new(192);
    let b = Rns::<18>::new(18);
    assert_eq!(a + b, Rns::new(210));
}

#[test]
fn test_add_random() {
    let mut rng = rand::rng();

    for _ in 0..100 {
        let a = rng.next_u32();
        let b = rng.next_u32();
        let ra = Rns::<18>::new(a as u64);
        let rb = Rns::<18>::new(b as u64);
        let res = ra + rb;
        let expected = a as u64 + b as u64;
        assert_eq!(res.into_u64(), expected);
    }
}
