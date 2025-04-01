#![allow(clippy::type_complexity)]
#[cfg(feature = "compat")]
#[path = "integration/bevy_math.rs"]
pub mod bevy_math;
#[path = "integration/determinism.rs"]
pub mod determinism;
#[path = "integration/reseeding.rs"]
pub mod reseeding;
#[path = "integration/extension.rs"]
pub mod extension;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
