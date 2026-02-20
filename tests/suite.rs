#![allow(clippy::type_complexity)]
#[path = "integration/bevy_math.rs"]
pub mod bevy_math;
#[path = "integration/determinism.rs"]
pub mod determinism;
#[path = "integration/extension.rs"]
pub mod extension;
#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
#[path = "integration/reflection.rs"]
pub mod reflection;
#[path = "integration/reseeding.rs"]
pub mod reseeding;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
