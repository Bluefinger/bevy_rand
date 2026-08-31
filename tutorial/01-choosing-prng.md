# Selecting and using PRNG Algorithms

If you need a TL;DR of this section: If you don't care about what algorithm to pick, just stick with the default `fast_rng` from `bevy_rand`, else if you need something that can't have its state recovered from enough of its outputs (and thus be predicted), opt for `quality_rng`. If you are looking for specific algorithm types, then select `wyrand` for the fastest PRNG with good enough entropy, or select `chacha20` for the best quality entropy at the expense of speed (with `ChaCha8Rng` being the fastest of the three available ones, with `ChaCha20Rng` being the most secure). If you are working with weak/old hardware that will only be 32-bit, then `fast_rng32` fills this niche (or `rand_pcg` for `Pcg32`).

## Choosing a PRNG

All supported PRNGs and compatible structs are provided by the `bevy_prng` crate. Simply activate the relevant features in `bevy_rand`/`bevy_prng` to pull in the PRNG algorithm you want to use, and then import them like so:

```toml
bevy_rand = { version = "0.15", features = ["fast_rng", "quality_rng"] }
```
```rust ignore
use bevy::prelude::*;
use bevy_rand::prelude::{QualityRng, FastRng};
```
or
```toml
bevy_rand = "0.15"
bevy_prng = { version = "0.15", features = ["chacha20", "wyrand"] }
```
```rust ignore
use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_prng::{ChaCha8Rng, WyRand};
```

When you've selected and imported the PRNG algorithm you want to use, you then need to enable it by adding the `EntropyPlugin` to your app. This then makes it available for use with `GlobalRng` and entity sources, as well as registering the types for reflection.

```rust
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_prng::FastRng;
use bevy_rand::prelude::EntropyPlugin;
use rand_core::Rng;

fn main() {
    App::new()
        .add_plugins(EntropyPlugin::<FastRng>::default())
        .run();
}
```

By default, the plugin will instantiate a global `EntropySource` entity (accessible via `Single<&mut FastRng, With<GlobalRng>>`) with a random seed from OS sources. If you want to initialise the plugin and `GlobalRng` with a set seed or from a different source, use [`crate::prelude::EntropyPlugin::with_seed`] instead.

```rust
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_prng::FastRng;
use bevy_rand::prelude::EntropyPlugin;
use rand_core::Rng;

fn main() {
    let seed: u64 = 234; // How you source this is up to you.
    
    App::new()
        .add_plugins(EntropyPlugin::<FastRng>::with_seed(seed.to_ne_bytes()))
        .run();
}
```

## Supported Algorithms

The current set of PRNG algorithms that are supported out of the box in `bevy_prng` are as follows:

- `wyrand`: This provides newtyped `WyRand` from `wyrand`, the same algorithm in use within `fastrand`/`turborand`.
- `rand_xoshiro`: This provides newtyped `Xoshiro*` structs from `rand_xoshiro`. It also exports a remote-reflected version of `Seed512` to allow setting up `Xoshiro512StarStar` and so forth.
- `rand_pcg`: This provides newtyped `Pcg*` structs from `rand_pcg`.
- `chacha20`: This provides newtyped `ChaCha*Rng` structs, for those that want/need to use a CSPRNG level source.

`bevy_prng` also provides simplified structs in case you don't care/know much about the algorithms, which are described as follows:

- `fast_rng`: This is the default/recommended one. `FastRng` is the fastest and produces good random output.
- `fast_rng32`: `FastRng32` is a niche RNG that is better for 32-bit hardware, whether old(retro) or specialised for 32-bit ops. Only enable this if you need something that is optimised for 32-bit, as it is statistically weaker than `fast_rng`.
- `quality_rng`: `QualityRng` is a RNG that provides much stronger quality random output that cannot be predicted. With enough outputs, `FastRng` _could_ potentially have its state recovered and thus be predicted, but `QualityRng`'s outputs do not correlate in any way, nor can its state be recovered from the outputs. Use this when you need that extra assurance, but it is a lot slower than `FastRng`.

`bevy_rand` re-exports these simplified structs, whilst `bevy_prng` contains both the simplified RNGs and the algorithm ones.

## `bevy_prng` and newtypes

Trying to use PRNGs directly as resources/components from the `rand_*` crates is not possible without newtyping, and better to use `bevy_prng` instead of writing your own newtypes. Types from `rand` like `StdRng` and `SmallRng` are not encouraged to be used or newtyped as they are not **portable**, nor are they guaranteed to be stable. `SmallRng` is not even the same algorithm if used on 32-bit platforms versus 64-bit platforms. This makes them unsuitable for purposes of reflected/stable/portable types needed for deterministic PRNG. Likely, you'd be using `StdRng`/`SmallRng` directly without integration into Bevy's ECS. This is why the algorithm crates are used directly by `bevy_prng`, as this is the recommendation from `rand` when stability and portability is important.

## Factors for selecting a PRNG algorithm

As a whole, which algorithm should be used/selected is dependent on a range of factors. Cryptographically Secure PRNGs (CSPRNGs) produce very hard to predict output (very high quality entropy), but in general are slow. The ChaCha algorithm can be sped up by using versions with less rounds (iterations of the algorithm), but this in turn reduces the quality of the output (making it easier to predict), or by compiling with CPU features enabled such as SIMD (AVX2 support in particular). However, `ChaCha8Rng` is still far stronger than what is feasible to be attacked, and is considerably faster as a source of entropy than the full `ChaCha20Rng`. `rand` uses `ChaCha12Rng` as a balance between security/quality of output and speed for its `StdRng`. CSPRNGs are important for cases when you _really_ don't want your output to be predictable and you need that extra level of assurance, such as doing any cryptography/authentication/security tasks. Do note however, `rand` is not intended to be a cryptography crate, nor used for cryptography purposes, and that should be delegated towards crates designed for that purpose. And `bevy_rand` RNG types *do not* implement `CryptoRng` in order to ensure they aren't used for secure cryptography purposes. Use `getrandom`'s `SysRng` if you need that level of assurance and unpredictability. `QualityRng` uses `ChaCha8Rng` internally.

If that extra level of randomness is not necessary (which will be most cases within a game), but there is still a need for extra speed while maintaining good enough randomness, other PRNG algorithms exist for this purpose. These algorithms still try to output as high quality entropy as possible, but the level of entropy is not enough for cryptographic purposes. These algorithms should **never be used in situations that demand security**. Algorithms like `WyRand` and `Xoshiro256StarStar` are tuned for maximum throughput, while still possessing _good enough_ entropy for use as a source of randomness for non-security purposes. It still matters that the output is not predictable, but not to the same extent as CSPRNGs are required to be. PRNGs like `WyRand` also have small state sizes, which makes them take less memory per instance compared to CSPRNGs like `ChaCha8Rng`. `FastRng` uses `WyRand` internally, and `FastRng32` uses `Pcg32` (which is only optimal on old 32-bit hardware/platforms).
