use std::{cell::UnsafeCell, marker::PhantomData, ptr::NonNull, rc::Rc};

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
pub(crate) struct ThreadLocalEntropy(PhantomData<*mut ()>);

impl ThreadLocalEntropy {
    /// Create a new [`ThreadLocalEntropy`] instance.
    #[inline]
    #[must_use]
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }

    #[inline]
    fn access_local_source<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(&mut ChaCha8Rng) -> O,
    {
        SOURCE.with(|source| {
            // SAFETY: Constructing `NonNull` from a `&T` is safe as it will never be a
            // null pointer, and the contents of the reference will always be initialised.
            let mut ptr = unsafe { NonNull::new_unchecked(source.get()) };

            // SAFETY: The `&mut` reference constructed from `NonNull` will never outlive the closure
            // for the thread local access.
            unsafe { f(ptr.as_mut()) }
        })
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
        self.access_local_source(RngCore::next_u32)
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.access_local_source(RngCore::next_u64)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.access_local_source(|rng| rng.fill_bytes(dest));
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.access_local_source(|rng| rng.try_fill_bytes(dest))
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

                let source = rng.access_local_source(|rng| rng.clone());

                sender.send(source).unwrap();
            });
            s.spawn(move || {
                // Obtain a thread local entropy source from this thread context.
                // It should be initialised with a random state.
                let mut rng = ThreadLocalEntropy::new();

                // Use the source to produce some stored entropy.
                rng.fill_bytes(b2);

                let source = rng.access_local_source(|rng| rng.clone());

                sender2.send(source).unwrap();
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
