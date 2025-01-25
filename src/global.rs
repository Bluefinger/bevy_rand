use core::fmt::Debug;
use core::marker::PhantomData;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{QueryData, ReadOnlyQueryData, With},
    system::{Commands, Single, SystemParam},
    world::Mut,
};
use bevy_prng::EntropySource;

use crate::{
    prelude::{EntityRngCommands, Entropy, RngCommandsExt},
    seed::RngSeed,
};

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

#[derive(Debug, QueryData)]
#[query_data(mutable)]
pub struct RngData<Rng: EntropySource>
where
    Rng::Seed: Debug + Send + Sync + Clone,
{
    entropy: &'static mut Entropy<Rng>,
    seed: &'static RngSeed<Rng>,
    entity: Entity,
}

impl<Rng: EntropySource> RngDataReadOnlyItem<'_, Rng>
where
    Rng::Seed: Debug + Send + Sync + Clone,
{
    fn entity(&self) -> Entity {
        self.entity
    }

    fn seed(&self) -> &RngSeed<Rng> {
        self.seed
    }
}

impl<'a, Rng: EntropySource> RngDataItem<'a, Rng>
where
    Rng::Seed: Debug + Send + Sync + Clone,
{
    fn entropy(&mut self) -> &mut Entropy<Rng> {
        self.entropy.as_mut()
    }

    fn rng_commands(&self, commands: &'a mut Commands) -> EntityRngCommands<'_, Rng>
    {
        commands.rng(self.entity)
    }
}

#[derive(SystemParam)]
pub struct GlobalRng<'w, 's, Rng: EntropySource>
where
    Rng::Seed: Debug + Send + Sync + Clone,
{
    commands: Commands<'w, 's>,
    data: Single<'w, RngData<Rng>, With<Global>>,
    _source: PhantomData<Rng>,
}

impl<Rng: EntropySource> GlobalRng<'_, '_, Rng>
where
    Rng::Seed: Debug + Send + Sync + Clone,
{
    pub fn rng_commands(&mut self) -> EntityRngCommands<'_, Rng> {
        self.commands.rng(self.data.entity)
    }
}
