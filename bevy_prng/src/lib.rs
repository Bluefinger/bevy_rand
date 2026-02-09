#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "chacha20")]
mod chacha;
#[cfg(any(
    feature = "wyrand",
    feature = "chacha20",
    feature = "rand_pcg",
    feature = "rand_xoshiro"
))]
mod newtype;
#[cfg(feature = "rand_pcg")]
mod pcg;
#[cfg(feature = "wyrand")]
mod wyrand;
#[cfg(feature = "rand_xoshiro")]
mod xoshiro;
mod utils;

#[cfg(feature = "thread_local_entropy")]
mod thread_local_entropy;

use core::fmt::Debug;

use bevy_ecs::component::{Component, Mutable};
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{FromReflect, Reflectable, Typed};
use rand_core::{Rng, SeedableRng};

#[cfg(feature = "chacha20")]
pub use chacha::*;
#[cfg(feature = "rand_pcg")]
pub use pcg::*;
#[cfg(feature = "thread_local_entropy")]
pub use thread_local_entropy::ThreadLocalEntropy;
#[cfg(feature = "wyrand")]
pub use wyrand::WyRand;
#[cfg(feature = "rand_xoshiro")]
pub use xoshiro::*;

/// Trait for handling `SeedableRng` requirements, imposing constraints
/// depending on whether reflection support is enabled or not
#[cfg(feature = "bevy_reflect")]
pub trait TypedSeed: SeedableRng<Seed: Typed + Debug + Send + Sync + Clone> {}

#[cfg(feature = "bevy_reflect")]
impl<T: SeedableRng<Seed: Typed + Debug + Send + Sync + Clone>> TypedSeed for T {}

/// Trait for handling `SeedableRng` requirements, imposing constraints
/// depending on whether reflection support is enabled or not
#[cfg(not(feature = "bevy_reflect"))]
pub trait TypedSeed: SeedableRng<Seed: Debug + Send + Sync + Clone> {}

#[cfg(not(feature = "bevy_reflect"))]
impl<T: SeedableRng<Seed: Debug + Send + Sync + Clone>> TypedSeed for T {}

/// Trait for handling contraints for valid implementations of [`EntropySource`]
/// depending on whether reflection support is enabled or not
#[cfg(feature = "bevy_reflect")]
pub trait RngReflectable: FromReflect + Reflectable {}

#[cfg(feature = "bevy_reflect")]
impl<T: FromReflect + Reflectable> RngReflectable for T {}

/// Trait for handling contraints for valid implementations of [`EntropySource`]
/// depending on whether reflection support is enabled or not
#[cfg(not(feature = "bevy_reflect"))]
pub trait RngReflectable: 'static {}

#[cfg(not(feature = "bevy_reflect"))]
impl<T: 'static> RngReflectable for T {}

/// A marker trait to define the required trait bounds for a seedable PRNG to be
/// integrated as a component. This is a sealed trait.
pub trait EntropySource:
    Rng
    + RngReflectable
    + TypedSeed
    + Clone
    + Debug
    + PartialEq
    + Component<Mutability = Mutable>
    + Sync
    + Send
    + RemoteRng
    + private::SealedSeedable
{
}

/// Marker trait for a suitable seed for [`EntropySource`]. This is an auto trait which will
/// apply to all suitable types that meet the trait criteria.
pub trait EntropySeed:
    Debug + Default + PartialEq + AsMut<[u8]> + Clone + Sync + Send + RngReflectable
{
}

impl<T: Debug + Default + PartialEq + AsMut<[u8]> + Clone + Sync + Send + RngReflectable>
    EntropySeed for T
{
}

/// Reflectable Rng. This trait ensures that if `bevy_reflect` is active,
/// that all [`EntropySource`] PRNGs can be used through reflection and interface
/// with [`Rng`].
#[cfg_attr(feature = "bevy_reflect", bevy_reflect::reflect_trait)]
pub trait RemoteRng: Rng {}

mod private {
    pub trait SealedSeedable {}

    impl<T: super::EntropySource> SealedSeedable for T {}
}
