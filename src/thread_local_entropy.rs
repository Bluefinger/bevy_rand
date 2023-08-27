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
/// [Too Much Crypto](https://eprint.iacr.org/2019/1492.pdf) paper. [`ThreadLocalEntropy`] is not thread-safe and
/// cannot be sent or synchronised between threads, it should be initialised within each thread context it is
/// needed in.
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

        let mut cloned = rng1.clone();

        rng1.fill_bytes(&mut bytes1);
        cloned.fill_bytes(&mut bytes2);

        // Cloned ThreadLocalEntropy instances won't output the same entropy
        assert_ne!(&bytes1, &bytes2);
    }

    #[test]
    fn unique_source_per_thread() {
        use std::sync::mpsc::channel;

        let mut bytes1: Vec<u8> = vec![0u8; 128];
        let mut bytes2: Vec<u8> = vec![0u8; 128];

        let b1 = bytes1.as_mut();
        let b2 = bytes2.as_mut();

        let (sender, receiver) = channel();
        let sender2 = sender.clone();

        std::thread::scope(|s| {
            s.spawn(move || {
                // Obtain a thread local entropy source from this thread context.
                // It should be initialised with a random state.
                let mut rng = ThreadLocalEntropy::new();

                // Use the source to produce some stored entropy.
                rng.fill_bytes(b1);

                // SAFETY: The pointer is valid and points to a ChaCha8Rng instance,
                // and it is not being accessed elsewhere nor being mutated. It is
                // safe to deference & cast the pointer so we can clone the RNG.
                let source = unsafe { &*rng.0.get() };

                sender.send(source.clone()).unwrap();
            });
            s.spawn(move || {
                // Obtain a thread local entropy source from this thread context.
                // It should be initialised with a random state.
                let mut rng = ThreadLocalEntropy::new();

                // Use the source to produce some stored entropy.
                rng.fill_bytes(b2);

                // SAFETY: The pointer is valid and points to a ChaCha8Rng instance,
                // and it is not being accessed elsewhere nor being mutated. It is
                // safe to deference & cast the pointer so we can clone the RNG.
                let source = unsafe { &*rng.0.get() };

                sender2.send(source.clone()).unwrap();
            });
        });

        // Wait for the threads to execute and resolve.
        let a = receiver.recv().unwrap();
        let b = receiver.recv().unwrap();

        // The references to the thread local RNG sources will not be
        // the same, as they each were initialised with different random
        // states to each other from OS sources, even though each went
        // through the exact same deterministic steps to fill some bytes.
        // If the tasks ran on the same thread, then the RNG sources should
        // be in different resulting states as the same source was advanced
        // further.
        assert_ne!(&a, &b);

        // Double check the entropy output in each buffer is not the same either.
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
