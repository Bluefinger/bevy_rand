use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use bevy_ecs::{
    bundle::Bundle,
    entity::Entity,
    relationship::RelatedSpawnerCommands,
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

/// Extension trait for [`Commands`] for getting access to [`RngEntityCommands`].
pub trait RngEntityCommandsExt<'a> {
    /// Takes an [`Entity`] and yields the [`RngEntityCommands`] for that entity.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_prng::WyRand;
    /// use bevy_rand::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct Target;
    ///
    /// fn intialise_rng_entities(mut commands: Commands, mut q_targets: Query<Entity, With<Target>>) {
    ///     for target in &q_targets {
    ///         commands.entity(target).rng::<WyRand>().reseed_from_os_rng();
    ///     }
    /// }
    /// ```
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

impl<Rng: EntropySource> DerefMut for RngEntityCommands<'_, Rng> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

/// Extension trait to create a [`RngEntityCommands`] directly from a [`Commands`].
pub trait RngCommandsExt {
    /// Creates a [`RngEntityCommands`] from a given [`Entity`].
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_rand::prelude::*;
    /// use bevy_prng::WyRand;
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn reseed(mut commands: Commands, query: Query<RngEntity<WyRand>, With<Source>>) {
    ///     for entity in &query {
    ///         commands.rng_entity(&entity).reseed_linked();
    ///     }
    /// }
    /// ```
    fn rng_entity<Rng: EntropySource>(
        &mut self,
        entity: &RngEntityItem<'_, Rng>,
    ) -> RngEntityCommands<'_, Rng>
    where
        Rng::Seed: Debug + Clone + Send + Sync;
}

impl RngCommandsExt for Commands<'_, '_> {
    fn rng_entity<Rng: EntropySource>(
        &mut self,
        entity: &RngEntityItem<'_, Rng>,
    ) -> RngEntityCommands<'_, Rng>
    where
        Rng::Seed: Debug + Clone + Send + Sync,
    {
        self.entity(entity.entity()).rng()
    }
}

impl<Rng: EntropySource> RngEntityCommands<'_, Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    /// Reseeds the current `Rng` with a provided seed value.
    #[inline]
    pub fn reseed(&mut self, seed: Rng::Seed) -> &mut Self {
        self.commands.insert(RngSeed::<Rng>::from_seed(seed));

        self
    }

    /// Reseeds the current `Rng` with a new seed drawn from userspace entropy sources.
    ///
    /// # Panics
    ///
    /// Panics if it is unable to source entropy from a user-space source.
    #[inline]
    #[cfg(feature = "thread_local_entropy")]
    pub fn reseed_from_local_entropy(&mut self) -> &mut Self {
        self.commands.insert(RngSeed::<Rng>::from_local_entropy());

        self
    }

    /// Reseeds the current `Rng` with a new seed drawn from userspace entropy sources.
    #[inline]
    #[cfg(feature = "thread_local_entropy")]
    pub fn try_reseed_from_local_entropy(&mut self) -> Result<&mut Self, std::thread::AccessError> {
        self.commands
            .insert(RngSeed::<Rng>::try_from_local_entropy()?);

        Ok(self)
    }

    /// Reseeds the current `Rng` with a new seed drawn from OS sources.
    ///
    /// # Panics
    ///
    /// Panics if it is unable to source entropy from an OS/Hardware source.
    #[inline]
    pub fn reseed_from_os_rng(&mut self) -> &mut Self {
        self.commands.insert(RngSeed::<Rng>::from_os_rng());

        self
    }

    /// Reseeds the current `Rng` with a new seed drawn from OS sources.
    #[inline]
    pub fn try_reseed_from_os_rng(&mut self) -> Result<&mut Self, rand_core::OsError> {
        self.commands.insert(RngSeed::<Rng>::try_from_os_rng()?);

        Ok(self)
    }
}

impl<Rng: EntropySource> RngEntityCommands<'_, Rng> {
    /// Spawns entities related to the current `Source` Rng, linking them and then seeding
    /// them automatically.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_rand::prelude::*;
    /// use bevy_prng::WyRand;
    ///
    /// #[derive(Component)]
    /// struct Source;
    /// #[derive(Component)]
    /// struct Target;
    ///
    /// fn setup_rng_sources(mut global: GlobalRngEntity<WyRand>) {
    ///     global.rng_commands().with_target_rngs([(
    ///     Source,
    ///     RngLinks::<WyRand, WyRand>::spawn((
    ///         Spawn(Target),
    ///         Spawn(Target),
    ///         Spawn(Target),
    ///         Spawn(Target),
    ///         Spawn(Target),
    ///     )),
    /// )]);
    /// }
    /// ```
    #[inline]
    pub fn with_target_rngs(
        &mut self,
        targets: impl IntoIterator<Item = impl Bundle>,
    ) -> &mut Self {
        self.with_target_rngs_as::<Rng>(targets)
    }

    /// Spawns entities related to the current Source Rng, linking them and then seeding
    /// them automatically with the specified `Target` Rng.
    ///
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_rand::prelude::*;
    /// use bevy_prng::{ChaCha8Rng, WyRand};
    ///
    /// #[derive(Component)]
    /// struct Source;
    /// #[derive(Component)]
    /// struct Target;
    ///
    /// fn setup_rng_sources(mut global: GlobalRngEntity<ChaCha8Rng>) {
    ///     global.rng_commands().with_target_rngs_as::<WyRand>([(
    ///         Source,
    ///         RngLinks::<WyRand, WyRand>::spawn((
    ///             Spawn(Target),
    ///             Spawn(Target),
    ///             Spawn(Target),
    ///             Spawn(Target),
    ///             Spawn(Target),
    ///         )),
    ///     )]);
    /// }
    /// ```
    pub fn with_target_rngs_as<Target: EntropySource>(
        &mut self,
        targets: impl IntoIterator<Item = impl Bundle>,
    ) -> &mut Self {
        self.commands.with_related_entities(
            |related: &mut RelatedSpawnerCommands<'_, RngSource<Rng, Target>>| {
                targets.into_iter().for_each(|bundle| {
                    related.spawn(bundle);
                });
            },
        );

        self.reseed_linked_as::<Target>()
    }

    /// Links a list of target [`Entity`]s to the current `Rng`, designating it
    /// as the Source `Rng` for the Targets to draw new seeds from.
    #[inline]
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
    #[inline]
    pub fn reseed_linked(&mut self) -> &mut Self {
        self.reseed_linked_as::<Rng>()
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
    #[inline]
    pub fn reseed_from_source(&mut self) -> &mut Self {
        self.commands.trigger(SeedFromSource::<Rng, Rng>::default());

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
    #[inline]
    pub fn reseed_from_global(&mut self) -> &mut Self {
        self.commands.trigger(SeedFromGlobal::<Rng, Rng>::default());

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
    #[inline]
    pub fn entity_commands(&mut self) -> EntityCommands<'_> {
        self.commands.reborrow()
    }

    /// Returns the underlying [`Commands`].
    #[inline]
    pub fn commands(&mut self) -> Commands<'_, '_> {
        self.commands.commands()
    }
}
