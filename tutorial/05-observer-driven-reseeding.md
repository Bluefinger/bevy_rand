# Observer-driven and Command-based Reseeding

Managing the seeding of related RNGs manually can be complex and also boilerplate-y, so `bevy_rand` provides a commands API powered by observers and bevy relations to make it much more easy to setup and maintain. This way, pushing a new seed to a single "source" RNG will then automatically push out new seeds to all linked "target" RNGs. It can also be set up for seeding between different kinds of PRNG, but it does require the addition of an extra plugin in order to facilitate this particular case.

The nature of the relations are strictly either One to One or One to Many. Many to Many relations are **not** supported, as it does not make sense for PRNG to have multiple source PRNGs.

```rust
use bevy_app::prelude::*;
use bevy_prng::{ChaCha8Rng, WyRand};
use bevy_rand::prelude::{EntropyPlugin, EntropyObserversPlugin};

fn main() {
    App::new()
        .add_plugins((
            // First initialise the RNGs
            EntropyPlugin::<ChaCha8Rng>::default(),
            EntropyPlugin::<WyRand>::default(),
            // This initialises observers for WyRand -> WyRand seeding relations
            EntropyObserversPlugin::<WyRand, WyRand>::default(),
            // This initialises observers for ChaCha8Rng -> WyRand seeding relations
            EntropyObserversPlugin::<ChaCha8Rng, WyRand>::default(),
        ))
        .run();
}
```

Once the plugins are initialised, various observer systems are ready to begin listening to various linking and reseeding events. Relations can exist between a global source and "local" sources, or between other entity local sources. So for example, a single `Global` source can seed many `Player` entities with their own RNGs, and to reseed all `Player` entities, you just need to push a new seed to the global source, or tell the global source to reseed all its linked RNGs.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{RngEntity, GlobalRngEntity};

#[derive(Component)]
struct Target;

fn link_and_seed_target_rngs_with_global(q_targets: Query<Entity, With<Target>>, mut global: GlobalRngEntity<WyRand>) {
    let targets = q_targets.iter().collect::<Vec<_>>();

    global.rng_commands().link_target_rngs(&targets).reseed_linked();
}
```

In the above example, we have created a relationship between the `Global` `WyRand` source and all `Target` entities. The above system creates the relations and then emits a reseeding event, causing all `Target` entities to receive a new `RngSeed` component from the `Global` source. This in turn initialises an `Entropy` component on each `Target` entity with the received seed.

The `GlobalRngEntity` is a special `SystemParam` that access the `Global` source `Entity` for a particular PRNG type. This then allows you to directly ask for an `RngEntityCommands` via the `rng_commands()` method. With this, you can link and seed your Source or Targets.

Alternatively, one can provide a set seed to reseed all target entities with:

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;

#[derive(Component)]
struct Target;

fn intialise_rng_entities_with_set_seed(mut commands: Commands, mut q_targets: Query<Entity, With<Target>>) {
    let seed = u64::to_ne_bytes(42); 

    for target in &q_targets {
        commands.entity(target).rng::<WyRand>().reseed(seed);
    }
}
```

The commands API takes an `EntityCommands` and extends it to provide an `rng()` method that takes a generic parameter to define which PRNG you want to initialise/use, and then provides methods to trigger events for seeding either the entity itself, or creating relations between other entities for seeding.

For entities that already have `RngSeed` attached to them, you can make use of `RngEntity` to query them. `Commands` also has a `rng()` method provided that takes a `&RngEntity` to give a `RngEntityCommands` without needing to explicitly provide the generics parameter to yield the correct PRNG target.

These command APIs are designed to alleviate or reduce some of the generic typing around the Rng components, so to make it less error prone and more robust that you are targetting the correct PRNG type, and also to make it easier on querying and managing more complex relations of dependencies between RNGs for seeding.

Once the relations are created, it becomes easy to pull new seeds from sources/global using the commands API:

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{RngEntity, RngCommandsExt};

#[derive(Component)]
struct Source;

#[derive(Component)]
struct Target;

fn pull_seeds_from_source(mut commands: Commands, q_targets: Query<RngEntity<WyRand>, With<Target>>) {
    for entity in q_targets {
        commands.rng(&entity).reseed_from_source();
    }
}
```

Of course, one _can_ also make use of the observer events directly, though it does require more typing to be involved. An example below:

```rust
use bevy_ecs::{prelude::*, relationship::RelatedSpawnerCommands};
use bevy_prng::WyRand;
use bevy_rand::prelude::{SeedFromGlobal, RngSource};

#[derive(Component)]
struct Source;

#[derive(Component, Clone, Copy)]
struct Target;

fn initial_setup(mut commands: Commands) {
    // Create the source entity with its related target entities and get the Entity id.
    let source = commands
        .spawn(Source)
        .with_related(|s: &mut RelatedSpawnerCommands<'_, RngSource<WyRand, WyRand>>| {
            vec![Target; 5].into_iter().for_each(|bundle| {
                s.spawn(bundle);
            });
        })
        .id();

    // Initialise the Source entity to be an RNG source and then seed all its
    // linked entities.
    commands.trigger_targets(SeedFromGlobal::<WyRand, WyRand>::default(), source);
}
```

Once the link has been created, child entities can also pull a new seed from its parent source. So if you want to reseed *one* entity from its parent source, but not all of the entities that have the same source, you can use the `SeedFromSource` observer event to achieve this.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::observers::SeedFromSource;

#[derive(Component)]
struct Source;

#[derive(Component, Clone, Copy)]
struct Target;

fn pull_seed_from_parent(mut commands: Commands, mut q_targets: Query<Entity, With<Target>>) {
    for target in &q_targets {
        commands.trigger_targets(SeedFromSource::<WyRand, WyRand>::default(), target);
    }
}
```

## Note about relations between PRNG types

As covered in Chapter One: "Selecting and using PRNG Algorithms", when creating relationship between different PRNG types, do not seed a stronger PRNG from a weaker one. CSPRNGs like `ChaCha8` should be seeded by either other `ChaCha8` sources or "stronger" sources like `ChaCha12` or `ChaCha20`, never from ones like `WyRand` or `Xoshiro256StarStar`. Always go from stronger to weaker, or same to same, never from weaker to stronger. Doing so makes it easier to predict the seeded PRNG, and reduces the advantage of using a CSPRNG in the first place.

So in summary:

| Source     | Target     |         |
| ---------- | ---------- | ------- |
| `ChaCha8`  | `Wyrand`   | ✅ Good |
| `ChaCha8`  | `WyRand`   | ✅ Good |
| `ChaCha12` | `ChaCha8`  | ✅ Good |
| `WyRand`   | `WyRand`   | ✅ Good |
| `Wyrand`   | `ChaCha8`  | ❌ Bad  |
| `ChaCha8`  | `ChaCha12` | ❌ Bad  |
