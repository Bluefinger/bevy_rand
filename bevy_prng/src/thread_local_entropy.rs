use alloc::rc::Rc;
use core::{cell::UnsafeCell, convert::Infallible};
use std::thread_local;

use chacha20::ChaCha8Rng;
use rand_core::{SeedableRng, TryCryptoRng, TryRng};

thread_local! {
    // We require `Rc` to avoid premature freeing when `ThreadLocalEntropy` is used within thread-local destructors.
    static SOURCE: Rc<UnsafeCell<ChaCha8Rng>> = {
        Rc::new(UnsafeCell::new(ChaCha8Rng::try_from_rng(&mut getrandom::SysRng).expect("Unable to source entropy for initialisation")))
    };
}

/// [`ThreadLocalEntropy`] uses thread local [`ChaCha8Rng`] instances to provide faster alternative for
/// sourcing entropy to OS/Hardware sources. The use of `ChaCha8` with 8 rounds as opposed to 12 or 20 rounds
/// is due to tuning for additional speed/throughput. While this does minimise the quality of the entropy,
/// the output should still be sufficiently secure as per the recommendations set in the
/// [Too Much Crypto](https://eprint.iacr.org/2019/1492.pdf) paper. [`ThreadLocalEntropy`] is not thread-safe and
/// cannot be sent or synchronised between threads, it should be initialised within each thread context it is
/// needed in.
pub struct ThreadLocalEntropy(Rc<UnsafeCell<ChaCha8Rng>>);

impl ThreadLocalEntropy {
    /// Obtain a new [`ThreadLocalEntropy`] instance.
    pub fn get() -> Result<Self, std::thread::AccessError> {
        Ok(Self(SOURCE.try_with(Rc::clone)?))
    }

    /// Initiates an access to the thread local source, passing a `&mut ChaCha8Rng` to the
    /// closure.
    #[inline(always)]
    fn access_local_source<F, O>(&mut self, f: F) -> Result<O, Infallible>
    where
        F: FnOnce(&mut ChaCha8Rng) -> Result<O, Infallible>,
    {
        // SAFETY: The `&mut` reference constructed here will never outlive the closure
        // for the thread local access. It is also will never be a null pointer and is aligned.
        unsafe { f(&mut *self.0.get()) }
    }
}

impl core::fmt::Debug for ThreadLocalEntropy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ThreadLocalEntropy").finish()
    }
}

impl TryRng for ThreadLocalEntropy {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        self.access_local_source(TryRng::try_next_u32)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        self.access_local_source(TryRng::try_next_u64)
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.access_local_source(|source| source.try_fill_bytes(dst))
    }
}

impl TryCryptoRng for ThreadLocalEntropy {}

#[cfg(test)]
mod tests {
    use alloc::{format, vec, vec::Vec};

    use rand_core::Rng;

    use super::*;

    #[test]
    fn smoke_test() -> Result<(), std::thread::AccessError> {
        let mut rng1 = ThreadLocalEntropy::get()?;
        let mut rng2 = ThreadLocalEntropy::get()?;

        // Neither instance should interfere with each other
        rng1.next_u32();
        rng2.next_u64();

        let mut bytes1 = vec![0u8; 128];
        let mut bytes2 = vec![0u8; 128];

        rng1.fill_bytes(&mut bytes1);
        rng2.fill_bytes(&mut bytes2);

        // ThreadLocalEntropy instances won't output the same entropy as the
        // underlying thread local source gets mutated each access.
        assert_ne!(&bytes1, &bytes2);

        Ok(())
    }

    #[test]
    fn unique_source_per_thread() {
        let mut bytes1: Vec<u8> = vec![0u8; 128];
        let mut bytes2: Vec<u8> = vec![0u8; 128];

        let b1 = bytes1.as_mut();
        let b2 = bytes2.as_mut();

        let (a, b) = std::thread::scope(|s| {
            let a = s.spawn(move || {
                // Obtain a thread local entropy source from this thread context.
                // It should be initialised with a random state.
                let mut rng =
                    ThreadLocalEntropy::get().expect("Should not fail when accessing local source");

                // Use the source to produce some stored entropy.
                rng.fill_bytes(b1);

                rng.access_local_source(|rng| Ok(rng.next_u64()))
            });
            let b = s.spawn(move || {
                // Obtain a thread local entropy source from this thread context.
                // It should be initialised with a random state.
                let mut rng =
                    ThreadLocalEntropy::get().expect("Should not fail when accessing local source");

                // Use the source to produce some stored entropy.
                rng.fill_bytes(b2);

                rng.access_local_source(|rng| Ok(rng.next_u64()))
            });

            (a.join(), b.join())
        });

        let Ok(a) = a.unwrap();
        let Ok(b) = b.unwrap();

        // The references to the thread local RNG sources will not be
        // the same, as they each were initialised with different random
        // states to each other from OS sources, even though each went
        // through the exact same deterministic steps to fill some bytes.
        // If the tasks ran on the same thread, then the RNG sources should
        // be in different resulting states as the same source was advanced
        // further.
        assert_ne!(a, b);

        // Double check the entropy output in each buffer is not the same either.
        assert_ne!(&bytes1, &bytes2);
    }

    #[test]
    fn non_leaking_debug() {
        assert_eq!(
            "Ok(ThreadLocalEntropy)",
            format!("{:?}", ThreadLocalEntropy::get())
        );
    }
}
