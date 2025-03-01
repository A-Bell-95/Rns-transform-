use crate::modulus::find_suitable_modulus;

#[test]
fn test_find_suitable_modulus_256() {
    let n = 256;
    for _ in 0..100 {
        let cand = find_suitable_modulus(n).unwrap();
        assert!(is_prime_strictly(cand));
        assert!((cand - 1) % (2 * n) == 0);
        assert!(cand > 1 << 39);
    }
}

#[test]
fn test_find_suitable_modulus_512() {
    let n = 512;
    for _ in 0..100 {
        let cand = find_suitable_modulus(n).unwrap();
        assert!(is_prime_strictly(cand));
        assert!((cand - 1) % (2 * n) == 0);
        assert!(cand > 1 << 39);
    }
}

#[test]
fn test_find_suitable_modulus_1024() {
    let n = 1024;
    for _ in 0..100 {
        let cand = find_suitable_modulus(n).unwrap();
        assert!(is_prime_strictly(cand));
        assert!((cand - 1) % (2 * n) == 0);
        assert!(cand > 1 << 39);
    }
}

fn is_prime_strictly(num: u64) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }

    true
}
