use bevy::{
    app::{App, PreStartup, Update},
    prelude::{Commands, Query, ResMut},
};
use bevy_prng::{ChaCha8Rng, WyRand};
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::EntropyComponent,
    resource::GlobalEntropy,
    traits::{ForkableAsSeed, ForkableSeed, SeedSource},
};
use rand_core::{RngCore, SeedableRng};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_global_reseeding() {
    let mut app = App::new();

    let seed = [2; 32];

    let rng_eq = GlobalEntropy::<ChaCha8Rng>::from_seed(seed);

    app.add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed(seed));

    {
        let global_rng = app.world().resource_ref::<GlobalEntropy<ChaCha8Rng>>();

        // Our RNGs should be the same as each other as they were initialised with the same seed
        assert_eq!(global_rng.as_ref(), &rng_eq);
    }

    app.update();

    {
        let global_rng = app.world().resource_ref::<GlobalEntropy<ChaCha8Rng>>();

        // Our RNGs should remain the same as each other as we have not run the update
        assert_eq!(global_rng.as_ref(), &rng_eq);
    }

    {
        let mut global_seed = app.world_mut().resource_mut::<GlobalEntropy<ChaCha8Rng>>();

        global_seed.reseed([3; 32]);
    }

    app.update();

    {
        let global_rng = app.world().resource_ref::<GlobalEntropy<ChaCha8Rng>>();

        // Now our RNG will not be the same, even though we did not use it directly
        assert_ne!(global_rng.as_ref(), &rng_eq);
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn component_fork_seed() {
    let mut app = App::new();

    let seed = [2; 32];

    app.add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed(seed))
        .add_systems(
            PreStartup,
            |mut commands: Commands, mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>| {
                for _ in 0..5 {
                    commands.spawn(rng.fork_seed());
                }
            },
        )
        .add_systems(
            Update,
            |mut q_rng: Query<&mut EntropyComponent<ChaCha8Rng>>| {
                let rngs = q_rng.iter_mut();

                assert_eq!(rngs.size_hint().0, 5);

                let values: Vec<_> = rngs.map(|mut rng| rng.next_u32()).collect();

                assert_eq!(
                    &values,
                    &[3315785188, 1951699392, 911252207, 791343233, 1599472206]
                );
            },
        );

    app.update();
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn component_fork_as_seed() {
    let mut app = App::new();

    let seed = [2; 32];

    app.add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed(seed))
        .add_systems(
            PreStartup,
            |mut commands: Commands, mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>| {
                for _ in 0..5 {
                    commands.spawn(rng.fork_as_seed::<WyRand>());
                }
            },
        )
        .add_systems(Update, |mut q_rng: Query<&mut EntropyComponent<WyRand>>| {
            let rngs = q_rng.iter_mut();

            assert_eq!(rngs.size_hint().0, 5);

            let values: Vec<_> = rngs.map(|mut rng| rng.next_u64()).collect();

            assert_eq!(
                &values,
                &[
                    10032395693880520184,
                    15375025802368380325,
                    10863580644061233257,
                    7067543572507795213,
                    7996461288508244033
                ]
            );
        });

    app.update();
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn observer_global_reseeding() {
    use bevy::prelude::{Entity, Event, PostUpdate, PreUpdate, Startup, Trigger, With};
    use bevy_rand::{seed::RngSeed, traits::ForkableInnerSeed};

    let seed = [2; 8];

    #[derive(Event)]
    struct Reseed([u8; 8]);

    fn reseed(trigger: Trigger<Reseed>, mut commands: Commands) {
        if let Some(mut entity) = commands.get_entity(trigger.entity()) {
            let seed = trigger.event();
            entity.insert(RngSeed::<WyRand>::from_seed(seed.0));
        }
    }

    let mut app = App::new();

    app.add_plugins(EntropyPlugin::<WyRand>::with_seed(seed))
        .add_systems(
            Startup,
            |mut commands: Commands, mut source: ResMut<GlobalEntropy<WyRand>>| {
                for _ in 0..5 {
                    commands.spawn(source.fork_seed());
                }
            },
        )
        .add_systems(PreUpdate, |query: Query<&RngSeed<WyRand>>| {
            let expected = [
                2484862625678185386u64,
                10323237495534242118,
                14704548354072994214,
                14638519449267265798,
                11723565746675474547,
            ];
            let seeds = query.iter().map(RngSeed::<WyRand>::clone_seed);

            expected
                .into_iter()
                .zip(seeds.map(u64::from_ne_bytes))
                .for_each(|(expected, actual)| assert_eq!(expected, actual));
        })
        .add_systems(
            Update,
            |mut commands: Commands,
             query: Query<Entity, With<EntropyComponent<WyRand>>>,
             mut source: ResMut<GlobalEntropy<WyRand>>| {
                for e in &query {
                    commands.trigger_targets(Reseed(source.fork_inner_seed()), e);
                }
            },
        )
        .add_systems(PostUpdate, |query: Query<&RngSeed<WyRand>>| {
            let prev_expected = [
                2484862625678185386u64,
                10323237495534242118,
                14704548354072994214,
                14638519449267265798,
                11723565746675474547,
            ];
            let seeds = query.iter().map(RngSeed::<WyRand>::clone_seed);

            prev_expected
                .into_iter()
                .zip(seeds.map(u64::from_ne_bytes))
                .for_each(|(expected, actual)| assert_ne!(expected, actual));
        })
        .observe(reseed);

    app.run();
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn observer_children_reseeding() {
    use bevy::prelude::{
        Component, Entity, EntityWorldMut, Event, Last, PostUpdate, PreUpdate, Startup, Trigger,
        With, Without, World,
    };
    use bevy_rand::{
        seed::RngSeed,
        traits::{ForkableInnerSeed, ForkableRng},
    };

    let seed = [2; 8];

    #[derive(Component)]
    struct SeedChildren(Vec<Entity>);

    #[derive(Event)]
    struct ReseedChildren;

    #[derive(Event)]
    struct Reseed([u8; 8]);

    fn reseed(trigger: Trigger<Reseed>, q_children: Query<&SeedChildren>, mut commands: Commands) {
        let entity = trigger.entity();

        if let Some(mut entity_commands) = commands.get_entity(trigger.entity()) {
            let seed = trigger.event();
            entity_commands.insert(RngSeed::<WyRand>::from_seed(seed.0));

            if q_children.contains(entity) {
                commands.trigger_targets(ReseedChildren, entity);
            }
        }
    }

    fn reseed_children(
        trigger: Trigger<ReseedChildren>,
        mut q_source: Query<(&mut EntropyComponent<WyRand>, &SeedChildren)>,
        mut commands: Commands,
    ) {
        let entity = trigger.entity();

        if let Ok((mut rng, children)) = q_source.get_mut(entity) {
            for child in children.0.iter() {
                commands.entity(*child).insert(rng.fork_seed());
            }
        }
    }

    let mut app = App::new();

    app.add_plugins(EntropyPlugin::<WyRand>::with_seed(seed))
        .add_systems(
            Startup,
            |mut commands: Commands, mut source: ResMut<GlobalEntropy<WyRand>>| {
                let mut source = commands.spawn(source.fork_seed());

                source.add(|mut entity: EntityWorldMut| {
                    let mut rng = entity
                        .get_mut::<EntropyComponent<WyRand>>()
                        .unwrap()
                        .fork_rng();

                    let children: Vec<Entity> = entity.world_scope(move |world: &mut World| {
                        world.spawn_batch((0..5).map(|_| rng.fork_seed())).collect()
                    });

                    entity.insert(SeedChildren(children));
                });
            },
        )
        .add_systems(PreUpdate, |query: Query<&RngSeed<WyRand>>| {
            let expected = [
                9035371013317154993u64,
                8695044747327652655,
                4791951605491714159,
                7661732659691953580,
                4722119124111390177,
            ];
            let seeds = query.iter().map(RngSeed::<WyRand>::clone_seed);

            expected
                .into_iter()
                .zip(seeds.map(u64::from_ne_bytes))
                .for_each(|(expected, actual)| assert_eq!(expected, actual));
        })
        .add_systems(
            Update,
            |mut commands: Commands,
             query: Query<Entity, With<SeedChildren>>,
             mut source: ResMut<GlobalEntropy<WyRand>>| {
                for e in &query {
                    commands.trigger_targets(Reseed(source.fork_inner_seed()), e);
                }
            },
        )
        .add_systems(PostUpdate, |query: Query<&RngSeed<WyRand>>| {
            let prev_expected = [
                9035371013317154993u64,
                8695044747327652655,
                4791951605491714159,
                7661732659691953580,
                4722119124111390177,
            ];
            let seeds = query.iter().map(RngSeed::<WyRand>::clone_seed);

            prev_expected
                .into_iter()
                .zip(seeds.map(u64::from_ne_bytes))
                .for_each(|(expected, actual)| assert_ne!(expected, actual));
        })
        .add_systems(
            Last,
            |source: Query<&EntropyComponent<WyRand>, With<SeedChildren>>,
             children: Query<&EntropyComponent<WyRand>, Without<SeedChildren>>| {
                assert_eq!(source.iter().size_hint().0, 1);
                assert_eq!(children.iter().size_hint().0, 5);
            },
        )
        .observe(reseed)
        .observe(reseed_children);

    app.run();
}
