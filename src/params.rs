use core::fmt::Debug;

use bevy_ecs::{entity::Entity, query::QueryData};
use bevy_prng::EntropySource;

use crate::{seed::RngSeed, traits::SeedSource};

/// A smart query data wrapper that selects for entities that match the `Rng` type,
/// returning read-only data for the seed and the entity.
#[derive(Debug, QueryData)]
pub struct RngEntity<Rng: EntropySource>
where
    Rng::Seed: Debug + Clone + Send + Sync,
{
    seed: &'static RngSeed<Rng>,
    entity: Entity,
}

impl<'w, Rng: EntropySource> RngEntityItem<'w, Rng>
where
    Rng::Seed: Debug + Clone,
{
    /// Return the [`Entity`] of the data
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Get a reference to the [`RngSeed`] component for the given data
    pub fn seed(&self) -> &RngSeed<Rng> {
        self.seed
    }

    /// Clone the seed from the data
    pub fn clone_seed(&self) -> Rng::Seed {
        self.seed.clone_seed()
    }
}
