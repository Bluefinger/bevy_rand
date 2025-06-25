# Basic Usage with `GlobalEntropy`

At the simplest case, using `GlobalEntropy` directly for all random number generation, though this does limit how well systems using `GlobalEntropy` can be parallelised. This is because `GlobalEntropy` is a query to access a single entity with a mutable reference to the `Entropy` component. All systems that access `GlobalEntropy` will run serially to each other.

```rust
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

fn print_random_value(mut rng: GlobalEntropy<ChaCha8Rng>) {
    println!("Random value: {}", rng.next_u32());
}
```

In addition, the `rand` crate can be optionally pulled in and used with `bevy_rand`, benefitting from the full suite of methods and utilities. This allows full compatibility with all the distribution utilities within `rand` and also with `bevy_math`.

```rust ignore
use bevy_math::{ShapeSample, primitives::Circle};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::GlobalEntropy;
use rand::Rng;

fn print_random_value(mut rng: GlobalEntropy<ChaCha8Rng>) {
    println!("Random u128 value: {}", rng.random::<u128>());
}

fn sample_from_circle(mut rng: GlobalEntropy<ChaCha8Rng>) {
    let circle = Circle::new(42.0);

    let boundary = circle.sample_boundary(rng.as_mut());
    let interior = circle.sample_interior(rng.as_mut());

    println!("Sampled values from Circle: {boundary:?}, {interior:?}");
}
```

## Determinism when using `GlobalEntropy`

When using `GlobalEntropy`, the way to ensure deterministic output/usage with the single entity is in the following ways:

- One time or non-looping accesses when generating random numbers.
- Iterating over set loops/ranges.
- Systems constrained with clear priorities and ordering so there are no ambiguities.

An example of a guaranteed deterministic system is perhaps spawning new entities with a randomised component value:

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

#[derive(Component)]
struct Npc;

#[derive(Component)]
struct Stat(u32);

fn spawn_randomised_npcs(mut commands: Commands, mut rng: GlobalEntropy<WyRand>) {
    for _ in 0..10 {
        commands.spawn((
            Npc,
            Stat(rng.next_u32())
        ));
    }
}
```

The above system will iterate a set number of times, and will yield 10 randomised `u32` values, leaving the PRNG in a determined state. Any system that then accesses `GlobalEntropy<WyRand>` afterwards will always yield a predetermined value if the PRNG was given a set seed.

However, iterating over queries will **not** yield deterministic output, as queries are not guaranteed to iterate over collected entities in the same order every time the system is ran. Therefore, the below example will not have deterministic output.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

#[derive(Component)]
struct Npc;

#[derive(Component)]
struct Stat(u32);

fn randomise_npc_stat(mut rng: GlobalEntropy<WyRand>, mut q_npc: Query<&mut Stat, With<Npc>>) {
    for mut stat in q_npc.iter_mut() {
        stat.0 = rng.next_u32();
    }
}
```

But how can we achieve determinism in cases of where we want to randomise values within a query iteration? Well, by not using `GlobalEntropy` but `Entropy` instead, moving the RNG source from a single global entity to the entities you want to provide entropy to themselves. The next section will cover their usage.
