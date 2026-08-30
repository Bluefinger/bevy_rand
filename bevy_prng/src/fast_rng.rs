#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;
use crate::newtype::newtype_prng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng! {
    #[feature = "fast_rng"]

    /// A Fast RNG component. This is the best choice for most RNG needs, as it is
    /// incredibly fast (on 64-bit platforms) and produces very good statistically random
    /// output. Do note, this is not cryptographically secure, and the state of the
    /// RNG can be determined/recovered with enough outputs. If this is a concern for
    /// gameplay reasons, for example you don't want players to be able to predict things
    /// like drop rates and such, opt for using [crate::QualityRng] instead, or use more
    /// advanced techniques to reseed this RNG often from non-deterministic sources.
    struct FastRng(wyrand::WyRand);
}
