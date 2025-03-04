# Bevy Rand

[![Crates.io](https://img.shields.io/crates/v/bevy_rand.svg)](https://crates.io/crates/bevy_rand)
[![CI](https://github.com/Bluefinger/bevy_rand/actions/workflows/ci.yml/badge.svg)](https://github.com/Bluefinger/bevy_rand/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/Bluefinger/bevy_rand)
[![Documentation](https://docs.rs/bevy_rand/badge.svg)](https://docs.rs/bevy_rand)

## What is Bevy Rand?

Bevy Rand is a plugin to provide integration of `rand` ecosystem PRNGs in an ECS friendly way. It provides a set of wrapper component and resource types that allow for safe access to a PRNG for generating random numbers, giving features like reflection, serialization for free. And with these types, it becomes possible to have determinism with the usage of these integrated PRNGs in ways that work with multi-threading and also avoid pitfalls such as unstable query iteration order.

## Using Bevy Rand

> There's now a tutorial, [go to here](https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/index.html) if you want a more comprehensive rundown of how to use `bevy_rand`.

Usage of Bevy Rand can range from very simple to quite complex use-cases, all depending on whether one cares about deterministic output or not. First, add `bevy_rand`, and either `rand_core` or `rand` to your `Cargo.toml` to bring in both the components and the PRNGs you want to use, along with the various traits needed to use the RNGs. To select a given algorithm type with `bevy_rand`, enable the feature representing the algorithm `rand_*` crate you want to use. This will then give you access to the PRNG structs via the prelude. Alternatively, you can use `bevy_prng` directly to get the newtyped structs with the same feature flags. However, using the algorithm crates like `rand_chacha` directly will not work as these don't implement the necessary traits to support bevy's reflection.

All supported PRNGs and compatible structs are provided by the `bevy_prng` crate. Simply activate the relevant features in `bevy_rand`/`bevy_prng` to pull in the PRNG algorithm you want to use, and then import them like so:

#### `bevy_rand` feature activation
```toml
rand_core = "0.6"
bevy_rand = { version = "0.10", features = ["rand_chacha", "wyrand"] }
```

#### `bevy_prng` feature activation
```toml
rand_core = "0.6"
bevy_rand = "0.10"
bevy_prng = { version = "0.10", features = ["rand_chacha", "wyrand"] }
```

The summary of what RNG algorithm to choose is: pick `wyrand` for almost all cases as it is faster and more portable than other algorithms. For cases where you need the extra assurance of entropy quality (as in, better and much less predictable 'randomness', etc), then use `rand_chacha`. For more information, [go here](https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch01_choosing_prng/index.html).

DO **NOT** use `bevy_rand` for actual security purposes, as this requires much more careful consideration and properly vetted crates designed for cryptography. A good starting point would be to look at [RustCrypto](https://github.com/RustCrypto) and go from there.

#### `no_std` support

`bevy_rand` is `no_std` compatible, but it requires disabling default features. It also assumes that `alloc` is available, just the same as `bevy`. Certain features like `thread_local_entropy` are not available for `no_std` due to requiring `std` specific functionalities like thread locals.

```toml
bevy_rand = { version = "0.10", default-features = false, features = ["rand_chacha", "wyrand"] }
```

All PRNG backends should support `no_std` environments. Furthermore, `getrandom` needs to be configured to support the platform, so in the case of a `no_std` environment such as an embedded board or console, you'll need to implement the [custom backend for `getrandom` to compile](https://docs.rs/getrandom/latest/getrandom/#custom-backend).

#### Usage within Web WASM environments

From `v0.9`, `bevy_rand` will no longer assume that `bevy` will be run in a web environment when compiled for WASM. To enable that, just paste the following into your `Cargo.toml` for your binary crate:

```toml
[target.'cfg(all(target_family = "wasm", any(target_os = "unknown", target_os = "none")))'.dependencies]
bevy_rand = { version = "0.10", features = ["wasm_js"] }
```

This enables the `wasm_js` backend to be made available for `getrandom`, but it doesn't actually build. The next step is to either edit your `.cargo/config.toml` with the below configuration:

```toml
# It's recommended to set the flag on a per-target basis:
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```

Or pass an environment variable: `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`. This then enables the `getrandom` WASM backend to get built correctly.

### Registering a PRNG for use with Bevy Rand

Before a PRNG can be used via `GlobalEntropy` or `Entropy`, it must be registered via the plugin.

```rust
use bevy_ecs::prelude::*;
use bevy_app::App;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use rand_core::RngCore;

fn example_main() {
    App::new()
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .run();
}
```

### Basic Usage

At the simplest case, using `GlobalEntropy` directly for all random number generation, though this does limit how well systems using `GlobalEntropy` can be parallelised. All systems that access `GlobalEntropy` will run serially to each other.

```rust
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

fn print_random_value(mut rng: GlobalEntropy<WyRand>) {
    println!("Random value: {}", rng.next_u32());
}
```

### Forking RNGs

For seeding `Entropy`s from a global source, it is best to make use of forking instead of generating the seed value directly. `GlobalEntropy` can only exist as a singular instance, so when forking normally, it will always fork as `Entropy` instances.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{GlobalEntropy, ForkableRng};

#[derive(Component)]
struct Source;

fn setup_source(mut commands: Commands, mut global: GlobalEntropy<WyRand>) {
    commands
        .spawn((
            Source,
            global.fork_rng(),
        ));
}
```

`Entropy`s can be seeded/forked from other `Entropy`s as well.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{Entropy, ForkableRng};

#[derive(Component)]
struct Npc;

#[derive(Component)]
struct Source;

fn setup_npc_from_source(
   mut commands: Commands,
   mut q_source: Single<&mut Entropy<WyRand>, (With<Source>, Without<Npc>)>,
) {
   let mut source = q_source.into_inner();
   for _ in 0..2 {
       commands
           .spawn((
               Npc,
               source.fork_rng()
           ));
   }
}
```

## Features

- **`std`** - Enables support for `std` environment, allows enabling `std` specific optimisations for `rand_chacha` and more. Enabled by default.
- **`thread_local_entropy`** - Enables `ThreadLocalEntropy`, overriding `SeedableRng::from_entropy` implementations to make use of thread local entropy sources for faster PRNG initialisation. Requires `std` environments so it enables the `std` feature. Enabled by default.
- **`serialize`** - Enables `Serialize` and `Deserialize` derives. Enabled by default.
- **`rand_chacha`** - This enables the exporting of newtyped `ChaCha*Rng` structs, for those that want/need to use a CSPRNG level source.
- **`rand_pcg`** - This enables the exporting of newtyped `Pcg*` structs from `rand_pcg`.
- **`rand_xoshiro`** - This enables the exporting of newtyped `Xoshiro*` structs from `rand_xoshiro`. It also exports a remote-reflected version of `Seed512` so to allow setting up `Xoshiro512StarStar` and so forth.
- **`wyrand`** - This enables the exporting of newtyped `WyRand` from `wyrand`, the same algorithm in use within `fastrand`/`turborand`.
- **`experimental`** - This enables any unstable/experimental features for `bevy_rand`. Currently, this does nothing at the moment.
- **`wasm_js`** - This enables the `getrandom` WASM backend, though doesn't make `getrandom` use it. That requires extra steps outlined [here](#usage-within-web-wasm-environments).
- **`compat`** - This enables the old v0.6 `RngCore` trait implementation on the RNGs, providing additional compatibility with other crates that haven't yet upgraded to the latest `rand_core`/`rand` versions.

## Supported Versions & MSRV

`bevy_rand` uses the same MSRV as `bevy`.

| `bevy` | `bevy_rand`  |
| ------ | ------------ |
| main   | v0.10 (main) |
| v0.15  | v0.8 - v0.9  |
| v0.14  | v0.7         |
| v0.13  | v0.5 - v0.6  |
| v0.12  | v0.4         |
| v0.11  | v0.2 - v0.3  |
| v0.10  | v0.1         |

The versions of `rand_core`/`rand` that `bevy_rand` is compatible with is as follows:

| `bevy_rand`   | `rand_core` | `rand` | `getrandom` | `compat` feature               |
| ------------- | ----------- | ------ | ----------- | ------------------------------ |
| v0.10         | v0.9        | v0.9   | v0.3        | ✅ (supports `rand_core` v0.6) |
| v0.1 -> v0.9  | v0.6        | v0.8   | v0.2        | ❌                             |

## Migrations

Notes on migrating between versions can be found [here](MIGRATIONS.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
