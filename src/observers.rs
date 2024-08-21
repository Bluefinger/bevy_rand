use std::marker::PhantomData;

use bevy::{
    app::App,
    prelude::{
        Commands, Component, Entity, Event, OnInsert, Query, ResMut, Trigger, With, Without,
    },
};

use bevy_prng::SeedableEntropySource;

use crate::prelude::{EntropyComponent, ForkableSeed, GlobalEntropy};

#[derive(Debug, Component)]
pub struct RngChildren<Source: SeedableEntropySource>(PhantomData<Source>);

#[derive(Debug, Component)]
pub struct RngParent<Source: SeedableEntropySource>(pub Entity, PhantomData<Source>);

#[derive(Debug, Event)]
pub struct SeedFromGlobal<Rng: SeedableEntropySource>(PhantomData<Rng>);

impl<Rng: SeedableEntropySource> SeedFromGlobal<Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn seed_from_global(
        trigger: Trigger<Self>,
        mut source: ResMut<GlobalEntropy<Rng>>,
        mut commands: Commands,
    ) {
        if let Some(mut entity) = commands.get_entity(trigger.entity()) {
            entity.insert(source.fork_seed());
        }
    }
}

#[derive(Debug, Event)]
pub struct LinkRngSourceToTarget<Target: Component, Rng: SeedableEntropySource> {
    rng: PhantomData<Rng>,
    target: PhantomData<Target>,
}

impl<Target: Component, Rng: SeedableEntropySource> LinkRngSourceToTarget<Target, Rng>
where
    Rng::Seed: Sync + Send + Clone,
{
    pub fn new() -> Self {
        Self {
            rng: PhantomData,
            target: PhantomData,
        }
    }

    pub(crate) fn initialize(app: &mut App) {
        app.observe(Self::link_targets).observe(Self::seed_children);
    }

    pub fn link_targets(
        trigger: Trigger<Self>,
        q_target: Query<Entity, With<Target>>,
        mut commands: Commands,
    ) {
        let parent = trigger.entity();

        for target in q_target.iter() {
            commands
                .entity(target)
                .insert(RngParent(parent, PhantomData::<Rng>));
        }

        commands
            .entity(parent)
            .insert(RngChildren(PhantomData::<Rng>));
    }

    pub fn seed_children(
        trigger: Trigger<OnInsert, EntropyComponent<Rng>>,
        mut q_source: Query<&mut EntropyComponent<Rng>, With<RngChildren<Rng>>>,
        q_target: Query<(Entity, &RngParent<Rng>), With<Target>>,
        mut commands: Commands,
    ) where
        Rng::Seed: Send + Sync + Clone,
    {
        let source = trigger.entity();

        if let Ok(mut rng) = q_source.get_mut(source) {
            q_target
                .iter()
                .filter_map(|(target, parent)| {
                    if parent.0 == source {
                        Some(target)
                    } else {
                        None
                    }
                })
                .for_each(|target| {
                    commands.entity(target).insert(rng.fork_seed());
                });
        }
    }
}

impl<Target: Component, Rng: SeedableEntropySource> Default for LinkRngSourceToTarget<Target, Rng>
where
    Rng::Seed: Sync + Send + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
