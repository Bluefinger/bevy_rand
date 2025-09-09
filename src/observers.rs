use alloc::vec::Vec;
use core::{fmt::Debug, marker::PhantomData};

use bevy_ecs::{
    error::Result,
    event::EntityEvent,
    lifecycle::Insert,
    observer::On,
    prelude::{Commands, Component, Entity, With},
    system::{Query, Single},
};

use bevy_prng::EntropySource;

use crate::{
    global::GlobalRng,
    params::RngEntity,
    prelude::{Entropy, RngCommandsExt},
    traits::ForkableAsSeed,
};

/// Component to denote a source has linked children entities
#[derive(Debug, Component)]
#[relationship_target(relationship = RngSource<Source, Target>)]
pub struct RngLinks<Source: EntropySource, Target: EntropySource> {
    #[relationship]
    related: Vec<Entity>,
    _source: PhantomData<Source>,
    _target: PhantomData<Target>,
}

impl<Source: EntropySource, Target: EntropySource> Default for RngLinks<Source, Target> {
    #[inline]
    fn default() -> Self {
        Self {
            related: Vec::new(),
            _source: PhantomData,
            _target: PhantomData,
        }
    }
}

/// Component to denote that the current Entity has a relation to a parent Rng source entity.
#[derive(Debug, Component)]
#[relationship(relationship_target = RngLinks<Source, Target>)]
pub struct RngSource<Source: EntropySource, Target: EntropySource> {
    #[relationship]
    linked: Entity,
    _source: PhantomData<Source>,
    _target: PhantomData<Target>,
}

impl<Source: EntropySource, Target: EntropySource> RngSource<Source, Target> {
    /// Initialises the relation component with the parent entity
    #[inline]
    pub fn new(parent: Entity) -> Self {
        Self {
            linked: parent,
            _source: PhantomData,
            _target: PhantomData,
        }
    }

    /// Get the parent source entity
    #[inline]
    pub fn entity(&self) -> Entity {
        self.linked
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// GlobalEntropy source.
#[derive(Debug, EntityEvent)]
pub struct SeedFromGlobal<Source, Target>(PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> Default for SeedFromGlobal<Source, Target> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// GlobalEntropy source.
#[derive(Debug, EntityEvent)]
pub struct SeedLinked<Source, Target>(PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> Default for SeedLinked<Source, Target> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// linked parent entity.
#[derive(Debug, EntityEvent)]
pub struct SeedFromSource<Source, Target>(PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> Default for SeedFromSource<Source, Target> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Observer System for pulling in a new seed from a GlobalEntropy source
pub fn seed_from_global<Source: EntropySource, Target: EntropySource>(
    event: On<SeedFromGlobal<Source, Target>>,
    mut source: Single<&mut Entropy<Source>, With<GlobalRng>>,
    mut commands: Commands,
) -> Result {
    let target = event.entity();

    let mut entity = commands.get_entity(target)?;

    entity.insert(source.fork_as_seed::<Target>());

    Ok(())
}

/// Observer System for pulling in a new seed for the current entity from its parent Rng source. This
/// observer system will only run if there are parent entities to have seeds pulled from.
pub fn seed_from_parent<Source: EntropySource, Target: EntropySource>(
    event: On<SeedFromSource<Source, Target>>,
    q_linked: Query<&RngSource<Source, Target>>,
    mut q_parents: Query<&mut Entropy<Source>, With<RngLinks<Source, Target>>>,
    mut commands: Commands,
) -> Result {
    let target = event.entity();

    let rng = q_linked
        .get(target)
        .and_then(|parent| q_parents.get_mut(parent.entity()))
        .map(|mut rng| rng.fork_as_seed::<Target>())?;

    // This won't panic, because we've already checked in the .get above whether `target` exists.
    commands.entity(target).insert(rng);

    Ok(())
}

/// Observer System for handling seed propagation from source Rng to all child entities. This observer
/// will only run if there is a source entity and also if there are target entities to seed.
pub fn seed_linked<Source: EntropySource, Target: EntropySource>(
    event: On<SeedLinked<Source, Target>>,
    mut q_source: Query<(&mut Entropy<Source>, &RngLinks<Source, Target>)>,
    mut commands: Commands,
) -> Result {
    let target = event.entity();

    let (mut rng, targets) = q_source.get_mut(target)?;

    let batched: Vec<_> = targets
        .related
        .iter()
        .copied()
        .map(|target| (target, rng.fork_as_seed::<Target>()))
        .collect();

    commands.insert_batch(batched);

    Ok(())
}

/// Observer System for triggering seed propagation from source Rng to all child entities. This observer
/// will only run if there is a source entity and also if there are target entities to seed.
pub fn trigger_seed_linked<Source: EntropySource, Target: EntropySource>(
    event: On<Insert, Entropy<Source>>,
    q_source: Query<RngEntity<Source>, With<RngLinks<Source, Target>>>,
    mut commands: Commands,
) {
    let target = event.entity();

    // Check whether the triggered entity is a source entity. If not, do nothing otherwise we
    // will keep triggering and cause a stack overflow.
    if let Ok(mut rng_source) = q_source
        .get(target)
        .map(|source| commands.rng_entity(&source))
    {
        rng_source.reseed_linked_as::<Target>();
    }
}
