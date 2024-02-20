#![warn(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// Components for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod component;
/// Plugin for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod plugin;
/// Prelude for providing all necessary types for easy use.
pub mod prelude;
/// Resource for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod resource;
#[cfg(feature = "thread_local_entropy")]
mod thread_local_entropy;
/// Traits for enabling utility methods for [`crate::component::EntropyComponent`] and [`crate::resource::GlobalEntropy`].
pub mod traits;
#[cfg(doc)]
pub mod tutorial;

#[cfg(feature = "internal_benchmarks")]
/// Only for benches
pub const BENCH: bool = true;
