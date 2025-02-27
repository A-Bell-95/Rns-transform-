const PRIME_BASE_U64: [u64; 18] = [3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67];

pub fn to_rns(x: u64) -> Vec<u64> {
    PRIME_BASE_U64.iter().map(|&m| x % m).collect()
}

/// Преобразование из RNS обратно в число
pub fn from_rns(remainders: &[u64]) -> u64 {
    let mut result = 0u128;
    let mut m_acc = 1u128; // Накопленный модуль

    for (&r_i, &m_i) in remainders.iter().zip(PRIME_BASE_U64.iter()) {
        let m_i_ = m_acc; // Текущее произведение предыдущих модулей
        // println!("M_i = {}, m_i = {}, extended_gcd = {:?}", m_i_, m_i, extended_gcd(m_i_ as i64, m_i as i64));

        let m_i_inv = mod_inverse(m_i_ as i64, m_i as i64)
            .expect(&format!("Обратный элемент не существует для {} mod {}", m_i_, m_i)) as u128;

        // Безопасное вычисление разницы (гарантированно неотрицательное значение)
        let delta = (r_i as u128 + m_i as u128 - (result % m_i as u128)) % m_i as u128;

        // Обновление результата по китайской теореме
        result = (result + delta * m_i_ * m_i_inv) % (m_acc * m_i as u128);
        m_acc *= m_i as u128; // Обновляем полный модуль
    }

    result as u64
}

/// Функция нахождения обратного элемента (mod_inverse)
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 { return None; } // Если НОД не 1, обратного элемента нет
    Some((x % m + m) % m)
}

/// Расширенный алгоритм Евклида
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 { return (b, 0, 1); }
    let (g, x1, y1) = extended_gcd(b % a, a);
    (g, y1 - (b / a) * x1, x1)
}
