#![allow(clippy::type_complexity)]
#[path = "integration/determinism.rs"]
pub mod determinism;
#[path = "integration/reseeding.rs"]
pub mod reseeding;
#[cfg(feature = "compat")]
#[path = "integration/math.rs"]
pub mod math;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
