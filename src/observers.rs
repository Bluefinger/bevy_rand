use std::marker::PhantomData;

use bevy_ecs::{
    prelude::{
        Commands, Component, Entity, EntityWorldMut, Event, OnInsert, Query, ResMut, Trigger, With,
    },
    query::QuerySingleError,
};

use bevy_prng::SeedableEntropySource;

use crate::{
    prelude::{EntropyComponent, ForkableSeed, GlobalEntropy},
    seed::RngSeed,
};

/// Component to denote a source has linked children entities
#[derive(Debug, Component)]
pub struct RngChildren<Source: SeedableEntropySource>(PhantomData<Source>);

impl<Rng: SeedableEntropySource> Default for RngChildren<Rng> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Component to denote has a relation to a parent Rng source entity.
#[derive(Debug, Component)]
pub struct RngParent<Source: SeedableEntropySource>(Entity, PhantomData<Source>);

impl<Source: SeedableEntropySource> RngParent<Source> {
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
pub struct SeedFromGlobal<Rng: SeedableEntropySource>(PhantomData<Rng>);

impl<Rng: SeedableEntropySource> Default for SeedFromGlobal<Rng> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Observer event for triggering an entity to pull a new seed value from a
/// linked parent entity.
#[derive(Debug, Event)]
pub struct SeedFromParent<Rng: SeedableEntropySource>(PhantomData<Rng>);

impl<Rng: SeedableEntropySource> Default for SeedFromParent<Rng> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Observer event for linking a source Rng to one or many target Rngs. This then creates the
/// association needed so that when the source Rng's seed is changed, it propagates new seeds to
/// all linked Rngs.
#[derive(Debug, Event)]
pub struct LinkRngSourceToTarget<Source: Component, Target: Component, Rng: SeedableEntropySource> {
    rng: PhantomData<Rng>,
    source: PhantomData<Source>,
    target: PhantomData<Target>,
}

impl<Source: Component, Target: Component, Rng: SeedableEntropySource> Default
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

/// Observer System for pulling in a new seed from a GlobalEntropy source
pub fn seed_from_global<Rng: SeedableEntropySource>(
    trigger: Trigger<SeedFromGlobal<Rng>>,
    mut source: ResMut<GlobalEntropy<Rng>>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    if let Some(mut entity) = commands.get_entity(trigger.entity()) {
        entity.insert(source.fork_seed());
    }
}

/// Observer System for pulling in a new seed for the current entity from its parent Rng source.
pub fn seed_from_parent<Rng: SeedableEntropySource>(
    trigger: Trigger<SeedFromParent<Rng>>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    let entity = trigger.entity();

    if entity != Entity::PLACEHOLDER {
        commands.entity(entity).queue(|mut entity: EntityWorldMut| {
            let Some(parent) = entity.get::<RngParent<Rng>>().map(|parent| parent.entity()) else {
                return;
            };
            entity
                .world_scope(|world| {
                    world.get_entity_mut(parent).ok().and_then(|mut parent| {
                        parent
                            .get_mut::<EntropyComponent<Rng>>()
                            .map(|mut rng| rng.fork_seed())
                    })
                })
                .map(|seed| entity.insert(seed));
        });
    }
}

/// Observer System for handling seed propagation from source Rng to all child entities.
pub fn seed_children<Source: Component, Target: Component, Rng: SeedableEntropySource>(
    trigger: Trigger<OnInsert, EntropyComponent<Rng>>,
    mut q_source: Query<&mut EntropyComponent<Rng>, (With<Source>, With<RngChildren<Rng>>)>,
    q_target: Query<Entity, With<Target>>,
    mut commands: Commands,
) where
    Rng::Seed: Send + Sync + Clone,
{
    let source = trigger.entity();

    if source != Entity::PLACEHOLDER {
        if let Ok(mut rng) = q_source.get_mut(source) {
            let batch: Vec<(Entity, RngSeed<Rng>)> = q_target
                .iter()
                .map(|target| (target, rng.fork_seed()))
                .collect();

            commands.insert_batch(batch);
        }
    }
}

/// Observer System for handling linking a source Rng with all target entities.
pub fn link_targets<Source: Component, Target: Component, Rng: SeedableEntropySource>(
    _trigger: Trigger<LinkRngSourceToTarget<Source, Target, Rng>>,
    q_source: Query<Entity, With<Source>>,
    q_target: Query<Entity, With<Target>>,
    mut commands: Commands,
) {
    let source = match q_source.get_single() {
        Ok(parent) => Some(parent),
        // If we somehow have more than one source, just use the first one and stick with that.
        Err(QuerySingleError::MultipleEntities(_)) => q_source.iter().next(),
        Err(QuerySingleError::NoEntities(_)) => None,
    };

    if let Some(parent) = source {
        let targets: Vec<_> = q_target
            .iter()
            .map(|target| (target, RngParent::<Rng>::new(parent)))
            .collect();

        commands.insert_batch(targets);

        commands
            .entity(parent)
            .insert(RngChildren::<Rng>::default());
    }
}
