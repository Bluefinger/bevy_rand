#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![no_std]

#[cfg(feature = "rand_chacha")]
mod chacha;
#[cfg(any(
    feature = "wyrand",
    feature = "rand_chacha",
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

use core::fmt::Debug;

use bevy_reflect::{FromReflect, Reflectable, Typed};
use rand_core::{RngCore, SeedableRng};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rand_chacha")]
pub use chacha::*;
#[cfg(feature = "rand_pcg")]
pub use pcg::*;
#[cfg(feature = "wyrand")]
pub use wyrand::WyRand;
#[cfg(feature = "rand_xoshiro")]
pub use xoshiro::*;

/// A marker trait to define the required trait bounds for a seedable PRNG to
/// integrate into `Entropy` or `GlobalEntropy`. This is a sealed trait.
#[cfg(feature = "serialize")]
pub trait EntropySource:
    RngCore
    + SeedableRng<Seed: Typed>
    + Clone
    + Debug
    + PartialEq
    + Sync
    + Send
    + FromReflect
    + Reflectable
    + Serialize
    + for<'a> Deserialize<'a>
    + private::SealedSeedable
{
}

/// Marker trait for a suitable seed for [`EntropySource`]. This is an auto trait which will
/// apply to all suitable types that meet the trait criteria.
#[cfg(feature = "serialize")]
pub trait EntropySeed:
    Debug
    + Default
    + PartialEq
    + AsMut<[u8]>
    + Clone
    + Sync
    + Send
    + Reflectable
    + FromReflect
    + Serialize
    + for<'a> Deserialize<'a>
{
}

#[cfg(feature = "serialize")]
impl<
    T: Debug
        + Default
        + PartialEq
        + AsMut<[u8]>
        + Clone
        + Sync
        + Send
        + Reflectable
        + FromReflect
        + Serialize
        + for<'a> Deserialize<'a>,
> EntropySeed for T
{
}

/// A marker trait to define the required trait bounds for a seedable PRNG to
/// integrate into `Entropy` or `GlobalEntropy`. This is a sealed trait.
#[cfg(not(feature = "serialize"))]
pub trait EntropySource:
    RngCore
    + SeedableRng<Seed: Typed>
    + Clone
    + Debug
    + PartialEq
    + Reflectable
    + FromReflect
    + Sync
    + Send
    + private::SealedSeedable
{
}

#[cfg(not(feature = "serialize"))]
/// Marker trait for a suitable seed for [`EntropySource`]. This is an auto trait which will
/// apply to all suitable types that meet the trait criteria.
pub trait EntropySeed:
    Debug + Default + PartialEq + AsMut<[u8]> + Clone + Sync + Send + Reflectable + FromReflect
{
}

#[cfg(not(feature = "serialize"))]
impl<T: Debug + Default + PartialEq + AsMut<[u8]> + Clone + Sync + Send + Reflectable + FromReflect>
    EntropySeed for T
{
}

mod private {
    pub trait SealedSeedable {}

    impl<T: super::EntropySource> SealedSeedable for T {}
}
