use bevy_math::{primitives::Circle, ShapeSample, Vec2};
use bevy_prng::WyRand;
use rand::SeedableRng;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn bevy_math_compatibility() {
    let mut source = WyRand::from_seed(42u64.to_ne_bytes());

    let circle = Circle::new(42.0);

    let boundary = circle.sample_boundary(&mut source);

    let interior = circle.sample_interior(&mut source);

    assert_eq!(&boundary, &Vec2::new(-40.885902, 9.609526));
    assert_eq!(&interior, &Vec2::new(-15.362211, 32.41336));
}
