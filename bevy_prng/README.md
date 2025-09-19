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
- **`std`** - This enables some `std` specific functionality in some PRNGs, particularly in `rand_chacha`. Only for `std` environments.
- **`thread_local_entropy`** - Enables `ThreadLocalEntropy`, overriding `SeedableRng::from_entropy` implementations to make use of thread local entropy sources for faster PRNG initialisation. Requires `std` environments so it enables the `std` feature.
- **`rand_chacha`** - This enables the exporting of `ChaCha*Rng` components, for those that want/need to use a CSPRNG level source.
- **`rand_pcg`** - This enables the exporting of `Pcg*` components from `rand_pcg`.
- **`rand_xoshiro`** - This enables the exporting of `Xoshiro*` components from `rand_xoshiro`. It also exports a remote-reflected version of `Seed512` so to allow setting up `Xoshiro512StarStar` and so forth.
- **`wyrand`** - This enables the exporting of the `WyRand` component from `wyrand`, the same algorithm in use within `fastrand`/`turborand`.
- **`compat`** - This enables the old `RngCore` trait implementations on the RNGs, providing additional compatibility with other crates that haven't yet upgraded to the latest `rand_core`/`rand` versions.

In addition to these feature flags to enable various supported algorithms, there's also **`serialize`** flag to provide `serde` support for `Serialize`/`Deserialize`.

All types are provided at the top-level of the module:

```rust ignore
use bevy_prng::*;
```

## Supported PRNG Algorithms/Crates

All the below crates implement the necessary traits to be compatible with `bevy_prng`. Additional PRNG crates can be added via PR's to this crate/repo, provided the PRNGs implement `Debug`, `Clone`, `PartialEq` and have optional `Serialize`/`Deserialize` `serde` traits implemented and put behind appropriate feature flags.

### Cryptographically Secure PRNGs

- [rand_chacha](https://crates.io/crates/rand_chacha)

### Non-Cryptographically Secure PRNGS

- [wyrand](https://crates.io/crates/wyrand)
- [rand_xoshiro](https://crates.io/crates/rand_xoshiro)
- [rand_pcg](https://crates.io/crates/rand_pcg)

## Supported Versions & MSRV

`bevy_prng` uses the same MSRV as `bevy`.

| `bevy` | `bevy_prng`   |
| ------ | ------------- |
| v0.17  | v0.12         |
| v0.16  | v0.10 - v0.11 |
| v0.15  | v0.8 - v0.9   |
| v0.14  | v0.7 - v0.8   |
| v0.13  | v0.5 - v0.6   |
| v0.12  | v0.2          |
| v0.11  | v0.1          |

The versions of `rand_core`/`rand` that `bevy_prng` is compatible with is as follows:

| `bevy_prng`    | `rand_core` | `rand` | `getrandom` | `compat` feature               |
| -------------- | ----------- | ------ | ----------- | ------------------------------ |
| v0.10 -> v0.12 | v0.9        | v0.9   | v0.3        | ✅ (supports `rand_core` v0.6) |
| v0.1 -> v0.9   | v0.6        | v0.8   | v0.2        | ❌                             |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
