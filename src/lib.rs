#![warn(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// Components for integrating `RngCore` PRNGs into bevy.
pub mod component;
/// Plugin for integrating `RngCore` PRNGs into bevy.
pub mod plugin;
/// Prelude for providing all necessary types for easy use.
pub mod prelude;
/// Resource for integrating `RngCore` PRNGs into bevy.
pub mod resource;
#[cfg(feature = "thread_local_entropy")]
mod thread_local_entropy;
mod traits;
