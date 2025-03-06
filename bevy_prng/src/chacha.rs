use crate::newtype::newtype_prng;

use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "serialize")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    ChaCha8Rng,
    ::rand_chacha::ChaCha8Rng,
    "A newtyped [`rand_chacha::ChaCha8Rng`] RNG",
    "rand_chacha"
);

newtype_prng!(
    ChaCha12Rng,
    ::rand_chacha::ChaCha12Rng,
    "A newtyped [`rand_chacha::ChaCha12Rng`] RNG",
    "rand_chacha"
);

newtype_prng!(
    ChaCha20Rng,
    ::rand_chacha::ChaCha20Rng,
    "A newtyped [`rand_chacha::ChaCha20Rng`] RNG",
    "rand_chacha"
);
