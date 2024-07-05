#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_prng::{ChaCha12Rng, ChaCha8Rng, SeedableEntropySource, WyRand};
use bevy_rand::prelude::{
    EntropyComponent, EntropyPlugin, ForkableAsRng, ForkableRng, GlobalEntropy, GlobalRngSeed,
};
use rand::prelude::{Rng, SeedableRng};

use rand_core::RngCore;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Component)]
struct SourceA;

#[derive(Component)]
struct SourceB;

#[derive(Component)]
struct SourceC;

#[derive(Component)]
struct SourceD;

#[derive(Component)]
struct SourceE;

fn random_output_a(mut q_source: Query<&mut EntropyComponent<ChaCha8Rng>, With<SourceA>>) {
    let mut rng = q_source.single_mut();

    assert_eq!(
        rng.gen::<u32>(),
        3315785188,
        "SourceA does not match expected output"
    );
}

fn random_output_b(mut q_source: Query<&mut EntropyComponent<ChaCha8Rng>, With<SourceB>>) {
    let mut rng = q_source.single_mut();

    assert!(rng.gen_bool(0.5), "SourceB does not match expected output");
}

fn random_output_c(mut q_source: Query<&mut EntropyComponent<ChaCha8Rng>, With<SourceC>>) {
    let mut rng = q_source.single_mut();

    assert_eq!(
        rng.gen_range(0u32..=20u32),
        4,
        "SourceC does not match expected output"
    );
}

fn random_output_d(mut q_source: Query<&mut EntropyComponent<ChaCha12Rng>, With<SourceD>>) {
    let mut rng = q_source.single_mut();

    assert_eq!(
        rng.gen::<(u16, u16)>(),
        (41421, 7891),
        "SourceD does not match expected output"
    );
}

fn random_output_e(mut q_source: Query<&mut EntropyComponent<WyRand>, With<SourceE>>) {
    let mut rng = q_source.single_mut();

    let mut bytes = [0u8; 8];

    rng.fill_bytes(bytes.as_mut());

    assert_eq!(
        &bytes,
        &[42, 244, 101, 178, 244, 252, 72, 104],
        "SourceE does not match expected output"
    );
}

fn setup_sources(mut commands: Commands, mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    commands.spawn((SourceA, rng.fork_rng()));

    commands.spawn((SourceB, rng.fork_rng()));

    commands.spawn((SourceC, rng.fork_rng()));

    commands.spawn((SourceD, rng.fork_as::<ChaCha12Rng>()));

    commands.spawn((SourceE, rng.fork_as::<WyRand>()));
}

fn read_global_seed(seed: Res<GlobalRngSeed<ChaCha8Rng>>) {
    assert_eq!(seed.get_seed(), [2; 32]);
}

/// Entities having their own sources side-steps issues with parallel execution and scheduling
/// not ensuring that certain systems run before others. With an entity having its own RNG source,
/// no matter when the systems that query that entity run, it will always result in a deterministic
/// output. The order of execution will not affect the RNG output, as long as the entities are
/// seeded deterministically and any systems that query a specific entity or group of entities that
/// share the same RNG source are assured to be in order.
///
/// As an added bonus, this also ensures determinism even when systems are run in single-threaded
/// environments such as WASM.
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_parallel_determinism() {
    let mut app = App::new();

    #[cfg(not(target_arch = "wasm32"))]
    app.edit_schedule(Update, |schedule| {
        use bevy::ecs::schedule::ExecutorKind;

        // Ensure the Update schedule is Multithreaded on non-WASM platforms
        schedule.set_executor_kind(ExecutorKind::MultiThreaded);
    });

    app.add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed([2; 32]))
        .add_systems(Startup, setup_sources)
        .add_systems(
            Update,
            (
                random_output_a,
                random_output_b,
                random_output_c,
                random_output_d,
                random_output_e,
                read_global_seed,
            ),
        )
        .run();
}

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
            rng.reseed(seed.get_seed());
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
