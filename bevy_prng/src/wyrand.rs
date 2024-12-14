use crate::{newtype::newtype_prng, EntropySource};

use bevy_reflect::{Reflect, ReflectFromReflect};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "serialize")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    WyRand,
    ::wyrand::WyRand,
    "A newtyped [`wyrand::WyRand`] RNG",
    "wyrand"
);
