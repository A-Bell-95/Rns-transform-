use super::{Rns, PRIME_BASE_U64};

impl<const N: usize> Rns<N> {
    pub fn new(val: u64) -> Self {
        let mut remainders = [0; N];
        for (i, p) in PRIME_BASE_U64.iter().take(N).enumerate() {
            remainders[i] = (val % p) as u8;
        }
        Rns { remainders }
    }

    pub fn try_into_u64(&self) -> Option<u64> {
        let mut result = 0u128;
        let mut m_acc = 1u128; // Накопленный модуль

        for (&r_i, &m_i) in self.remainders.iter().zip(PRIME_BASE_U64.iter()) {
            let m_i_ = m_acc; // Текущее произведение предыдущих модулей

            let m_i_inv = mod_inverse(m_i_ as i64, m_i as i64)? as u128;

            // Безопасное вычисление разницы (гарантированно неотрицательное значение)
            let delta = (r_i as u128 + m_i as u128 - (result % m_i as u128)) % m_i as u128;

            // Обновление результата по китайской теореме
            result = (result + delta * m_i_ * m_i_inv) % (m_acc * m_i as u128);
            m_acc *= m_i as u128; // Обновляем полный модуль
        }

        Some(result as u64)
    }

    pub fn into_u64(&self) -> u64 {
        self.try_into_u64().unwrap()
    }
}

impl<const N: usize> TryInto<u64> for &Rns<N> {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        self.try_into_u64().ok_or(())
    }
}

impl<const N: usize> TryInto<u64> for Rns<N> {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        self.try_into_u64().ok_or(())
    }
}

/// Функция нахождения обратного элемента (mod_inverse)
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        return None;
    } // Если НОД не 1, обратного элемента нет
    Some((x % m + m) % m)
}

/// Расширенный алгоритм Евклида
fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut old_s, mut s, mut old_t, mut t) = (1, 0, 0, 1);

    while a != 0 {
        let q = b / a;
        (b, a) = (a, b % a);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (b, old_t, old_s)
}
