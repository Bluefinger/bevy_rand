use core::{fmt::Debug, ops::Deref};

use bevy_ecs::{
    component::Component,
    query::With,
    system::{Commands, Single, SystemParam},
};
use bevy_prng::EntropySource;

use crate::{
    params::{RngEntity, RngEntityItem},
    prelude::{Entropy, RngEntityCommands, RngEntityCommandsExt},
};

/// A marker component to signify a global source. Warning: there should only be **one** entity per
/// PRNG type that qualifies as the `Global` source.
#[derive(Debug, Component)]
pub struct Global;

/// A helper query to yield the [`Global`] source for a given [`bevy_prng::EntropySource`]. This returns the
/// [`Entropy`] component to generate new random numbers from.
pub type GlobalEntropy<'w, T> = Single<'w, &'static mut Entropy<T>, With<Global>>;

/// A helper [`SystemParam`] to obtain the [`Global`] entity & seed of a given `Rng`. This yields
/// read-only access to the global entity and its seed, and also allows constructing a
/// [`RngEntityCommands`] directly from it.
#[derive(SystemParam)]
pub struct GlobalRngEntity<'w, 's, Rng: EntropySource>
where
    Rng::Seed: Debug + Clone + Send + Sync,
{
    commands: Commands<'w, 's>,
    data: Single<'w, RngEntity<Rng>, With<Global>>,
}

impl<Rng: EntropySource> GlobalRngEntity<'_, '_, Rng>
where
    Rng::Seed: Debug + Send + Sync + Clone,
{
    /// Creates a [`Global`]'s [`RngEntityCommands`].
    pub fn rng_commands(&mut self) -> RngEntityCommands<'_, Rng> {
        self.commands.entity(self.data.entity()).rng()
    }
}

impl<'w, Rng: EntropySource> Deref for GlobalRngEntity<'w, '_, Rng>
where
    Rng::Seed: Debug + Clone + Send + Sync,
{
    type Target = RngEntityItem<'w, Rng>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
