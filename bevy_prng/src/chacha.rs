use crate::{newtype::newtype_prng, SeedableEntropySource};

use bevy::prelude::{Reflect, ReflectFromReflect};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "serialize")]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    ChaCha8Rng,
    ::rand_chacha::ChaCha8Rng,
    [u8; 32],
    "A newtyped [`rand_chacha::ChaCha8Rng`] RNG",
    "rand_chacha"
);

newtype_prng!(
    ChaCha12Rng,
    ::rand_chacha::ChaCha12Rng,
    [u8; 32],
    "A newtyped [`rand_chacha::ChaCha12Rng`] RNG",
    "rand_chacha"
);

newtype_prng!(
    ChaCha20Rng,
    ::rand_chacha::ChaCha20Rng,
    [u8; 32],
    "A newtyped [`rand_chacha::ChaCha20Rng`] RNG",
    "rand_chacha"
);
