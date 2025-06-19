use core::fmt::Debug;

use bevy_ecs::{entity::Entity, query::QueryData};
use bevy_prng::EntropySource;

use crate::{seed::RngSeed, traits::SeedSource};

/// A smart query data wrapper that selects for entities that match the `Rng` type,
/// returning read-only data for the seed and the entity.
#[derive(Debug, QueryData)]
pub struct RngEntity<Rng: EntropySource> {
    seed: &'static RngSeed<Rng>,
    entity: Entity,
}

impl<Rng: EntropySource> RngEntityItem<'_, '_, Rng> {
    /// Return the [`Entity`] of the data
    #[inline]
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Get a reference to the [`RngSeed`] component for the given data
    #[inline]
    pub fn seed(&self) -> &RngSeed<Rng> {
        self.seed
    }

    /// Clone the seed from the data
    #[inline]
    pub fn clone_seed(&self) -> Rng::Seed {
        self.seed.clone_seed()
    }
}
