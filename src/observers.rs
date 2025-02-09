use alloc::vec::Vec;
use core::marker::PhantomData;

use bevy_ecs::{
    component::{ComponentHooks, Immutable, Mutable, StorageType},
    prelude::{Commands, Component, Entity, Event, OnInsert, Trigger, With},
    relationship::{Relationship, RelationshipTarget},
    system::Query,
};

use bevy_prng::EntropySource;

use crate::{
    prelude::{Entropy, GlobalEntropy},
    traits::ForkableAsSeed,
};

/// Component to denote a source has linked children entities
#[derive(Debug)]
pub struct RngLinks<Source, Target>(Vec<Entity>, PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> RelationshipTarget for RngLinks<Source, Target> {
    type Relationship = RngSource<Source, Target>;
    type Collection = Vec<Entity>;
    const LINKED_SPAWN: bool = false;

    fn collection(&self) -> &Self::Collection {
        &self.0
    }

    fn collection_mut_risky(&mut self) -> &mut Self::Collection {
        &mut self.0
    }

    fn from_collection_risky(collection: Self::Collection) -> Self {
        Self(collection, PhantomData, PhantomData)
    }
}

impl<Source: EntropySource, Target: EntropySource> Component for RngLinks<Source, Target> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_replace(<Self as RelationshipTarget>::on_replace);
        hooks.on_despawn(<Self as RelationshipTarget>::on_despawn);
    }
}

impl<Source: EntropySource, Target: EntropySource> Default for RngLinks<Source, Target> {
    fn default() -> Self {
        Self(Vec::new(), PhantomData, PhantomData)
    }
}

/// Component to denote that the current Entity has a relation to a parent Rng source entity.
#[derive(Debug)]
pub struct RngSource<Source, Target>(Entity, PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> Component for RngSource<Source, Target> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Immutable;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(<Self as Relationship>::on_insert);
        hooks.on_replace(<Self as Relationship>::on_replace);
    }
}

impl<Source: EntropySource, Target: EntropySource> Relationship for RngSource<Source, Target> {
    type RelationshipTarget = RngLinks<Source, Target>;

    fn get(&self) -> Entity {
        self.0
    }

    fn from(entity: Entity) -> Self {
        Self(entity, PhantomData, PhantomData)
    }
}

impl<Source: EntropySource, Target: EntropySource> RngSource<Source, Target> {
    /// Initialises the relation component with the parent entity
    pub fn new(parent: Entity) -> Self {
        Self(parent, PhantomData, PhantomData)
    }

    /// Get the parent source entity
    pub fn entity(&self) -> Entity {
        self.0
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// GlobalEntropy source.
#[derive(Debug, Event)]
pub struct SeedFromGlobal<Source, Target>(PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> Default for SeedFromGlobal<Source, Target> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// GlobalEntropy source.
#[derive(Debug, Event)]
pub struct SeedLinked<Source, Target>(PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> Default for SeedLinked<Source, Target> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// linked parent entity.
#[derive(Debug, Event)]
pub struct SeedFromSource<Source, Target>(PhantomData<Source>, PhantomData<Target>);

impl<Source: EntropySource, Target: EntropySource> Default for SeedFromSource<Source, Target> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Observer System for pulling in a new seed from a GlobalEntropy source
pub fn seed_from_global<Source: EntropySource, Target: EntropySource>(
    trigger: Trigger<SeedFromGlobal<Source, Target>>,
    mut source: GlobalEntropy<Source>,
    mut commands: Commands,
) where
    Source::Seed: Send + Sync + Clone,
    Target::Seed: Send + Sync + Clone,
{
    if let Some(mut entity) = commands.get_entity(trigger.target()) {
        entity.insert(source.fork_as_seed::<Target>());
    }
}

/// Observer System for pulling in a new seed for the current entity from its parent Rng source. This
/// observer system will only run if there are parent entities to have seeds pulled from.
pub fn seed_from_parent<Source: EntropySource, Target: EntropySource>(
    trigger: Trigger<SeedFromSource<Source, Target>>,
    q_linked: Query<&RngSource<Source, Target>>,
    mut q_parents: Query<&mut Entropy<Source>, With<RngLinks<Source, Target>>>,
    mut commands: Commands,
) where
    Source::Seed: Send + Sync + Clone,
    Target::Seed: Send + Sync + Clone,
{
    let target = trigger.target();

    if let Ok(mut rng) = q_linked
        .get(target)
        .and_then(|parent| q_parents.get_mut(parent.entity()))
    {
        commands.entity(target).insert(rng.fork_as_seed::<Target>());
    }
}

/// Observer System for handling seed propagation from source Rng to all child entities. This observer
/// will only run if there is a source entity and also if there are target entities to seed.
pub fn seed_linked<Source: EntropySource, Target: EntropySource>(
    trigger: Trigger<SeedLinked<Source, Target>>,
    mut q_source: Query<(&mut Entropy<Source>, &RngLinks<Source, Source>)>,
    mut commands: Commands,
) where
    Source::Seed: Send + Sync + Clone,
    Target::Seed: Send + Sync + Clone,
{
    if let Ok((mut rng, targets)) = q_source.get_mut(trigger.target()) {
        let batched: Vec<_> = targets
            .0
            .iter()
            .copied()
            .map(|target| (target, rng.fork_as_seed::<Target>()))
            .collect();

        commands.insert_batch(batched);
    }
}

/// Observer System for triggering seed propagation from source Rng to all child entities. This observer
/// will only run if there is a source entity and also if there are target entities to seed.
pub fn trigger_seed_linked<Source: EntropySource, Target: EntropySource>(
    trigger: Trigger<OnInsert, Entropy<Source>>,
    q_source: Query<Entity, With<RngLinks<Source, Target>>>,
    mut commands: Commands,
) where
    Source::Seed: Send + Sync + Clone,
    Target::Seed: Send + Sync + Clone,
{
    // Check whether the triggered entity is a source entity. If not, do nothing otherwise we
    // will keep triggering and cause a stack overflow.
    if let Ok(source) = q_source.get(trigger.target()) {
        commands.trigger_targets(SeedLinked::<Source, Target>::default(), source);
    }
}
