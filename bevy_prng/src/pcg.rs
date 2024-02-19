use crate::{newtype::newtype_prng, SeedableEntropySource};

use bevy::prelude::{Reflect, ReflectFromReflect};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "serialize")]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    Pcg32,
    ::rand_pcg::Pcg32,
    [u8; 16],
    "A newtyped [`rand_pcg::Pcg32`] RNG",
    "rand_pcg"
);

newtype_prng!(
    Pcg64,
    ::rand_pcg::Pcg64,
    [u8; 32],
    "A newtyped [`rand_pcg::Pcg64`] RNG",
    "rand_pcg"
);

newtype_prng!(
    Pcg64Mcg,
    ::rand_pcg::Pcg64Mcg,
    [u8; 16],
    "A newtyped [`rand_pcg::Pcg64Mcg`] RNG",
    "rand_pcg"
);
