use bevy::{
    app::{App, PreStartup, PreUpdate, Update},
    prelude::{Commands, DetectChanges, Query, Res, ResMut},
};
use bevy_prng::{ChaCha8Rng, SeedableEntropySource, WyRand};
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::EntropyComponent,
    resource::GlobalEntropy,
    seed::GlobalRngSeed,
    traits::{ForkableAsSeed, ForkableSeed, SeedSource},
};
use rand_core::{RngCore, SeedableRng};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_global_reseeding() {
    /// Basic Reseeding mechanism by change detection against GlobalRngSeed
    fn reseed_global_rng<R: SeedableEntropySource>(
        seed: Res<GlobalRngSeed<R>>,
        mut rng: ResMut<GlobalEntropy<R>>,
    ) where
        R::Seed: Sync + Send + Clone,
    {
        if seed.is_changed() && !seed.is_added() {
            rng.reseed(seed.clone_seed());
        }
    }

    let mut app = App::new();

    let seed = [2; 32];

    let rng_eq = GlobalEntropy::<ChaCha8Rng>::from_seed(seed);

    app.add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed(seed))
        .add_systems(PreUpdate, reseed_global_rng::<ChaCha8Rng>);

    {
        let global_rng = app.world().resource_ref::<GlobalEntropy<ChaCha8Rng>>();
        let global_seed = app.world().resource_ref::<GlobalRngSeed<ChaCha8Rng>>();

        // Our RNGs should be the same as each other as they were initialised with the same seed
        assert_eq!(global_rng.as_ref(), &rng_eq);

        // The condition here should mean our reseeding system will NOT run
        assert!(global_seed.is_changed() && global_seed.is_added());
    }

    app.update();

    {
        let global_rng = app.world().resource_ref::<GlobalEntropy<ChaCha8Rng>>();

        // Our RNGs should remain the same as each other as we have not run the update
        assert_eq!(global_rng.as_ref(), &rng_eq);
    }

    {
        let mut global_seed = app.world_mut().resource_mut::<GlobalRngSeed<ChaCha8Rng>>();

        global_seed.set_seed([3; 32]);

        // The condition here should mean our reseeding system WILL run
        assert!(global_seed.is_changed() && !global_seed.is_added());
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
