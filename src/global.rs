use bevy_ecs::{component::Component, entity::Entity, query::With, system::Single};

use crate::{prelude::Entropy, seed::RngSeed};

/// A marker component to signify a global source. Warning: there should only be **one** entity per
/// PRNG type that qualifies as the `Global` source.
#[derive(Debug, Component)]
pub struct Global;

/// A helper query to yield the [`Global`] source for a given [`bevy_prng::EntropySource`]. This returns the
/// [`Entropy`] component to generate new random numbers from.
pub type GlobalEntropy<'w, T> = Single<'w, &'static mut Entropy<T>, With<Global>>;

/// A helper query to yield the [`Global`] source for a given [`EntropySource`]. This returns the
/// [`RngSeed`] component to allow inspection to the initial seed for the source.
pub type GlobalSeed<'w, T> = Single<'w, &'static RngSeed<T>, With<Global>>;

/// A helper query to yield the [`Global`] source for a given [`EntropySource`]. This returns the
/// [`Entity`] id to modify the source with via commands.
pub type GlobalSource<'w, T> = Single<'w, Entity, (With<RngSeed<T>>, With<Global>)>;
