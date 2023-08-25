#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand::prelude::Rng;

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

fn random_output_d(mut q_source: Query<&mut EntropyComponent<ChaCha8Rng>, With<SourceD>>) {
    let mut rng = q_source.single_mut();

    assert_eq!(
        rng.gen::<(u16, u16)>(),
        (61569, 26940),
        "SourceD does not match expected output"
    );
}

fn setup_sources(mut commands: Commands, mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    commands.spawn((SourceA, EntropyComponent::from(&mut rng)));

    commands.spawn((SourceB, EntropyComponent::from(&mut rng)));

    commands.spawn((SourceC, EntropyComponent::from(&mut rng)));

    commands.spawn((SourceD, EntropyComponent::from(&mut rng)));
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
    App::new()
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed([2; 32]))
        .add_systems(Startup, setup_sources)
        .add_systems(
            Update,
            (
                random_output_a,
                random_output_b,
                random_output_c,
                random_output_d,
            ),
        )
        .run();
}
