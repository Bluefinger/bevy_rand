# Bevy Rand

[![Crates.io](https://img.shields.io/crates/v/bevy_rand.svg)](https://crates.io/crates/bevy_rand)
[![CI](https://github.com/Bluefinger/bevy_rand/actions/workflows/ci.yml/badge.svg)](https://github.com/Bluefinger/bevy_rand/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/Bluefinger/bevy_rand)
[![Documentation](https://docs.rs/bevy_rand/badge.svg)](https://docs.rs/bevy_rand)

## What is Bevy Rand?

Bevy Rand is a plugin to provide integration of `rand` ecosystem PRNGs in an ECS friendly way. It provides a set of wrapper component and resource types that allow for safe access to a PRNG for generating random numbers, giving features like reflection, serialization for free. And with these types, it becomes possible to have determinism with the usage of these integrated PRNGs in ways that work with multi-threading and also avoid pitfalls such as unstable query iteration order.

## Overview

Games often use randomness as a core mechanic. For example, card games generate a random deck for each game and killing monsters in an RPG often rewards players with a random item. While randomness makes games more interesting and increases replayability, it also makes games harder to test and prevents advanced techniques such as [deterministic lockstep](https://gafferongames.com/post/deterministic_lockstep/).

Let's pretend you are creating a poker game where a human player can play against the computer. The computer's poker logic is very simple: when the computer has a good hand, it bets all of its money. To make sure the behavior works, you write a test to first check the computer's hand and if it is good confirm that all its money is bet. If the test passes does it ensure the computer behaves as intended? Sadly, no.

Because the deck is randomly shuffled for each game (without doing so the player would already know the card order from the previous game), it is not guaranteed that the computer player gets a good hand and thus the betting logic goes unchecked.
While there are ways around this (a fake deck that is not shuffled, running the test many times to increase confidence, breaking the logic into units and testing those) it would be very helpful to have randomness as well as a way to make it _less_ random.

Luckily, when a computer needs a random number it doesn't use real randomness and instead uses a [pseudorandom number generator](https://en.wikipedia.org/wiki/Pseudorandom_number_generator). Popular Rust libraries containing pseudorandom number generators are [`rand`](https://crates.io/crates/rand) and [`fastrand`](https://crates.io/crates/fastrand).

Pseudorandom number generators require a source of [entropy](https://en.wikipedia.org/wiki/Entropy) called a [random seed](https://en.wikipedia.org/wiki/Random_seed). The random seed is used as input to generate numbers that _appear_ random but are instead in a specific and deterministic order. For the same random seed, a pseudorandom number generator always returns the same numbers in the same order.

For example, let's say you seed a pseudorandom number generator with `1234`.
You then ask for a random number between `10` and `99` and the pseudorandom number generator returns `12`.
If you run the program again with the same seed (`1234`) and ask for another random number between `1` and `99`, you will again get `12`.
If you then change the seed to `4567` and run the program, more than likely the result will not be `12` and will instead be a different number.
If you run the program again with the `4567` seed, you should see the same number from the previous `4567`-seeded

There are many types of pseudorandom number generators each with their own strengths and weaknesses. Because of this, Bevy does not include a pseudorandom number generator. Instead, the `bevy_rand` plugin includes a source of entropy to use as a random seed for your chosen pseudorandom number generator.

Note that Bevy currently has [other sources of non-determinism](https://github.com/bevyengine/bevy/discussions/2480) unrelated to pseudorandom number generators.

## Concepts

Bevy Rand operates around a global entropy source provided as a resource, and then entropy components that can then be attached to entities. The use of resources/components allow the ECS to schedule systems appropriately so to make it easier to achieve determinism.

### GlobalEntropy

`GlobalEntropy` is the main resource for providing a global entropy source. It can only be accessed via a `ResMut` if looking to generate random numbers from it, as `RngCore` only exposes `&mut self` methods. As a result, working with `ResMut<GlobalEntropy<T>>` means any systems that access it will not be able to run in parallel to each other, as the `mut` access requires the scheduler to ensure that only one system at a time is accessing it. Therefore, if one intends on parallelising RNG workloads, limiting use/access of `GlobalEntropy` is vital. However, if one intends on having a single seed to deterministic control/derive many RNGs, `GlobalEntropy` is the best source for this purpose.

### EntropyComponent

`EntropyComponent` is a wrapper component that allows for entities to have their own RNG source. In order to generate random numbers from it, the `EntropyComponent` must be accessed with a `&mut` reference. Doing so will limit systems accessing the same source, but to increase parallelism, one can create many different sources instead. For ensuring determinism, query iteration must be accounted for as well as it isn't stable. Therefore, entities that need to perform some randomised task should 'own' their own `EntropyComponent`.

`EntropyComponent` can be seeded directly, or be created from a `GlobalEntropy` source or other `EntropyComponent`s.

### Forking

If cloning creates a second instance that shares the same state as the original, forking derives a new state from the original, leaving the original 'changed' and the new instance with a randomised seed. Forking RNG instances from a global source is a way to ensure that one seed produces many deterministic states, while making it difficult to predict outputs from many sources and also ensuring no one source shares the same state either with the original or with each other.

Bevy Rand approaches forking via `From` implementations of the various component/resource types, making it straightforward to use.

## Using Bevy Rand

Usage of Bevy Rand can range from very simple to quite complex use-cases, all depending on whether one cares about deterministic output or not. First, add `bevy_rand`,`bevy_prng`, and either `rand_core` or `rand` to your `Cargo.toml` to bring in both the components and the PRNGs you want to use, along with the various traits needed to use the RNGs. To select a given algorithm type with `bevy_prng`, enable the feature representing the newtypes from the `rand_*` crate you want to use.

```toml
rand_core = "0.6"
bevy_rand = "0.3"
bevy_prng = { version = "0.1", features = ["rand_chacha"] }
```

### Registering a PRNG for use with Bevy Rand

Before a PRNG can be used via `GlobalEntropy` or `EntropyComponent`, it must be registered via the plugin.

```rust
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;
use bevy_prng::ChaCha8Rng;

fn main() {
    App::new()
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .run();
}
```

### Basic Usage

At the simplest case, using `GlobalEntropy` directly for all random number generation, though this does limit how well systems using `GlobalEntropy` can be parallelised. All systems that access `GlobalEntropy` will run serially to each other.

```rust
use bevy::prelude::ResMut;
use bevy_rand::prelude::*;
use rand_core::RngCore;
use bevy_prng::ChaCha8Rng;

fn print_random_value(mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    println!("Random value: {}", rng.next_u32());
}
```

### Forking RNGs

For seeding `EntropyComponent`s from a global source, it is best to make use of forking instead of generating the seed value directly.

```rust
use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;

#[derive(Component)]
struct Source;

fn setup_source(mut commands: Commands, mut global: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    commands
        .spawn((
            Source,
            EntropyComponent::from(&mut global),
        ));
}
```

`EntropyComponent`s can be seeded/forked from other `EntropyComponent`s as well.

```rust
use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;

#[derive(Component)]
struct Npc;

#[derive(Component)]
struct Source;

fn setup_npc_from_source(
   mut commands: Commands,
   mut q_source: Query<&mut EntropyComponent<ChaCha8Rng>, (With<Source>, Without<Npc>)>,
) {
   let mut source = q_source.single_mut();
   for _ in 0..2 {
       commands
           .spawn((
               Npc,
               EntropyComponent::from(&mut source)
           ));
   }
}
```

### Enabling Determinism

Determinism relies on not just how RNGs are seeded, but also how systems are grouped and ordered relative to each other. Systems accessing the same source/entities will run serially to each other, but if you can separate entities into different groups that do not overlap with each other, systems can then run in parallel as well. Overall, care must be taken with regards to system ordering and scheduling, as well as unstable query iteration meaning the order of entities a query iterates through is not the same per run. This can affect the outcome/state of the PRNGs, producing different results.

The examples provided as integration tests in this repo demonstrate the two different concepts of parallelisation and deterministic outputs, so check them out to see how one might achieve determinism.

## Selecting and using PRNG Algorithms

All supported PRNGs and compatible structs are provided by `bevy_prng`, so the easiest way to work with `bevy_rand` is to import the necessary algorithm from `bevy_prng`. Simply activate the relevant features in `bevy_prng` to pull in the PRNG algorithm you want to use, and then import them like so:

```toml
bevy_prng = { version = "0.1", features = ["rand_chacha", "wyrand"] }
```

```rust ignore
use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_prng::{ChaCha8Rng, WyRand};
```

Using PRNGs directly from the `rand_*` crates is not possible without newtyping, and better to use `bevy_prng` instead of writing your own newtypes. Types from `rand` like `StdRng` and `SmallRng` are not encouraged to be used or newtyped as they are not **portable**, nor are they guaranteed to be stable. `SmallRng` is not even the same algorithm if used on 32-bit platforms versus 64-bit platforms. This makes them unsuitable for purposes of reflected/stable/portable types needed for deterministic PRNG. Likely, you'd be using `StdRng`/`SmallRng` directly without integration into Bevy's ECS. This is why the algorithm crates are used directly by `bevy_prng`, as this is the recommendation from `rand` when stability and portability is important.

As a whole, which algorithm should be used/selected is dependent on a range of factors. Cryptographically Secure PRNGs (CSPRNGs) produce very hard to predict output (very high quality entropy), but in general are slow. The ChaCha algorithm can be sped up by using versions with less rounds (iterations of the algorithm), but this in turn reduces the quality of the output (making it easier to predict). However, `ChaCha8Rng` is still far stronger than what is feasible to be attacked, and is considerably faster as a source of entropy than the full `ChaCha20Rng`. `rand` uses `ChaCha12Rng` as a balance between security/quality of output and speed for its `StdRng`. CSPRNGs are important for cases when you _really_ don't want your output to be predictable and you need that extra level of assurance, such as doing any cryptography/authentication/security tasks.

If that extra level of security is not necessary, but there is still need for extra speed while maintaining good enough randomness, other PRNG algorithms exist for this purpose. These algorithms still try to output as high quality entropy as possible, but the level of entropy is not enough for cryptographic purposes. These algorithms should **never be used in situations that demand security**. Algorithms like `WyRand` and `Xoshiro256StarStar` are tuned for maximum throughput, while still possessing _good enough_ entropy for use as a source of randomness for non-security purposes. It still matters that the output is not predictable, but not to the same extent as CSPRNGs are required to be.

## Features

- **`thread_local_entropy`** - Enables `ThreadLocalEntropy`, overriding `SeedableRng::from_entropy` implementations to make use of thread local entropy sources for faster PRNG initialisation. Enabled by default.
- **`serialize`** - Enables [`Serialize`] and [`Deserialize`] derives. Enabled by default.

## Supported Versions & MSRV

`bevy_rand` uses the same MSRV as `bevy`.

| `bevy`   | `bevy_rand` |
| -------- | ----------- |
| v0.11    | v0.2, v0.3  |
| v0.10    | v0.1        |

## Migrating from v0.2 to v0.3

As v0.3 is a breaking change to v0.2, the process to migrate over is fairly simple. The rand algorithm crates can no longer be used directly, but they can be swapped wholesale with `bevy_prng` instead. So the following `Cargo.toml` changes:

```diff
- rand_chacha = { version = "0.3", features = ["serde1"] }
+ bevy_prng = { version = "0.1", features = ["rand_chacha"] }
```

allows then you to swap your import like so, which should then plug straight into existing `bevy_rand` usage seamlessly:

```diff
use bevy::prelude::*;
use bevy_rand::prelude::*;
- use rand_chacha::ChaCha8Rng;
+ use bevy_prng::ChaCha8Rng;
```

This **will** change the type path and the serialization format for the PRNGs, but currently, moving between different bevy versions has this problem as well as there's currently no means to migrate serialized formats from one version to another yet. The rationale for this change is to enable stable `TypePath` that is being imposed by bevy's reflection system, so that future compiler changes won't break things unexpectedly as `std::any::type_name` has no stability guarantees. Going forward, this should resolve any stability problems `bevy_rand` might have and be able to hook into any migration tool `bevy` might offer for when scene formats change/update.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
