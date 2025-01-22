use alloc::vec::Vec;
use core::marker::PhantomData;

use bevy_ecs::{
    component::{Immutable, Mutable, StorageType},
    prelude::{Commands, Component, Entity, Event, OnInsert, Trigger, With},
    relationship::{Relationship, RelationshipTarget},
    system::{Populated, Query},
};

use bevy_prng::EntropySource;

use crate::{
    prelude::{Entropy, ForkableSeed, GlobalEntropy},
    seed::RngSeed,
    traits::SeedSource,
};
/// Component to denote a source has linked children entities
#[derive(Debug)]
pub struct RngLinks<Source: EntropySource>(Vec<Entity>, PhantomData<Source>);

impl<Source: EntropySource> RelationshipTarget for RngLinks<Source> {
    type Relationship = RngSource<Source>;
    type Collection = Vec<Entity>;

    fn collection(&self) -> &Self::Collection {
        &self.0
    }

    fn collection_mut_risky(&mut self) -> &mut Self::Collection {
        &mut self.0
    }

    fn from_collection_risky(collection: Self::Collection) -> Self {
        Self(collection, PhantomData)
    }
}

impl<Source: EntropySource> Component for RngLinks<Source> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;

    fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
        hooks.on_replace(<Self as bevy_ecs::relationship::RelationshipTarget>::on_replace);
        hooks.on_despawn(<Self as bevy_ecs::relationship::RelationshipTarget>::on_despawn);
    }
}

impl<Source: EntropySource> Default for RngLinks<Source> {
    fn default() -> Self {
        Self(Vec::new(), PhantomData)
    }
}

/// Component to denote has a relation to a parent Rng source entity.
#[derive(Debug)]
pub struct RngSource<Source: EntropySource>(Entity, PhantomData<Source>);

impl<Source: EntropySource> Component for RngSource<Source> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Immutable;

    fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
        hooks.on_insert(<Self as bevy_ecs::relationship::Relationship>::on_insert);
        hooks.on_replace(<Self as bevy_ecs::relationship::Relationship>::on_replace);
    }
}

impl<Source: EntropySource> Relationship for RngSource<Source> {
    type RelationshipTarget = RngLinks<Source>;

    fn get(&self) -> Entity {
        self.0
    }

    fn from(entity: Entity) -> Self {
        Self(entity, PhantomData)
    }
}

impl<Source: EntropySource> RngSource<Source> {
    /// Initialises the relation component with the parent entity
    pub fn new(parent: Entity) -> Self {
        Self(parent, PhantomData)
    }

    /// Get the parent source entity
    pub fn entity(&self) -> Entity {
        self.0
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// GlobalEntropy source.
#[derive(Debug, Event)]
pub struct SeedFromGlobal<Rng: EntropySource>(PhantomData<Rng>);

impl<Rng: EntropySource> Default for SeedFromGlobal<Rng> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// GlobalEntropy source.
#[derive(Debug, Event)]
pub struct SeedLinked<Rng: EntropySource>(PhantomData<Rng>);

impl<Rng: EntropySource> Default for SeedLinked<Rng> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// linked parent entity.
#[derive(Debug, Event)]
pub struct SeedFromSource<Rng: EntropySource>(PhantomData<Rng>);

impl<Rng: EntropySource> Default for SeedFromSource<Rng> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Observer event for triggering an entity to use a new seed value from the
/// the event.
#[derive(Debug, Event)]
pub struct ReseedRng<Rng: EntropySource>(Rng::Seed);

impl<Rng: EntropySource> ReseedRng<Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    /// Create a new reseed event with a specified seed value.
    pub fn new(seed: Rng::Seed) -> Self {
        Self(seed)
    }
}

/// Observer event for linking a source Rng to one or many target Rngs. This then creates the
/// association needed so that when the source Rng's seed is changed, it propagates new seeds to
/// all linked Rngs.
#[derive(Debug, Event)]
pub struct LinkRngSourceToTarget<Rng: EntropySource> {
    rng: PhantomData<Rng>,
    source: Entity,
    target: Vec<Entity>,
}

impl<Rng: EntropySource> LinkRngSourceToTarget<Rng>
where
    Rng::Seed: Sync + Send + Clone,
{
    /// Construct a new linking event, taking one source [`Entity`] that will link to one
    /// or many [`Entity`],
    pub fn new(source: Entity, target: Vec<Entity>) -> Self {
        Self {
            rng: PhantomData,
            source,
            target,
        }
    }
}

/// Observer system for reseeding a target RNG on an entity with a provided seed value.
pub fn reseed<Rng: EntropySource>(trigger: Trigger<ReseedRng<Rng>>, mut commands: Commands)
where
    Rng::Seed: Sync + Send + Clone,
{
    let target = trigger.target();

    if target != Entity::PLACEHOLDER {
        commands
            .entity(target)
            .insert(RngSeed::<Rng>::from_seed(trigger.0.clone()));
    }
}

/// Observer System for pulling in a new seed from a GlobalEntropy source
pub fn seed_from_global<Rng: EntropySource>(
    trigger: Trigger<SeedFromGlobal<Rng>>,
    mut source: GlobalEntropy<Rng>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    if let Some(mut entity) = commands.get_entity(trigger.target()) {
        entity.insert(source.fork_seed());
    }
}

/// Observer System for pulling in a new seed for the current entity from its parent Rng source. This
/// observer system will only run if there are parent entities to have seeds pulled from.
pub fn seed_from_parent<Rng: EntropySource>(
    trigger: Trigger<SeedFromSource<Rng>>,
    q_linked: Populated<&RngSource<Rng>>,
    mut q_parents: Populated<&mut Entropy<Rng>, With<RngLinks<Rng>>>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    let target = trigger.target();

    if let Ok(mut rng) = q_linked
        .get(target)
        .and_then(|parent| q_parents.get_mut(parent.entity()))
    {
        commands.entity(target).insert(rng.fork_seed());
    }
}

/// Observer System for handling seed propagation from source Rng to all child entities. This observer
/// will only run if there is a source entity and also if there are target entities to seed.
pub fn seed_children<Rng: EntropySource>(
    trigger: Trigger<SeedLinked<Rng>>,
    mut q_source: Query<(&mut Entropy<Rng>, &RngLinks<Rng>)>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    if let Ok((mut rng, targets)) = q_source.get_mut(trigger.target()) {
        let batched: Vec<_> = targets
            .0
            .iter()
            .copied()
            .map(|target| (target, rng.fork_seed()))
            .collect();

        commands.insert_batch(batched);
    }
}

/// Observer System for triggering seed propagation from source Rng to all child entities. This observer
/// will only run if there is a source entity and also if there are target entities to seed.
pub fn trigger_seed_children<Rng: EntropySource>(
    trigger: Trigger<OnInsert, Entropy<Rng>>,
    q_source: Query<Entity, With<RngLinks<Rng>>>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    // Check whether the triggered entity is a source entity. If not, do nothing otherwise we
    // will keep triggering and cause a stack overflow.
    if let Ok(source) = q_source.get(trigger.target()) {
        commands.trigger_targets(SeedLinked::<Rng>::default(), source);
    }
}

/// Observer System for handling linking a source Rng with all target entities. This observer will only
/// run if there is a single source entity and if there are target entities to link with. If these assumptions
/// are not met, the observer system will not run.
pub fn link_targets<Rng: EntropySource>(
    trigger: Trigger<LinkRngSourceToTarget<Rng>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.source)
        .add_related::<RngSource<Rng>>(&trigger.target);
}
