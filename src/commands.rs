use core::{fmt::Debug, marker::PhantomData, ops::{Deref, DerefMut}};

use bevy_ecs::{
    entity::Entity,
    system::{Commands, EntityCommands},
};
use bevy_prng::EntropySource;

use crate::{
    observers::{RngSource, SeedFromGlobal, SeedFromSource, SeedLinked},
    params::RngEntityItem,
    seed::RngSeed,
    traits::SeedSource,
};

/// Commands for handling RNG specific operations with regards to seeding and
/// linking.
pub struct RngEntityCommands<'a, Rng: EntropySource> {
    commands: EntityCommands<'a>,
    _rng: PhantomData<Rng>,
}

/// Extension trait for [`Commands`] for getting access to [`EntityRngCommands`].
pub trait RngEntityCommandsExt<'a> {
    /// Takes an [`Entity`] and yields the [`EntityRngCommands`] for that entity.
    fn rng<Rng: EntropySource>(self) -> RngEntityCommands<'a, Rng>;
}

impl<'a> RngEntityCommandsExt<'a> for EntityCommands<'a> {
    fn rng<Rng: EntropySource>(self) -> RngEntityCommands<'a, Rng> {
        RngEntityCommands {
            commands: self,
            _rng: PhantomData,
        }
    }
}

impl<'a, Rng: EntropySource> Deref for RngEntityCommands<'a, Rng> {
    type Target = EntityCommands<'a>;

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}

impl<'a, Rng: EntropySource> DerefMut for RngEntityCommands<'a, Rng> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

/// Extension trait to create a [`RngEntityCommands`] directly from a [`Commands`].
pub trait RngCommandsExt {
    /// Creates a [`RngEntityCommands`] from a given [`Entity`].
    fn rng<Rng: EntropySource>(&mut self, entity: &RngEntityItem<'_, Rng>) -> RngEntityCommands<'_, Rng>
    where
        Rng::Seed: Debug + Clone + Send + Sync;
}

impl RngCommandsExt for Commands<'_, '_> {
    fn rng<Rng: EntropySource>(&mut self, entity: &RngEntityItem<'_, Rng>) -> RngEntityCommands<'_, Rng>
        where
            Rng::Seed: Debug + Clone + Send + Sync {
        self.entity(entity.entity()).rng()
    }
}

impl<Rng: EntropySource> RngEntityCommands<'_, Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    /// Reseeds the current `Rng` with a provided seed value.
    pub fn reseed(&mut self, seed: Rng::Seed) -> &mut Self {
        self.commands.insert(RngSeed::<Rng>::from_seed(seed));

        self
    }

    /// Reseeds the current `Rng` with a new seed drawn from OS or userspace entropy sources.
    pub fn reseed_from_entropy(&mut self) -> &mut Self {
        self.commands.insert(RngSeed::<Rng>::from_entropy());

        self
    }
}

impl<Rng: EntropySource> RngEntityCommands<'_, Rng> {
    /// Links a list of target [`Entity`]s to the current `Rng`, designating it
    /// as the Source `Rng` for the Targets to draw new seeds from.
    pub fn link_target_rngs(&mut self, targets: &[Entity]) -> &mut Self {
        self.commands.add_related::<RngSource<Rng, Rng>>(targets);

        self
    }

    /// Links a list of target [`Entity`]s to the current `Rng` as the specified `Target` type,
    /// designating it as the Source `Rng` for the Targets to draw new seeds from.
    pub fn link_target_rngs_as<Target: EntropySource>(&mut self, targets: &[Entity]) -> &mut Self {
        self.commands.add_related::<RngSource<Rng, Target>>(targets);

        self
    }

    /// Emits an event for the current Source `Rng` to generate and push out new seeds to
    /// all linked target `Rng`s.
    pub fn reseed_linked(&mut self) -> &mut Self {
        self.commands.trigger(SeedLinked::<Rng, Rng>::default());

        self
    }

    /// Emits an event for the current Source `Rng` to generate and push out new seeds to
    /// all linked target `Rng`s as the specified `Target` type.
    pub fn reseed_linked_as<Target: EntropySource>(&mut self) -> &mut Self {
        self.commands.trigger(SeedLinked::<Rng, Target>::default());

        self
    }

    /// Emits an event for the current `Rng` to pull a new seed from its linked
    /// Source `Rng`. This method assumes the `Source` and `Target` are the same `Rng`
    /// type.
    pub fn reseed_from_source(&mut self) -> &mut Self {
        self.commands
            .trigger(SeedFromSource::<Rng, Rng>::default());

        self
    }

    /// Emits an event for the current `Rng` to pull a new seed from its linked
    /// Source `Rng`. A `Rng` entity can have multiple linked sources, so a source
    /// `Rng` must be specified explicitly if you want to pull from a `Source` that
    /// isn't the same `Rng` kind as the target.
    pub fn reseed_from_source_as<Source: EntropySource>(&mut self) -> &mut Self {
        self.commands
            .trigger(SeedFromSource::<Source, Rng>::default());

        self
    }

    /// Emits an event for the current `Rng` to pull a new seed from the specified
    /// Global `Rng`.
    pub fn reseed_from_global(&mut self) -> &mut Self {
        self.commands
            .trigger(SeedFromGlobal::<Rng, Rng>::default());

        self
    }

    /// Emits an event for the current `Rng` to pull a new seed from the specified
    /// Global `Rng`.
    pub fn reseed_from_global_as<Source: EntropySource>(&mut self) -> &mut Self {
        self.commands
            .trigger(SeedFromGlobal::<Source, Rng>::default());

        self
    }

    /// Returns the inner [`EntityCommands`] with a smaller lifetime.
    pub fn entity_commands(&mut self) -> EntityCommands<'_> {
        self.commands.reborrow()
    }

    /// Returns the underlying [`Commands`].
    pub fn commands(&mut self) -> Commands<'_, '_> {
        self.commands.commands()
    }
}
