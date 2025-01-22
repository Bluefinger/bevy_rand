use core::marker::PhantomData;

use bevy_ecs::{
    entity::Entity,
    system::{Commands, EntityCommands},
};
use bevy_prng::EntropySource;

use crate::observers::{RngSource, SeedFromGlobal, SeedFromSource, SeedLinked};

/// Commands for handling RNG specific operations with regards to seeding and
/// linking.
pub struct EntityRngCommands<'a, Rng: EntropySource> {
    commands: EntityCommands<'a>,
    _rng: PhantomData<Rng>,
}

/// Extension trait for [`Commands`] for getting access to [`EntityRngCommands`].
pub trait RngCommandsExt {
    /// Takes an [`Entity`] and yields the [`EntityRngCommands`] for that entity.
    fn rng<Rng: EntropySource>(&mut self, entity: Entity) -> EntityRngCommands<'_, Rng>;
}

impl RngCommandsExt for Commands<'_, '_> {
    fn rng<Rng: EntropySource>(&mut self, entity: Entity) -> EntityRngCommands<'_, Rng> {
        EntityRngCommands {
            commands: self.entity(entity),
            _rng: PhantomData,
        }
    }
}

impl<Rng: EntropySource> EntityRngCommands<'_, Rng> {
    /// Links a list of target [`Entity`]s to the current Rng, designating it
    /// as the Source Rng for the Targets to draw new seeds from.
    pub fn link_target_rngs(&mut self, targets: &[Entity]) -> &mut Self {
        self.commands.add_related::<RngSource<Rng>>(targets);

        self
    }

    /// Emits an event for the current Source Rng to push out new seeds to
    /// all linked target Rngs.
    pub fn reseed_linked(&mut self) -> &mut Self {
        self.commands.trigger(SeedLinked::<Rng>::default());

        self
    }

    /// Emits an event for the current Rng to pull a new seed from its linked
    /// Source Rng.
    pub fn reseed_from_source(&mut self) -> &mut Self {
        self.commands.trigger(SeedFromSource::<Rng>::default());

        self
    }

    /// Emits an event for the current Rng to pull a new seed from the
    /// Global Rng.
    pub fn reseed_from_global(&mut self) -> &mut Self {
        self.commands.trigger(SeedFromGlobal::<Rng>::default());

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
