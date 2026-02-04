use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::{
    plugin::EntropyPlugin,
    traits::{ForkRngExt, ForkSeedExt, SeedSource},
};

use rand::Rng;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn exclusive_system_forking() {
    let mut app = App::new();

    app.add_plugins(EntropyPlugin::<WyRand>::with_seed(42u64.to_ne_bytes()))
        .add_systems(Update, |mut world: &mut World| {
            let mut forked = world
                .fork_rng::<WyRand>()
                .expect("Forking should be successful");

            assert_eq!(forked.next_u32(), 2755170287);
        })
        .run();
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn exclusive_system_forking_seeds() {
    let mut app = App::new();

    app.add_plugins(EntropyPlugin::<WyRand>::with_seed(42u64.to_ne_bytes()))
        .add_systems(Update, |mut world: &mut World| {
            let forked = world
                .fork_seed::<WyRand>()
                .expect("Forking should be successful");

            assert_eq!(forked.get_seed(), &[137, 57, 152, 118, 124, 216, 113, 202]);

            let inner = world
                .fork_inner_seed::<WyRand>()
                .expect("Forking should be successful");

            // Forking should always yield new and different seeds
            assert_ne!(&inner, &[137, 57, 152, 118, 124, 216, 113, 202]);
            assert_ne!(forked.get_seed(), &inner);
        })
        .run();
}

#[test]
#[should_panic]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn exclusive_system_forking_returns_error_without_correct_setup() {
    let mut app = App::new();

    app.add_systems(Update, |mut world: &mut World| {
        let mut forked = world
            .fork_rng::<WyRand>()
            .expect("Forking should be successful");

        assert_eq!(forked.next_u32(), 2755170287);
    })
    .run();
}
