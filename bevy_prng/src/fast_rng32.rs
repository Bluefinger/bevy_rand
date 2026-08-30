use crate::newtype::newtype_prng;

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

newtype_prng! {
    #[feature = "fast_rng32"]

    /// A Fast RNG component specialised for 32-bit platforms. This is the best choice
    /// for when you have to support weaker or more specialised hardware/platforms, whose
    /// performance characteristics favour 32-bit instructions over 64-bit. The RNG has weaker
    /// statistically random output then [crate::FastRng] and is slower on 64-bit platforms.
    /// Do note, this is not cryptographically secure, and the state of the RNG can be
    /// determined/recovered with enough outputs. If this is a concern for gameplay reasons,
    /// for example you don't want players to be able to predict things like drops rates and such,
    /// opt for using [crate::QualityRng] instead, or use more advanced techniques to reseed this
    /// RNG often from non-deterministic sources.
    struct FastRng32(rand_pcg::Pcg32);
}
