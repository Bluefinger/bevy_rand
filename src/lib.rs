#![allow(clippy::type_complexity)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(missing_docs)]
#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Command extensions for relating groups of RNGs.
pub mod commands;
/// Global PRNG sources, with query helpers.
pub mod global;
/// Utility observers for handling seeding between parent/child entropy sources
pub mod observers;
/// Utility query/system parameters for accessing RNGs.
pub mod params;
/// Plugin for integrating [`rand_core::RngCore`] PRNGs into bevy. Must be newtyped to support [`bevy_reflect::Reflect`].
pub mod plugin;
/// Prelude for providing all necessary types for easy use.
pub mod prelude;
/// Seed Components for seeding PRNG components.
pub mod seed;
#[cfg(feature = "thread_local_entropy")]
mod thread_local_entropy;
/// Traits for enabling utility methods for PRNG components.
pub mod traits;
#[cfg(doc)]
pub mod tutorial;
