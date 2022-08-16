mod behave;
mod curve;
mod isogeny;
mod utils;

use utils::{swap_bit_reverse, EcFftCache};

use pairing::bn256::Fq as Fp;

// precomputed params for ecfft
#[derive(Clone, Debug)]
pub(crate) struct EcFft {
    // polynomial degree 2^k
    k: u32,
    // precomputed ecfft params
    cache: EcFftCache,
}

impl EcFft {
    pub fn new(k: u32) -> Self {
        assert!(k == 14);
        let cache = EcFftCache::new(k);

        EcFft { k, cache }
    }

    // perform ecfft
    pub fn fft(&self, coeffs: &mut [Fp]) {
        let n = 1 << self.k;
        assert_eq!(coeffs.len(), n);

        swap_bit_reverse(coeffs, n, self.k);

        ecfft_arithmetic(coeffs, n)
    }
}

// ecfft using divide and conquer algorithm
fn ecfft_arithmetic(coeffs: &mut [Fp], n: usize) {
    if n == 1 {
    } else {
    }
}
