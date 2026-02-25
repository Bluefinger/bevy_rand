# Bevy PRNG

[![Crates.io](https://img.shields.io/crates/v/bevy_prng.svg)](https://crates.io/crates/bevy_prng)
[![CI](https://github.com/Bluefinger/bevy_rand/actions/workflows/ci.yml/badge.svg)](https://github.com/Bluefinger/bevy_rand/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/Bluefinger/bevy_rand)
[![Documentation](https://docs.rs/bevy_prng/badge.svg)](https://docs.rs/bevy_prng)

## What is Bevy PRNG?

`bevy_prng` is a crate that provides componentised versions of various `rand_*` PRNG algorithm crates to make them suitable for integration within `bevy` for reflection purposes. It enables these types to have stable `TypePath`s and otherwise implement various required traits. This crate can be used as standalone to provide access to various PRNG algorithms of one's choice, to then use components for one's game in `bevy`, but primarily, it's purpose to support and be a counterpart to `bevy_rand` (which provides the utilities that `bevy_prng` types can plug in to).

This crate is `no_std` compatible.

## Using Bevy PRNG

By default, `bevy_prng` won't export anything _unless_ the feature/algorithm you require is explicitly defined. In order to gain access to a PRNG component, you'll have activate one of the following features:

- **`bevy_reflect`** - Enables reflection support for all `bevy_prng` types.
- **`std`** - This enables some `std` specific functionality. Only for `std` environments.
- **`thread_local_entropy`** - Enables `ThreadLocalEntropy`, overriding `SeedableRng::from_entropy` implementations to make use of thread local entropy sources for faster PRNG initialisation. Requires `std` environments so it enables the `std` feature.
- **`chacha20`** - This enables the exporting of `ChaCha*Rng` components, for those that want/need to use a CSPRNG level source.
- **`rand_pcg`** - This enables the exporting of `Pcg*` components from `rand_pcg`.
- **`rand_xoshiro`** - This enables the exporting of `Xoshiro*` components from `rand_xoshiro`. It also exports a remote-reflected version of `Seed512` so to allow setting up `Xoshiro512StarStar` and so forth.
- **`wyrand`** - This enables the exporting of the `WyRand` component from `wyrand`, the same algorithm in use within `fastrand`/`turborand`.
- **`compat_06`** - This enables the old v0.6 `RngCore` trait implementation on the RNGs, providing additional compatibility with other crates that haven't yet upgraded to the latest `rand_core`/`rand` versions.
- **`compat_09`** - This enables the old v0.9 `RngCore` trait implementation on the RNGs, providing additional compatibility with other crates that haven't yet upgraded to the latest `rand_core`/`rand` versions.
- **`wasm_js`** - This enables the `getrandom` WASM JS backend, though this should only be activated conditionally for `wasm` targets. That requires extra steps outlined [here](#usage-within-web-wasm-environments).

In addition to these feature flags to enable various supported algorithms, there's also **`serialize`** flag to provide `serde` support for `Serialize`/`Deserialize`.

All types are provided at the top-level of the module:

```rust ignore
use bevy_prng::*;
```

## Supported PRNG Algorithms/Crates

All the below crates implement the necessary traits to be compatible with `bevy_prng`. Additional PRNG crates can be added via PR's to this crate/repo, provided the PRNGs implement `Debug`, `Clone`, `PartialEq` and have optional `Serialize`/`Deserialize` `serde` traits implemented and put behind appropriate feature flags.

### Cryptographically Secure PRNGs

- [chacha20](https://crates.io/crates/chacha20)

### Non-Cryptographically Secure PRNGS

- [wyrand](https://crates.io/crates/wyrand)
- [rand_xoshiro](https://crates.io/crates/rand_xoshiro)
- [rand_pcg](https://crates.io/crates/rand_pcg)

## Usage within Web WASM environments

To enable `bevy_prng` to work with Web WASM in `v0.14`, just paste the following into your `Cargo.toml` for your binary crate:

```toml
[target.'cfg(all(target_family = "wasm", any(target_os = "unknown", target_os = "none")))'.dependencies]
bevy_prng = { version = "0.14", features = ["wasm_js"] }
```

This enables the `wasm_js` backend to be made available for `getrandom`, which will allow `bevy_prng` to compile correctly for web WASM environments. The reason for this is that `wasm32-unknown-unknown` is itself not actually a web target, so to actually target a web environment, we must specify the feature in order to activate `wasm-bindgen` to do its thing.

If you have older versions of `getrandom` in your dep tree that are getting compiled in, then you might need to add further configuration to your `Cargo.toml` in order to enable Web WASM builds to compile correctly:

```toml
[target.'cfg(all(target_family = "wasm", any(target_os = "unknown", target_os = "none")))'.dependencies]
bevy_prng = { version = "0.14", features = ["wasm_js"] }
# Add the line below to make v0.3.4 getrandom work in Web WASM builds
getrandom_03 = { version = "0.3.4", features = ["wasm_js"], package = "getrandom" }
# Add the line below to make v0.2.17 getrandom work in Web WASM builds
getrandom_02 = { version = "0.2.17", features = ["js"], package = "getrandom" }
```

## Supported Versions & MSRV

`bevy_prng` uses the same MSRV as `bevy`.

| `bevy` | `bevy_prng`   |
| ------ | ------------- |
| v0.18  | v0.13 - v0.14 |
| v0.17  | v0.12         |
| v0.16  | v0.10 - v0.11 |
| v0.15  | v0.8 - v0.9   |
| v0.14  | v0.7 - v0.8   |
| v0.13  | v0.5 - v0.6   |
| v0.12  | v0.2          |
| v0.11  | v0.1          |

The versions of `rand_core`/`rand` that `bevy_prng` is compatible with is as follows:

| `bevy_prng`    | `rand_core` | `rand` | `getrandom` | `compat_*` features                  |
| -------------- | ----------- | ------ | ----------- | ------------------------------------ |
| v0.14          | v0.10       | v0.10  | v0.4        | ✅ (supports `rand_core` v0.6, v0.9) |
| v0.10 -> v0.13 | v0.9        | v0.9   | v0.3        | ✅ (supports `rand_core` v0.6)       |
| v0.1 -> v0.9   | v0.6        | v0.8   | v0.2        | ❌                                   |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
