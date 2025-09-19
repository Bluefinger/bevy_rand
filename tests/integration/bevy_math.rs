use bevy_app::{App, Update};
use bevy_ecs::{query::With, system::Single};
use bevy_math::{ShapeSample, Vec2, primitives::Circle};
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalRng, plugin::EntropyPlugin};
use rand::SeedableRng;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn prng_compatibility() {
    let mut source = WyRand::from_seed(42u64.to_ne_bytes());

    let circle = Circle::new(42.0);

    let boundary = circle.sample_boundary(&mut source);
    let interior = circle.sample_interior(&mut source);

    // Reduce the precision of the check to account for Miri's randomness, which simulates
    // the undefined precision of some float operations
    assert!(boundary.abs_diff_eq(Vec2::new(-40.885902, 9.609526), f32::EPSILON * 1000.0));
    assert!(interior.abs_diff_eq(Vec2::new(-15.362211, 32.41336), f32::EPSILON * 1000.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn component_compatibility() {
    App::new()
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(42u64.to_ne_bytes()))
        .add_systems(
            Update,
            |mut source: Single<&mut WyRand, With<GlobalRng>>| {
                let circle = Circle::new(42.0);

                let boundary = circle.sample_boundary(source.as_mut());
                let interior = circle.sample_interior(source.as_mut());

                // Reduce the precision of the check to account for Miri's randomness, which simulates
                // the undefined precision of some float operations
                assert!(
                    boundary.abs_diff_eq(Vec2::new(-40.885902, 9.609526), f32::EPSILON * 1000.0)
                );
                assert!(
                    interior.abs_diff_eq(Vec2::new(-15.362211, 32.41336), f32::EPSILON * 1000.0)
                );
            },
        )
        .run();
}
