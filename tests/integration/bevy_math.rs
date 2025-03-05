use bevy_app::{App, Update};
use bevy_math::{ShapeSample, Vec2, primitives::Circle};
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalEntropy, plugin::EntropyPlugin};
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

    assert_eq!(&boundary, &Vec2::new(-40.885902, 9.609526));
    assert_eq!(&interior, &Vec2::new(-15.362211, 32.41336));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn component_compatibility() {
    App::new()
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(42u64.to_ne_bytes()))
        .add_systems(Update, |mut source: GlobalEntropy<WyRand>| {
            let circle = Circle::new(42.0);

            let boundary = circle.sample_boundary(source.as_mut());
            let interior = circle.sample_interior(source.as_mut());

            assert_eq!(&boundary, &Vec2::new(-40.885902, 9.609526));
            assert_eq!(&interior, &Vec2::new(-15.362211, 32.41336));
        })
        .run();
}
