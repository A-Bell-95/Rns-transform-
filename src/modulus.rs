use rand::{rngs::OsRng, TryRngCore};

type RandError = <OsRng as TryRngCore>::Error;

/// Генерация 40-битного простого модуля q, удовлетворяющего условиям NTT
pub fn find_suitable_modulus(n: u64) -> Result<u64, RandError> {
    // Мы можем рассчитывать на безопасность OsRng, так как он реализует трейт TryCryptoRng.
    let mut rng = OsRng;

    loop {
        let candidate = rng.try_next_u64()? % (1 << 40);
        let candidate = candidate | (1 << 39) | 1; // 40-битное нечётное число
        if (candidate - 1) % (2 * n) == 0 && is_prime(candidate, 10)? {
            return Ok(candidate);
        }
    }
}

/// Функция для проверки числа на простоту (Миллер-Рабин)
pub(crate) fn is_prime(n: u64, k: u32) -> Result<bool, RandError> {
    if n < 2 {
        return Ok(false);
    }
    if n % 2 == 0 {
        return Ok(n == 2);
    }

    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    let mut rng = OsRng;
    for _ in 0..k {
        let a = rng.try_next_u64()?;
        let a = (a % (n - 2)) + 2;

        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 0..r - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Быстрое возведение в степень по модулю
pub(crate) fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut base = base as u128;
    let mut exp = exp as u128;
    let modulus = modulus as u128;

    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }

    result as u64
}
