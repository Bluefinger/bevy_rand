use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_prng::{FastRng, QualityRng};
use bevy_rand::prelude::{
    EntropyPlugin, ForkableAsRng, ForkableAsSeed, ForkableRng, ForkableSeed, GlobalRng,
    GlobalRngEntity,
};
use rand::RngExt;
use rand_core::Rng;

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

fn random_output_a(mut rng: Single<&mut QualityRng, With<SourceA>>) {
    assert_eq!(
        rng.random::<u32>(),
        3315785188,
        "SourceA does not match expected output"
    );
}

fn random_output_b(mut rng: Single<&mut QualityRng, With<SourceB>>) {
    assert!(
        rng.random_bool(0.5),
        "SourceB does not match expected output"
    );
}

fn random_output_c(mut rng: Single<&mut QualityRng, With<SourceC>>) {
    assert_eq!(
        rng.random_range(0u32..=20u32),
        4,
        "SourceC does not match expected output"
    );
}

fn random_output_d(mut rng: Single<&mut FastRng, With<SourceD>>) {
    assert_eq!(
        rng.random::<(u16, u16)>(),
        (5547, 31934),
        "SourceD does not match expected output"
    );
}

fn random_output_e(mut rng: Single<&mut FastRng, With<SourceE>>) {
    let mut bytes = [0u8; 8];

    rng.fill_bytes(bytes.as_mut());

    assert_eq!(
        &bytes,
        &[106, 106, 25, 30, 16, 159, 227, 197],
        "SourceE does not match expected output"
    );
}

fn setup_sources(mut commands: Commands, mut rng: Single<&mut QualityRng, With<GlobalRng>>) {
    commands.spawn((SourceA, rng.fork_rng()));

    commands.spawn((SourceB, rng.fork_seed()));

    commands.spawn((SourceC, rng.fork_rng()));

    commands.spawn((SourceD, rng.fork_as::<FastRng>()));

    commands.spawn((SourceE, rng.fork_as_seed::<FastRng>()));
}

fn read_global_seed(rng: GlobalRngEntity<QualityRng>) {
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

    app.add_plugins(EntropyPlugin::<QualityRng>::with_seed([2; 32]))
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
