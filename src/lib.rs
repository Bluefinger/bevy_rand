#![allow(clippy::type_complexity)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Components for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod component;
/// Global [`crate::component::Entropy`] sources, with query helpers.
pub mod global;
#[cfg(feature = "experimental")]
/// Utility observers for handling seeding between parent/child entropy sources
pub mod observers;
/// Plugin for integrating [`RngCore`] PRNGs into bevy. Must be newtyped to support [`Reflect`].
pub mod plugin;
/// Prelude for providing all necessary types for easy use.
pub mod prelude;
/// Seed Resource for seeding [`crate::resource::GlobalEntropy`].
pub mod seed;
#[cfg(feature = "thread_local_entropy")]
mod thread_local_entropy;
/// Traits for enabling utility methods for [`crate::component::Entropy`] and [`crate::resource::GlobalEntropy`].
pub mod traits;
#[cfg(doc)]
pub mod tutorial;
