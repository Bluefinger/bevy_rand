use crate::newtype::newtype_prng;

use rand_core::SeedableRng;

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    Pcg32,
    ::rand_pcg::Pcg32,
    "A [`rand_pcg::Pcg32`] RNG component",
    "rand_pcg"
);

newtype_prng!(
    Pcg64,
    ::rand_pcg::Pcg64,
    "A [`rand_pcg::Pcg64`] RNG component",
    "rand_pcg"
);

newtype_prng!(
    Pcg64Mcg,
    ::rand_pcg::Pcg64Mcg,
    "A [`rand_pcg::Pcg64Mcg`] RNG component",
    "rand_pcg"
);
