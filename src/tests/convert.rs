use crate::Rns;

#[test]
fn test_zero_to_rns() {
    let expected = Rns {
        remainders: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    assert_eq!(Rns::new(0), expected);
}

#[test]
fn test_one_to_rns() {
    let expected = Rns {
        remainders: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    };
    assert_eq!(Rns::new(1), expected);
}

#[test]
fn test_5000_to_rns() {
    let expected = Rns {
        remainders: [2, 0, 2, 6, 8, 2, 3, 9, 12, 9, 5, 39, 12, 18, 18, 44, 59, 42],
    };
    assert_eq!(Rns::new(5000), expected);
}

#[test]
fn test_89_000_000_to_rns() {
    let expected = Rns {
        remainders: [
            2, 0, 5, 1, 11, 2, 10, 5, 15, 23, 15, 29, 19, 1, 15, 34, 24, 14,
        ],
    };
    assert_eq!(Rns::new(89_000_000), expected);
}

#[test]
fn test_184_467_440_737_095_516_to_rns() {
    let expected = Rns {
        remainders: [0, 1, 0, 0, 0, 16, 4, 16, 14, 0, 34, 0, 11, 25, 9, 17, 0, 65],
    };
    assert_eq!(Rns::new(184_467_440_737_095_516), expected);
}

#[test]
fn test_zero_from_rns() {
    let rns = Rns {
        remainders: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    let result: Result<u64, ()> = rns.try_into();
    assert_eq!(result, Ok(0));
}

#[test]
fn test_one_from_rns() {
    let rns = Rns {
        remainders: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    };
    let result: u64 = rns.into_u64();
    assert_eq!(result, 1);
}

#[test]
fn test_5000_from_rns() {
    let rns = Rns {
        remainders: [2, 0, 2, 6, 8, 2, 3, 9, 12, 9, 5, 39, 12, 18, 18, 44, 59, 42],
    };
    let result: u64 = rns.into_u64();
    assert_eq!(result, 5000);
}

#[test]
fn test_89_000_000_from_rns() {
    let rns = Rns {
        remainders: [
            2, 0, 5, 1, 11, 2, 10, 5, 15, 23, 15, 29, 19, 1, 15, 34, 24, 14,
        ],
    };
    let result: u64 = rns.into_u64();
    assert_eq!(result, 89_000_000);
}

#[test]
fn test_184_467_440_737_095_516_from_rns() {
    let rns = Rns {
        remainders: [0, 1, 0, 0, 0, 16, 4, 16, 14, 0, 34, 0, 11, 25, 9, 17, 0, 65],
    };
    let result: u64 = rns.into_u64();
    assert_eq!(result, 184_467_440_737_095_516);
}
