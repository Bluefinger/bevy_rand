use core::marker::PhantomData;

use rand_core::{RngCore, SeedableRng};

use crate::thread_local_entropy::ThreadLocalEntropy;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum FastSeed {
    #[cfg(feature = "wyrand")]
    WyRand(
        <bevy_prng::WyRand as SeedableRng>::Seed,
        #[cfg_attr(feature = "bevy_reflect", reflect(ignore))] PhantomData<bevy_prng::WyRand>,
    ),
    #[cfg(feature = "rand_pcg")]
    Pcg64Mcg(
        <bevy_prng::Pcg64Mcg as SeedableRng>::Seed,
        #[cfg_attr(feature = "bevy_reflect", reflect(ignore))] PhantomData<bevy_prng::Pcg64Mcg>,
    ),
}

impl FastSeed {
    /// Creates a new FastSeed with entropy source
    ///
    /// ```
    /// use bevy_rand::prngs::{FastSeed};
    ///
    /// let _ = FastSeed::with_entropy(FastSeed::WyRand);
    /// ```
    #[inline]
    pub fn with_entropy<V: Fn(Prng::Seed, PhantomData<Prng>) -> FastSeed, Prng: SeedableRng>(
        v: V,
    ) -> Self {
        let mut seed: Prng::Seed = Default::default();

        ThreadLocalEntropy::new().unwrap().fill_bytes(seed.as_mut());

        v(seed, PhantomData)
    }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum FastRngBackend {
    #[cfg(feature = "wyrand")]
    WyRand(bevy_prng::WyRand),
    #[cfg(feature = "rand_pcg")]
    Pcg64Mcg(bevy_prng::Pcg64Mcg),
}

impl From<FastSeed> for FastRngBackend {
    #[inline]
    fn from(value: FastSeed) -> Self {
        match value {
            #[cfg(feature = "wyrand")]
            FastSeed::WyRand(seed, _) => Self::WyRand(bevy_prng::WyRand::from_seed(seed)),
            #[cfg(feature = "rand_pcg")]
            FastSeed::Pcg64Mcg(seed, _) => Self::Pcg64Mcg(bevy_prng::Pcg64Mcg::from_seed(seed)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fastseed_smoke_test() {
        let a = FastSeed::with_entropy(FastSeed::WyRand);
        let b = FastSeed::with_entropy(FastSeed::WyRand);
        let c = FastSeed::with_entropy(FastSeed::Pcg64Mcg);

        // Same algo backend, but randomised entropy states should mean they never
        // equal the same
        assert_ne!(&a, &b);
        // Different algo backends, so they will never be the same as each other
        assert_ne!(&a, &c);
    }
}
