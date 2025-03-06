use crate::newtype::newtype_prng;

use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "serialize")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    Pcg32,
    ::rand_pcg::Pcg32,
    "A newtyped [`rand_pcg::Pcg32`] RNG",
    "rand_pcg"
);

newtype_prng!(
    Pcg64,
    ::rand_pcg::Pcg64,
    "A newtyped [`rand_pcg::Pcg64`] RNG",
    "rand_pcg"
);

newtype_prng!(
    Pcg64Mcg,
    ::rand_pcg::Pcg64Mcg,
    "A newtyped [`rand_pcg::Pcg64Mcg`] RNG",
    "rand_pcg"
);
