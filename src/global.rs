use core::fmt::Debug;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Single, SystemParam},
};
use bevy_prng::EntropySource;

use crate::{
    params::RngEntity,
    prelude::{Entropy, RngEntityCommands, RngEntityCommandsExt},
    seed::RngSeed,
};

/// A marker component to signify a global source. Warning: there should only be **one** entity per
/// PRNG type that qualifies as the `Global` source.
#[derive(Debug, Component)]
pub struct Global;

/// A helper query to yield the [`Global`] source for a given [`bevy_prng::EntropySource`]. This returns the
/// [`Entropy`] component to generate new random numbers from.
pub type GlobalEntropy<'w, 's, T> = Single<'w, 's, &'static mut Entropy<T>, With<Global>>;

/// A helper [`SystemParam`] to obtain the [`Global`] entity & seed of a given `Rng`. This yields
/// read-only access to the global entity and its seed, and also allows constructing a
/// [`RngEntityCommands`] directly from it.
/// ```
/// use bevy_ecs::prelude::*;
/// use bevy_rand::prelude::*;
/// use bevy_prng::WyRand;
///
/// fn reseed_all_linked_rngs_from_global(mut global: GlobalRngEntity<WyRand>) {
///     global.rng_commands().reseed_linked();
/// }
/// ```
#[derive(SystemParam)]
pub struct GlobalRngEntity<'w, 's, Rng: EntropySource> {
    commands: Commands<'w, 's>,
    data: Single<'w, 's, RngEntity<Rng>, With<Global>>,
}

impl<Rng: EntropySource> GlobalRngEntity<'_, '_, Rng> {
    /// Creates a [`Global`]'s [`RngEntityCommands`].
    pub fn rng_commands(&mut self) -> RngEntityCommands<'_, Rng> {
        self.commands.entity(self.data.entity()).rng()
    }

    /// Return the [`Entity`] of the data
    pub fn entity(&self) -> Entity {
        self.data.entity()
    }

    /// Get a reference to the [`RngSeed`] component for the given data
    pub fn seed(&self) -> &RngSeed<Rng> {
        self.data.seed()
    }

    /// Clone the seed from the data
    pub fn clone_seed(&self) -> Rng::Seed {
        self.data.clone_seed()
    }
}
