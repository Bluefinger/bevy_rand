use core::marker::PhantomData;

use bevy_ecs::{
    entity::Entity,
    system::{Commands, EntityCommands},
};
use bevy_prng::EntropySource;

use crate::observers::{RngParent, SeedChildren, SeedFromGlobal, SeedFromParent};

pub struct EntityRngCommands<'a, Rng: EntropySource> {
    commands: EntityCommands<'a>,
    _rng: PhantomData<Rng>,
}

pub trait RngCommandsExt {
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
    pub fn link_target_rngs(&mut self, targets: &[Entity]) -> &mut Self {
        self.commands.add_related::<RngParent<Rng>>(targets);

        self
    }

    pub fn reseed_linked(&mut self) -> &mut Self {
        self.commands.trigger(SeedChildren::<Rng>::default());

        self
    }

    pub fn reseed_from_source(&mut self) -> &mut Self {
        self.commands.trigger(SeedFromParent::<Rng>::default());

        self
    }

    pub fn reseed_from_global(&mut self) -> &mut Self {
        self.commands.trigger(SeedFromGlobal::<Rng>::default());

        self
    }

    pub fn entity_commands(&mut self) -> EntityCommands<'_> {
        self.commands.reborrow()
    }

    pub fn commands(&mut self) -> Commands<'_, '_> {
        self.commands.commands()
    }
}
