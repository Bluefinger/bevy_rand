use std::{cell::UnsafeCell, rc::Rc};

use rand_chacha::ChaCha8Rng;
use rand_core::{CryptoRng, RngCore, SeedableRng};

thread_local! {
    // We require `Rc` to avoid premature freeing when `ThreadLocalEntropy` is used within thread-local destructors.
    static SOURCE: Rc<UnsafeCell<ChaCha8Rng>> = Rc::new(UnsafeCell::new(ChaCha8Rng::from_entropy()));
}

/// [`ThreadLocalEntropy`] uses thread local [`ChaCha8Rng`] instances to provide faster alternative for
/// sourcing entropy to OS/Hardware sources. The use of `ChaCha8` with 8 rounds as opposed to 12 or 20 rounds
/// is due to tuning for additional speed/throughput. While this does minimise the quality of the entropy,
/// the output should still be sufficiently secure as per the recommendations set in the
/// [Too Much Crypto](https://eprint.iacr.org/2019/1492.pdf) paper.
#[derive(Clone)]
pub(crate) struct ThreadLocalEntropy(Rc<UnsafeCell<ChaCha8Rng>>);

impl ThreadLocalEntropy {
    /// Create a new [`ThreadLocalEntropy`] instance. Only one instance can exist per thread at a time, so to
    /// prevent the creation of multiple `&mut` references to
    #[inline]
    #[must_use]
    pub(crate) fn new() -> Self {
        Self(SOURCE.with(|source| Rc::clone(source)))
    }
}

impl core::fmt::Debug for ThreadLocalEntropy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ThreadLocalEntropy").finish()
    }
}

impl RngCore for ThreadLocalEntropy {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        // SAFETY: We will drop this `&mut` reference before we can
        // create another one, and only one reference can exist to the
        // underlying thread local cell at any given time.
        let rng = unsafe { &mut *self.0.get() };

        rng.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        // SAFETY: We will drop this `&mut` reference before we can
        // create another one, and only one reference can exist to the
        // underlying thread local cell at any given time.
        let rng = unsafe { &mut *self.0.get() };

        rng.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // SAFETY: We will drop this `&mut` reference before we can
        // create another one, and only one reference can exist to the
        // underlying thread local cell at any given time.
        let rng = unsafe { &mut *self.0.get() };

        rng.fill_bytes(dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        // SAFETY: We will drop this `&mut` reference before we can
        // create another one, and only one reference can exist to the
        // underlying thread local cell at any given time.
        let rng = unsafe { &mut *self.0.get() };

        rng.try_fill_bytes(dest)
    }
}

impl CryptoRng for ThreadLocalEntropy {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let mut rng1 = ThreadLocalEntropy::new();
        let mut rng2 = ThreadLocalEntropy::new();

        // Neither instance should interfere with each other
        rng1.next_u32();
        rng2.next_u64();

        let mut bytes1 = vec![0u8; 128];
        let mut bytes2 = vec![0u8; 128];

        rng1.fill_bytes(&mut bytes1);
        rng1.clone().fill_bytes(&mut bytes2);

        // Cloned ThreadLocalEntropy instances won't output the same entropy
        assert_ne!(&bytes1, &bytes2);
    }

    #[test]
    fn non_leaking_debug() {
        assert_eq!(
            "ThreadLocalEntropy",
            format!("{:?}", ThreadLocalEntropy::new())
        );
    }
}
