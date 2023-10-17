#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

use std::fmt::Debug;

use bevy::{
    prelude::{FromReflect, Reflect},
    reflect::{GetTypeRegistration, TypePath},
};
use rand_core::{RngCore, SeedableRng};

#[cfg(any(
    feature = "wyrand",
    feature = "rand_chacha",
    feature = "rand_pcg",
    feature = "rand_xoshiro"
))]
use bevy::prelude::ReflectFromReflect;

#[cfg(all(
    any(
        feature = "wyrand",
        feature = "rand_chacha",
        feature = "rand_pcg",
        feature = "rand_xoshiro"
    ),
    feature = "serialize"
))]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

#[cfg(all(
    any(
        feature = "wyrand",
        feature = "rand_chacha",
        feature = "rand_pcg",
        feature = "rand_xoshiro"
    ),
    feature = "serialize"
))]
use serde::{Deserialize, Serialize};

/// A marker trait to define the required trait bounds for a seedable PRNG to
/// integrate into `EntropyComponent` or `GlobalEntropy`. This is a sealed trait.
#[cfg(feature = "serialize")]
pub trait SeedableEntropySource:
    RngCore
    + SeedableRng
    + Clone
    + Debug
    + PartialEq
    + Sync
    + Send
    + Reflect
    + TypePath
    + FromReflect
    + GetTypeRegistration
    + Serialize
    + for<'a> Deserialize<'a>
    + private::SealedSeedable
{
}

/// A marker trait to define the required trait bounds for a seedable PRNG to
/// integrate into `EntropyComponent` or `GlobalEntropy`. This is a sealed trait.
#[cfg(not(feature = "serialize"))]
pub trait SeedableEntropySource:
    RngCore
    + SeedableRng
    + Clone
    + Debug
    + PartialEq
    + Reflect
    + TypePath
    + FromReflect
    + GetTypeRegistration
    + Sync
    + Send
    + private::SealedSeedable
{
}

mod private {
    pub trait SealedSeedable {}

    impl<T: super::SeedableEntropySource> SealedSeedable for T {}
}

#[cfg(any(
    feature = "wyrand",
    feature = "rand_chacha",
    feature = "rand_pcg",
    feature = "rand_xoshiro"
))]
macro_rules! newtype_prng {
    ($newtype:tt, $rng:ty, $seed:ty, $doc:tt, $feature:tt) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Reflect)]
        #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
        #[cfg_attr(
            all(feature = "serialize"),
            reflect_value(Debug, PartialEq, FromReflect, Serialize, Deserialize)
        )]
        #[cfg_attr(
            all(not(feature = "serialize")),
            reflect_value(Debug, PartialEq, FromReflect)
        )]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        #[repr(transparent)]
        pub struct $newtype($rng);

        impl $newtype {
            /// Create a new instance.
            #[inline]
            #[must_use]
            pub fn new(rng: $rng) -> Self {
                Self(rng)
            }
        }

        impl RngCore for $newtype {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                self.0.next_u32()
            }

            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                self.0.next_u64()
            }

            #[inline]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                self.0.fill_bytes(dest)
            }

            #[inline]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
                self.0.try_fill_bytes(dest)
            }
        }

        impl SeedableRng for $newtype {
            type Seed = $seed;

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self::new(<$rng>::from_seed(seed))
            }
        }

        impl From<$rng> for $newtype {
            #[inline]
            fn from(value: $rng) -> Self {
                Self::new(value)
            }
        }

        impl SeedableEntropySource for $newtype {}
    };
}

#[cfg(feature = "wyrand")]
newtype_prng!(
    WyRand,
    ::wyrand::WyRand,
    [u8; 8],
    "A newtyped [`wyrand::WyRand`] RNG",
    "wyrand"
);

#[cfg(feature = "rand_chacha")]
newtype_prng!(
    ChaCha8Rng,
    ::rand_chacha::ChaCha8Rng,
    [u8; 32],
    "A newtyped [`rand_chacha::ChaCha8Rng`] RNG",
    "rand_chacha"
);

#[cfg(feature = "rand_chacha")]
newtype_prng!(
    ChaCha12Rng,
    ::rand_chacha::ChaCha12Rng,
    [u8; 32],
    "A newtyped [`rand_chacha::ChaCha12Rng`] RNG",
    "rand_chacha"
);

#[cfg(feature = "rand_chacha")]
newtype_prng!(
    ChaCha20Rng,
    ::rand_chacha::ChaCha20Rng,
    [u8; 32],
    "A newtyped [`rand_chacha::ChaCha20Rng`] RNG",
    "rand_chacha"
);

#[cfg(feature = "rand_pcg")]
newtype_prng!(
    Pcg32,
    ::rand_pcg::Pcg32,
    [u8; 16],
    "A newtyped [`rand_pcg::Pcg32`] RNG",
    "rand_pcg"
);

#[cfg(feature = "rand_pcg")]
newtype_prng!(
    Pcg64,
    ::rand_pcg::Pcg64,
    [u8; 32],
    "A newtyped [`rand_pcg::Pcg64`] RNG",
    "rand_pcg"
);

#[cfg(feature = "rand_pcg")]
newtype_prng!(
    Pcg64Mcg,
    ::rand_pcg::Pcg64Mcg,
    [u8; 16],
    "A newtyped [`rand_pcg::Pcg64Mcg`] RNG",
    "rand_pcg"
);

#[cfg(feature = "rand_xoshiro")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_xoshiro")))]
pub use rand_xoshiro::Seed512;

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro512StarStar,
    ::rand_xoshiro::Xoshiro512StarStar,
    ::rand_xoshiro::Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512StarStar`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro512PlusPlus,
    ::rand_xoshiro::Xoshiro512PlusPlus,
    ::rand_xoshiro::Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512PlusPlus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro512Plus,
    ::rand_xoshiro::Xoshiro512Plus,
    ::rand_xoshiro::Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512Plus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro256StarStar,
    ::rand_xoshiro::Xoshiro256StarStar,
    [u8; 32],
    "A newtyped [`rand_xoshiro::Xoshiro256StarStar`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro256PlusPlus,
    ::rand_xoshiro::Xoshiro256PlusPlus,
    [u8; 32],
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro256Plus,
    ::rand_xoshiro::Xoshiro256Plus,
    [u8; 32],
    "A newtyped [`rand_xoshiro::Xoshiro256Plus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoroshiro128StarStar,
    ::rand_xoshiro::Xoroshiro128StarStar,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128StarStar`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoroshiro128PlusPlus,
    ::rand_xoshiro::Xoroshiro128PlusPlus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoroshiro128Plus,
    ::rand_xoshiro::Xoroshiro128Plus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128Plus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro128StarStar,
    ::rand_xoshiro::Xoshiro128StarStar,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128StarStar`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro128PlusPlus,
    ::rand_xoshiro::Xoshiro128PlusPlus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoshiro128Plus,
    ::rand_xoshiro::Xoshiro128Plus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128Plus`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoroshiro64StarStar,
    ::rand_xoshiro::Xoroshiro64StarStar,
    [u8; 8],
    "A newtyped [`rand_xoshiro::Xoroshiro64StarStar`] RNG",
    "rand_xoshiro"
);

#[cfg(feature = "rand_xoshiro")]
newtype_prng!(
    Xoroshiro64Star,
    ::rand_xoshiro::Xoroshiro64Star,
    [u8; 8],
    "A newtyped [`rand_xoshiro::Xoroshiro64Star`] RNG",
    "rand_xoshiro"
);
