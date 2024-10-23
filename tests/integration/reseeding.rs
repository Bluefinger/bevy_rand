use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_prng::{ChaCha8Rng, WyRand};
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::EntropyComponent,
    resource::GlobalEntropy,
    seed::RngSeed,
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
    use bevy_app::prelude::{PostUpdate, PreUpdate, Startup};
    use bevy_ecs::prelude::{Entity, Event, Trigger, With};
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
        .add_observer(reseed);

    app.run();
}

#[test]
#[cfg(feature = "experimental")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn generic_observer_reseeding_from_parent() {
    use bevy_app::prelude::{PostUpdate, PreUpdate, Startup};
    use bevy_ecs::prelude::{Entity, With};
    use bevy_rand::{
        observers::{LinkRngSourceToTarget, SeedFromGlobal, SeedFromParent},
        plugin::LinkedEntropySources,
    };

    let seed = [2u8; 8];

    #[derive(Component)]
    struct Source;
    #[derive(Component)]
    struct Target;

    let mut app = App::new();

    app.add_plugins((
        EntropyPlugin::<WyRand>::with_seed(seed),
        LinkedEntropySources::<Target, WyRand>::default(),
    ))
    .add_systems(Startup, |mut commands: Commands| {
        let source = commands.spawn(Source).id();
        commands.spawn(Target);

        commands.trigger_targets(LinkRngSourceToTarget::<Target, WyRand>::default(), source);
        commands.trigger_targets(SeedFromGlobal::<WyRand>::default(), source);
    })
    .add_systems(PreUpdate, |query: Query<&RngSeed<WyRand>, With<Target>>| {
        let expected = 6445550333322662121;
        let seed = u64::from_ne_bytes(query.single().clone_seed());

        assert_eq!(seed, expected);
    })
    .add_systems(PreUpdate, |query: Query<&RngSeed<WyRand>, With<Source>>| {
        let expected = 2484862625678185386;
        let seed = u64::from_ne_bytes(query.single().clone_seed());

        assert_eq!(seed, expected);
    })
    .add_systems(
        Update,
        |mut commands: Commands, query: Query<Entity, With<Target>>| {
            commands.trigger_targets(SeedFromParent::<WyRand>::default(), query.single());
        },
    )
    .add_systems(
        PostUpdate,
        |query: Query<&RngSeed<WyRand>, With<Target>>| {
            let prev_expected = 6445550333322662121;
            let expected = 14968821102299026759;
            let seed = u64::from_ne_bytes(query.single().clone_seed());

            assert_ne!(seed, prev_expected);
            assert_eq!(seed, expected);
        },
    );

    app.run();
}

#[test]
#[cfg(feature = "experimental")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn generic_observer_reseeding_children() {
    use bevy_app::prelude::{Last, PostUpdate, PreUpdate, Startup};
    use bevy_ecs::prelude::{Component, Entity, With, Without};
    use bevy_rand::{
        observers::{LinkRngSourceToTarget, SeedFromGlobal},
        plugin::LinkedEntropySources,
    };

    let seed = [2u8; 8];

    #[derive(Component)]
    struct Source;
    #[derive(Component, Clone, Copy)]
    struct Target;

    let mut app = App::new();

    app.add_plugins((
        EntropyPlugin::<WyRand>::with_seed(seed),
        LinkedEntropySources::<Target, WyRand>::default(),
    ))
    .add_systems(Startup, |mut commands: Commands| {
        commands.spawn_batch(vec![Target; 5]);
        let source = commands.spawn(Source).id();

        commands.trigger_targets(LinkRngSourceToTarget::<Target, WyRand>::default(), source);
        commands.trigger_targets(SeedFromGlobal::<WyRand>::default(), source);
    })
    .add_systems(PreUpdate, |query: Query<&RngSeed<WyRand>, With<Target>>| {
        let expected = [
            6445550333322662121u64,
            14968821102299026759,
            12617564484450995185,
            908888629357954483,
            6128439264405451235,
        ];
        let seeds = query.iter().map(RngSeed::<WyRand>::clone_seed);

        assert_eq!(seeds.size_hint().0, 5);

        expected
            .into_iter()
            .zip(seeds.map(u64::from_ne_bytes))
            .for_each(|(expected, actual)| assert_eq!(expected, actual));
    })
    .add_systems(PreUpdate, |query: Query<&RngSeed<WyRand>, With<Source>>| {
        let expected = 2484862625678185386u64;
        let seeds = u64::from_ne_bytes(query.single().clone_seed());

        assert_eq!(expected, seeds);
    })
    .add_systems(
        Update,
        |mut commands: Commands, query: Query<Entity, With<Source>>| {
            for entity in &query {
                commands.trigger_targets(SeedFromGlobal::<WyRand>::default(), entity);
            }
        },
    )
    .add_systems(
        PostUpdate,
        |query: Query<&RngSeed<WyRand>, With<Target>>| {
            let prev_expected = [
                6445550333322662121u64,
                14968821102299026759,
                12617564484450995185,
                908888629357954483,
                6128439264405451235,
            ];
            let expected = [
                2656876351602726802u64,
                4226413670151402273,
                2344778986622729714,
                9109365740673988404,
                6101264679293753504,
            ];

            prev_expected
                .into_iter()
                .zip(expected)
                .zip(
                    query
                        .iter()
                        .map(RngSeed::<WyRand>::clone_seed)
                        .map(u64::from_ne_bytes),
                )
                .for_each(|((previous, expected), actual)| {
                    // Must not equal the previous seeds.
                    assert_ne!(previous, actual);
                    // Should equal the expected updated seeds.
                    assert_eq!(expected, actual)
                });
        },
    )
    .add_systems(
        Last,
        |source: Query<&RngSeed<WyRand>, With<Source>>,
         children: Query<&RngSeed<WyRand>, Without<Source>>| {
            // Check we have the correct amount of allocated RNG entities
            assert_eq!(source.iter().size_hint().0, 1);
            assert_eq!(children.iter().size_hint().0, 5);
        },
    );

    app.run();
}
