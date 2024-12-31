use std::marker::PhantomData;

use bevy_ecs::{
    prelude::{Commands, Component, Entity, Event, OnInsert, Trigger, With},
    query::Without,
    system::{Populated, Single},
};

use bevy_prng::EntropySource;

use crate::{
    prelude::{Entropy, ForkableSeed, GlobalEntropy},
    seed::RngSeed,
    traits::SeedSource,
};

/// Component to denote a source has linked children entities
#[derive(Debug, Component)]
pub struct RngChildren<Source: EntropySource>(PhantomData<Source>);

impl<Rng: EntropySource> Default for RngChildren<Rng> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Component to denote has a relation to a parent Rng source entity.
#[derive(Debug, Component)]
pub struct RngParent<Source: EntropySource>(Entity, PhantomData<Source>);

impl<Source: EntropySource> RngParent<Source> {
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
/// linked parent entity.
#[derive(Debug, Event)]
pub struct SeedFromParent<Rng: EntropySource>(PhantomData<Rng>);

impl<Rng: EntropySource> Default for SeedFromParent<Rng> {
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
pub struct LinkRngSourceToTarget<Source: Component, Target: Component, Rng: EntropySource> {
    rng: PhantomData<Rng>,
    source: PhantomData<Source>,
    target: PhantomData<Target>,
}

impl<Source: Component, Target: Component, Rng: EntropySource> Default
    for LinkRngSourceToTarget<Source, Target, Rng>
where
    Rng::Seed: Sync + Send + Clone,
{
    fn default() -> Self {
        Self {
            rng: PhantomData,
            source: PhantomData,
            target: PhantomData,
        }
    }
}

/// Observer system for reseeding a target RNG on an entity with a provided seed value.
pub fn reseed<Rng: EntropySource>(trigger: Trigger<ReseedRng<Rng>>, mut commands: Commands)
where
    Rng::Seed: Sync + Send + Clone,
{
    let target = trigger.entity();

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
    if let Some(mut entity) = commands.get_entity(trigger.entity()) {
        entity.insert(source.fork_seed());
    }
}

/// Observer System for pulling in a new seed for the current entity from its parent Rng source. This
/// observer system will only run if there are parent entities to have seeds pulled from.
pub fn seed_from_parent<Rng: EntropySource>(
    trigger: Trigger<SeedFromParent<Rng>>,
    q_linked: Populated<&RngParent<Rng>>,
    mut q_parents: Populated<&mut Entropy<Rng>, With<RngChildren<Rng>>>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    let target = trigger.entity();

    if let Ok(mut rng) = q_linked
        .get(target)
        .and_then(|parent| q_parents.get_mut(parent.entity()))
    {
        commands.entity(target).insert(rng.fork_seed());
    }
}

/// Observer System for handling seed propagation from source Rng to all child entities. This observer
/// will only run if there is a single source entity and also if there are target entities to seed.
pub fn seed_children<Source: Component, Target: Component, Rng: EntropySource>(
    trigger: Trigger<OnInsert, Entropy<Rng>>,
    q_source: Single<
        (Entity, &mut Entropy<Rng>),
        (With<Source>, With<RngChildren<Rng>>, Without<Target>),
    >,
    q_target: Populated<Entity, (With<Target>, With<RngParent<Rng>>, Without<Source>)>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    let (source, mut rng) = q_source.into_inner();
    // Check whether the triggered entity is a source entity. If not, do nothing otherwise we
    // will keep triggering and cause a stack overflow.
    if source == trigger.entity() {
        let batch: Vec<(Entity, RngSeed<Rng>)> = q_target
            .iter()
            .map(|target| (target, rng.fork_seed()))
            .collect();

        commands.insert_batch(batch);
    }
}

/// Observer System for handling linking a source Rng with all target entities. This observer will only
/// run if there is a single source entity and if there are target entities to link with. If these assumptions
/// are not met, the observer system will not run.
pub fn link_targets<Source: Component, Target: Component, Rng: EntropySource>(
    _trigger: Trigger<LinkRngSourceToTarget<Source, Target, Rng>>,
    q_source: Single<Entity, (With<Source>, Without<Target>)>,
    q_target: Populated<Entity, (With<Target>, Without<Source>)>,
    mut commands: Commands,
) {
    let parent = q_source.into_inner();

    let mut targets = q_target.iter();

    if targets.size_hint().0 == 1 {
        let target = targets.next().unwrap();

        commands
            .entity(target)
            .insert(RngParent::<Rng>::new(parent));
    } else {
        let targets: Vec<_> = targets
            .map(|target| (target, RngParent::<Rng>::new(parent)))
            .collect();

        commands.insert_batch(targets);
    }

    commands
        .entity(parent)
        .insert(RngChildren::<Rng>::default());
}
