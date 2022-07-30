mod utils;

use ff::Field;
use group::ff::PrimeField;
use pasta_curves::arithmetic::*;
use rayon::join;
use utils::{butterfly_arithmetic, swap_bit_reverse};

// classic fft structure
#[derive(Clone, Debug)]
pub struct ClassicFft<G: Group> {
    // polynomial degree 2^k
    k: u32,
    // generator which generates order 2^{k - 1} multiplicative group used as twiddle factors
    multiplicative_generator: G::Scalar,
}

impl<G: Group> ClassicFft<G> {
    pub fn new(k: u32) -> Self {
        let unity_of_root = G::Scalar::root_of_unity();
        let k_diff = G::Scalar::S - k;
        let multiplicative_generator = unity_of_root.pow_vartime(&[k_diff as u64, 0, 0, 0]);

        ClassicFft {
            k,
            multiplicative_generator,
        }
    }

    // perform classic fft
    pub fn fft(self, coeffs: &mut [G]) {
        let n = 1 << self.k;
        assert_eq!(coeffs.len(), n);

        swap_bit_reverse(coeffs, n, self.k);

        // precompute twiddle factors
        let twiddle_factors: Vec<_> = (0..(n / 2) as usize)
            .scan(G::Scalar::one(), |w, _| {
                let tw = *w;
                w.group_scale(&self.multiplicative_generator);
                Some(tw)
            })
            .collect();

        classic_fft(coeffs, n, 1, &twiddle_factors)
    }
}

// classic fft using divide and conquer algorithm
fn classic_fft<G: Group>(coeffs: &mut [G], n: usize, twiddle_chunk: usize, twiddles: &[G::Scalar]) {
    if n == 2 {
        let t = coeffs[1];
        coeffs[1] = coeffs[0];
        coeffs[0].group_add(&t);
        coeffs[1].group_sub(&t);
    } else {
        let (left, right) = coeffs.split_at_mut(n / 2);
        join(
            || classic_fft(left, n / 2, twiddle_chunk * 2, twiddles),
            || classic_fft(right, n / 2, twiddle_chunk * 2, twiddles),
        );
        butterfly_arithmetic(left, right, twiddle_chunk, twiddles)
    }
}
