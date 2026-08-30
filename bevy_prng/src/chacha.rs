use rand_core::SeedableRng;

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;
use crate::newtype::chacha_prng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

chacha_prng! {
    #[feature = "chacha20"]

    /// A [`chacha20::ChaCha8Rng`] RNG component
    struct ChaCha8Rng(chacha20::ChaCha8Rng);

    /// A [`chacha20::ChaCha12Rng`] RNG component
    struct ChaCha12Rng(chacha20::ChaCha12Rng);

    /// A [`chacha20::ChaCha20Rng`] RNG component
    struct ChaCha20Rng(chacha20::ChaCha20Rng);
}
