# Basic Usage with `GlobalEntropy`

At the simplest case, using `GlobalEntropy` directly for all random number generation, though this does limit how well systems using `GlobalEntropy` can be parallelised. All systems that access `GlobalEntropy` will run serially to each other.

```rust
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

fn print_random_value(mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    println!("Random value: {}", rng.next_u32());
}
```

In addition, the `rand` crate can be optionally pulled in and used with `bevy_rand`, benefitting from the full suite of methods and utilities.

## Determinism when using `GlobalEntropy`

When using `GlobalEntropy`, the way to ensure deterministic output/usage with the resource is in the following ways:

- One time or non-looping accesses when generating random numbers.
- Iterating over set loops/ranges.
- Systems constrained with clear priorities so there are no ambiguities.

An example of a guaranteed deterministic system is perhaps spawning new entities with a randomised component value:

```rust
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

#[derive(Component)]
struct Npc;

#[derive(Component)]
struct Stat(u32);

fn spawn_randomised_npcs(mut commands: Commands, mut rng: ResMut<GlobalEntropy<WyRand>>) {
    for _ in 0..10 {
        commands.spawn((
            Npc,
            Stat(rng.next_u32())
        ));
    }
}
```

The above system will iterate a set number of times, and will yield 10 randomis `u32` values, leaving the PRNG in a determined state. Any system that then accesses `GlobalEntropy<WyRand>` afterwards will always yield a predetermined value if the PRNG was given a set seed.

However, iterating over queries will **not** yield deterministic output, as queries are not guaranteed to iterate over collected entities in the same order every time the system is ran. Therefore, the below example will not have deterministic output.

```rust
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

#[derive(Component)]
struct Npc;

#[derive(Component)]
struct Stat(u32);

fn randomise_npc_stat(mut rng: ResMut<GlobalEntropy<WyRand>>, mut q_npc: Query<&mut Stat, With<Npc>>) {
    for mut stat in q_npc.iter_mut() {
        stat.0 = rng.next_u32();
    }
}
```

But how can we achieve determinism in cases of where we want to randomise values within a query iteration? Well, by not using `GlobalEntropy` but `EntropyComponent` instead, moving the RNG source to the entities themselves. The next section will cover their usage.
