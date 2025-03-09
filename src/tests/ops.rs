use crate::Rns;
use rand::RngCore;

#[test]
fn test_add() {
    let a = Rns::<18>::new(192);
    let b = Rns::<18>::new(18);
    assert_eq!(a + b, Rns::new(210));

    let a = Rns::<18>::new(0);
    let b = Rns::<18>::new(1);
    assert_eq!(a + b, Rns::new(1));

    let a = Rns::<18>::new(100000);
    let b = Rns::<18>::new(10000000);
    assert_eq!(a + b, Rns::new(10100000));
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

#[test]
fn test_add_assign() {
    let mut a = Rns::<18>::new(192);
    let b = Rns::<18>::new(18);
    a += b;
    assert_eq!(a, Rns::new(210));

    let mut a = Rns::<18>::new(0);
    let b = Rns::<18>::new(1);
    a += b;
    assert_eq!(a, Rns::new(1));

    let mut a = Rns::<18>::new(100000);
    let b = Rns::<18>::new(10000000);
    a += b;
    assert_eq!(a, Rns::new(10100000));
}

#[test]
fn test_add_assign_random() {
    let mut rng = rand::rng();

    for _ in 0..100 {
        let a = rng.next_u32();
        let b = rng.next_u32();
        let mut ra = Rns::<18>::new(a as u64);
        let rb = Rns::<18>::new(b as u64);
        ra += rb;
        let expected = a as u64 + b as u64;
        assert_eq!(ra.into_u64(), expected);
        assert_eq!(ra, Rns::new(expected));
    }
}
