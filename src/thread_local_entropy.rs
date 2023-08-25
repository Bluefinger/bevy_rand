use std::{cell::UnsafeCell, rc::Rc};

use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

thread_local! {
    // We require `Rc` to avoid premature freeing when `ThreadLocalEntropy` is used within thread-local destructors.
    static SOURCE: Rc<UnsafeCell<ChaCha8Rng>> = Rc::new(UnsafeCell::new(ChaCha8Rng::from_entropy()));
}

/// [`ThreadLocalEntropy`] uses thread local [`ChaCha8Rng`] instances to provide faster alternative for
/// sourcing entropy to OS/Hardware sources. The use of `ChaCha8` with 8 rounds as opposed to 12 or 20 rounds
/// is due to tuning for additional speed/throughput. While this does minimise the quality of the entropy,
/// the output should still be sufficiently secure as per the recommendations set in the
/// [Too Much Crypto](https://eprint.iacr.org/2019/1492.pdf) paper.
pub(crate) struct ThreadLocalEntropy;

impl ThreadLocalEntropy {
    /// Inspired by `rand`'s approach to `ThreadRng` as well as `turborand`'s instantiation methods. The [`Rc`]
    /// prevents the Rng instance from being cleaned up, giving it a `'static` lifetime. However, it does not
    /// allow mutable access without a cell, so using [`UnsafeCell`] to bypass overheads associated with
    /// `RefCell`. There's no direct access to the pointer or mutable reference, so we control how long it
    /// lives and can ensure no multiple mutable references exist.
    ///
    /// # Safety
    ///
    /// The borrow checker ensures that only one mut reference can exist/be used at any given time. This is also
    /// an internal method which prevents usage outside of its intended area, so no `&mut` references to the
    /// underlying [`ChaCha8Rng`] instance is ever exposed.
    #[inline]
    fn get_rng(&'_ mut self) -> &'_ mut ChaCha8Rng {
        // Obtain pointer to thread local instance of PRNG which with Rc, should be !Send & !Sync as well
        // as 'static.
        let rng = SOURCE.with(|source| source.get());

        // SAFETY: The `&mut` reference is checked by the borrower checker due to `get_rng`
        // method being `&mut self` and dependent on the lifetime of the instance. This means
        // there can only ever be a single `&mut` reference in use at any given time.
        unsafe { &mut *rng }
    }
}

impl RngCore for ThreadLocalEntropy {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.get_rng().next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.get_rng().next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.get_rng().fill_bytes(dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.get_rng().try_fill_bytes(dest)
    }
}
