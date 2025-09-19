use crate::newtype::newtype_prng;

use rand_core::{CryptoRng, SeedableRng};

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    ChaCha8Rng,
    ::rand_chacha::ChaCha8Rng,
    "A [`rand_chacha::ChaCha8Rng`] RNG component",
    "rand_chacha"
);

impl CryptoRng for ChaCha8Rng {}

newtype_prng!(
    ChaCha12Rng,
    ::rand_chacha::ChaCha12Rng,
    "A [`rand_chacha::ChaCha12Rng`] RNG component",
    "rand_chacha"
);

impl CryptoRng for ChaCha12Rng {}

newtype_prng!(
    ChaCha20Rng,
    ::rand_chacha::ChaCha20Rng,
    "A [`rand_chacha::ChaCha20Rng`] RNG component",
    "rand_chacha"
);

impl CryptoRng for ChaCha20Rng {}
