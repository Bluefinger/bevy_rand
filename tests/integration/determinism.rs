use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_prng::{ChaCha12Rng, ChaCha8Rng, WyRand};
use bevy_rand::{
    global::GlobalRngEntity,
    prelude::{Entropy, EntropyPlugin, ForkableAsRng, ForkableRng, GlobalEntropy},
};
use rand::prelude::Rng;

use rand_core::RngCore;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

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

fn random_output_a(mut q_source: Query<&mut Entropy<ChaCha8Rng>, With<SourceA>>) {
    let mut rng = q_source.single_mut();

    assert_eq!(
        rng.gen::<u32>(),
        3315785188,
        "SourceA does not match expected output"
    );
}

fn random_output_b(mut q_source: Query<&mut Entropy<ChaCha8Rng>, With<SourceB>>) {
    let mut rng = q_source.single_mut();

    assert!(rng.gen_bool(0.5), "SourceB does not match expected output");
}

fn random_output_c(mut q_source: Query<&mut Entropy<ChaCha8Rng>, With<SourceC>>) {
    let mut rng = q_source.single_mut();

    assert_eq!(
        rng.gen_range(0u32..=20u32),
        4,
        "SourceC does not match expected output"
    );
}

fn random_output_d(mut q_source: Query<&mut Entropy<ChaCha12Rng>, With<SourceD>>) {
    let mut rng = q_source.single_mut();

    assert_eq!(
        rng.gen::<(u16, u16)>(),
        (41421, 7891),
        "SourceD does not match expected output"
    );
}

fn random_output_e(mut q_source: Query<&mut Entropy<WyRand>, With<SourceE>>) {
    let mut rng = q_source.single_mut();

    let mut bytes = [0u8; 8];

    rng.fill_bytes(bytes.as_mut());

    assert_eq!(
        &bytes,
        &[42, 244, 101, 178, 244, 252, 72, 104],
        "SourceE does not match expected output"
    );
}

fn setup_sources(mut commands: Commands, mut rng: GlobalEntropy<ChaCha8Rng>) {
    commands.spawn((SourceA, rng.fork_rng()));

    commands.spawn((SourceB, rng.fork_rng()));

    commands.spawn((SourceC, rng.fork_rng()));

    commands.spawn((SourceD, rng.fork_as::<ChaCha12Rng>()));

    commands.spawn((SourceE, rng.fork_as::<WyRand>()));
}

fn read_global_seed(rng: GlobalRngEntity<ChaCha8Rng>) {
    assert_eq!(rng.clone_seed(), [2; 32]);
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
        use bevy_ecs::schedule::ExecutorKind;

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
