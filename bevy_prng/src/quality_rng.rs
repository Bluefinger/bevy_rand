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
    #[feature = "quality_rng"]

    /// A Quality RNG component. This is the best choice for when you need to
    /// ensure that the RNG state can't be recovered/predicted from its generated
    /// outputs, producing statistically very high quality randomness. The RNG is
    /// tuned to be as fast as possible, but it will still be considerably slower
    /// than [crate::FastRng]. It is recommended to reseed this RNG often in order
    /// to ensure its outputs are as unpredictable as possible, particularly when
    /// used to seed child RNG components. **DO NOT** use this for actual
    /// cryptographic/security purposes.
    struct QualityRng(chacha20::ChaCha8Rng);
}
