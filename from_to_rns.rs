fn to_rns(x: u64, moduli: &[u64]) -> Vec<u64> {
    moduli.iter().map(|&m| x % m).collect()
}

fn from_rns(remainders: &[u64], moduli: &[u64]) -> u64 {
    let mut result: u128 = 0;
    let product: u128 = moduli.iter().map(|&m| m as u128).product();

    for (&r, &m) in remainders.iter().zip(moduli.iter()) {
        let mi = product / (m as u128);
        if let Some(inv) = mod_inverse(mi as u64, m) {
            result = (result + (r as u128) * mi % product * inv as u128 % product) % product;
        } else {
            panic!("Модули должны быть взаимно простыми!");
        }
    }

    result as u64
}

fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let mut a = a as i64;
    let m0 = m as i64;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut m = m0;

    while a > 1 {
        if m == 0 {
            return None; // Нет обратного элемента
        }
        let q = a / m;
        let t = m;
        m = a % m;
        a = t;
        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += m0;
    }

    Some(x1 as u64)
}

fn main() {
    let moduli = vec![3, 5, 7];
    let x = 10;
    let rns = to_rns(x, &moduli);
    println!("RNS представление: {:?}", rns);
    let reconstructed = from_rns(&rns, &moduli);
    println!("Восстановленное число: {}", reconstructed);
}
