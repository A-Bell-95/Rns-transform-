use num_complex::Complex;
use std::f64::consts::PI;

/// Reverse elements in bit count
fn bit_reverse_indices(n: usize) -> Vec<usize> {
    let mut result = vec![0; n];
    let bits = n.trailing_zeros();

    for i in 0..n {
        result[i] = i.reverse_bits() >> (usize::BITS - bits);
    }

    result
}

/// Interactive in-place FFT with pre-count twiddle factors
pub fn fft_in_place(buffer: &mut [Complex<f64>]) {
    let n = buffer.len();
    assert!(n.is_power_of_two(), "Length must be a power of two");

    // 1. Bit-reversal permutation
    let br = bit_reverse_indices(n);
    for i in 0..n {
        if i < br[i] {
            buffer.swap(i, br[i]);
        }
    }

    // 2. Precount twiddles
    let mut step = 2;
    while step <= n {
        let half_step = step / 2;
        let theta = -2.0 * PI / step as f64;

        for i in (0..n).step_by(step) {
            for j in 0..half_step {
                let w = Complex::from_polar(1.0, theta * j as f64);
                let u = buffer[i + j];
                let t = w * buffer[i + j + half_step];
                buffer[i + j] = u + t;
                buffer[i + j + half_step] = u - t;
            }
        }

        step *= 2;
    }
}
