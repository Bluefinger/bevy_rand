#![warn(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Components for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod component;
/// Plugin for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod plugin;
/// Prelude for providing all necessary types for easy use.
pub mod prelude;
/// Resource for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod resource;
/// Seed Resource for seeding [`crate::resource::GlobalEntropy`].
pub mod seed;
#[cfg(feature = "thread_local_entropy")]
mod thread_local_entropy;
/// Traits for enabling utility methods for [`crate::component::EntropyComponent`] and [`crate::resource::GlobalEntropy`].
pub mod traits;
#[cfg(doc)]
pub mod tutorial;
/// Utility commands
pub mod observers;
