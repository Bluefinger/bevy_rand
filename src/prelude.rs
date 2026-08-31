pub use crate::commands::{RngEntityCommands, RngEntityCommandsExt};
pub use crate::global::{GlobalRng, GlobalRngEntity};
pub use crate::observers::{RngLinks, RngSource, SeedFromGlobal, SeedFromSource, SeedLinked};
pub use crate::params::{RngEntity, RngEntityItem};
pub use crate::plugin::{EntropyPlugin, EntropyRelationsPlugin};
pub use crate::seed::RngSeed;
pub use crate::traits::{
    ForkRngExt, ForkSeedExt, ForkableAsRng, ForkableAsSeed, ForkableInnerSeed, ForkableRng,
    ForkableSeed, SeedSource,
};

#[cfg(feature = "fast_rng")]
#[cfg_attr(docsrs, doc(cfg(feature = "fast_rng")))]
pub use bevy_prng::FastRng;

#[cfg(feature = "fast_rng32")]
#[cfg_attr(docsrs, doc(cfg(feature = "fast_rng32")))]
pub use bevy_prng::FastRng32;

#[cfg(feature = "quality_rng")]
#[cfg_attr(docsrs, doc(cfg(feature = "quality_rng")))]
pub use bevy_prng::QualityRng;

#[cfg(feature = "thread_local_entropy")]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "thread_local_entropy", feature = "std")))
)]
pub use bevy_prng::ThreadLocalEntropy;
