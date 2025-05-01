use crate::newtype::newtype_prng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    WyRand,
    ::wyrand::WyRand,
    "A newtyped [`wyrand::WyRand`] RNG",
    "wyrand"
);
