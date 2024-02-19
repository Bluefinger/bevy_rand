#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

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

use std::fmt::Debug;

use bevy::{
    prelude::{FromReflect, Reflect},
    reflect::{GetTypeRegistration, TypePath},
};
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
#[cfg(feature = "rand_xoshiro")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_xoshiro")))]
pub use rand_xoshiro::Seed512;

/// A marker trait to define the required trait bounds for a seedable PRNG to
/// integrate into `EntropyComponent` or `GlobalEntropy`. This is a sealed trait.
#[cfg(feature = "serialize")]
pub trait SeedableEntropySource:
    RngCore
    + SeedableRng
    + Clone
    + Debug
    + PartialEq
    + Sync
    + Send
    + Reflect
    + TypePath
    + FromReflect
    + GetTypeRegistration
    + Serialize
    + for<'a> Deserialize<'a>
    + private::SealedSeedable
{
}

/// A marker trait to define the required trait bounds for a seedable PRNG to
/// integrate into `EntropyComponent` or `GlobalEntropy`. This is a sealed trait.
#[cfg(not(feature = "serialize"))]
pub trait SeedableEntropySource:
    RngCore
    + SeedableRng
    + Clone
    + Debug
    + PartialEq
    + Reflect
    + TypePath
    + FromReflect
    + GetTypeRegistration
    + Sync
    + Send
    + private::SealedSeedable
{
}

mod private {
    pub trait SealedSeedable {}

    impl<T: super::SeedableEntropySource> SealedSeedable for T {}
}
