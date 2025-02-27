use crate::convert;

#[test]
fn test_zero_to_rns() {
    let zero = 0;
    let rns = convert::to_rns(zero);
    println!("{rns:?}");
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], rns);
}

#[test]
fn test_one_to_rns() {
    let one = 1;
    let rns = convert::to_rns(one);
    println!("{rns:?}");
    assert_eq!(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], rns);
}

#[test]
fn test_5000_to_rns() {
    let five_thousand = 5_000;
    let rns = convert::to_rns(five_thousand);
    println!("{rns:?}");
    assert_eq!(vec![2, 0, 2, 6, 8, 2, 3, 9, 12, 9, 5, 39, 12, 18, 18, 44, 59, 42], rns);
}

#[test]
fn test_89_000_000_to_rns() {
    let _89_000_000 = 89_000_000;
    let rns = convert::to_rns(_89_000_000);
    println!("{rns:?}");
    assert_eq!(vec![2, 0, 5, 1, 11, 2, 10, 5, 15, 23, 15, 29, 19, 1, 15, 34, 24, 14], rns);
}

#[test]
fn test_zero_from_rns() {
    let rns = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let result = convert::from_rns(&rns);
    println!("{result}");
    assert_eq!(0, result);
}

#[test]
fn test_one_from_rns() {
    let rns = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let result = convert::from_rns(&rns);
    println!("{result}");
    assert_eq!(1, result);
}

#[test]
fn test_5000_from_rns() {
    let rns = vec![2, 0, 2, 6, 8, 2, 3, 9, 12, 9, 5, 39, 12, 18, 18, 44, 59, 42];
    let result = convert::from_rns(&rns);
    println!("{result}");
    assert_eq!(5000, result);
}

#[test]
fn test_89_000_000_from_rns() {
    let rns = vec![2, 0, 5, 1, 11, 2, 10, 5, 15, 23, 15, 29, 19, 1, 15, 34, 24, 14];
    let result = convert::from_rns(&rns);
    println!("{result}");
    assert_eq!(89_000_000, result);
    println!("{} -- {}", u64::MIN, u64::MAX);
}

#[test]
fn test_184_467_440_737_095_516_to_rns() {
    let _184_467_440_737_095_516 = 184_467_440_737_095_516;
    let rns = convert::to_rns(_184_467_440_737_095_516);
    println!("{rns:?}");
    assert_eq!(vec![0, 1, 0, 0, 0, 16, 4, 16, 14, 0, 34, 0, 11, 25, 9, 17, 0, 65], rns);
}

#[test]
fn test_184_467_440_737_095_516_from_rns() {
    let rns = vec![0, 1, 0, 0, 0, 16, 4, 16, 14, 0, 34, 0, 11, 25, 9, 17, 0, 65];
    let result = convert::from_rns(&rns);
    println!("{result}");
    assert_eq!(184_467_440_737_095_516, result);
}
