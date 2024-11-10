# EXPERIMENTAL - Observer-driven Reseeding

The following feature is _experimental_ so to enable it, you'll need to edit your Cargo.toml file and change the dependency declaration for `bevy_rand` to have `features=["experimental"]` applied. Once done, you'll get access to some utils that will enable easy setup of observer driven reseeding utilities for managing when entities with `EntropyComponent`s obtain new seeds from which sources. Keep in mind, this feature is not *stable* and will be subject to further work and iteration, so if problems and issues are encountered, please do create issues outlining the use-cases and difficulties.

By default, when the `experimental` feature is enabled, you'll be able to trigger a reseeding for a given entity either by pulling from a global source, or by providing a set seed value. This does not require any specific setup, and can simply be triggered by emitting the event on the entity needing to be reseeded. ALl observer events require providing a generic for the RNG algorithm to be targetted, as an entity could have multiple RNG sources attached to it.

```rust ignore
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::observers::SeedFromGlobal;

#[derive(Component)]
struct Target;

fn reseed_target_entities_from_global(mut commands: Commands, mut q_targets: Query<Entity, With<Target>>) {
    for target in &q_targets {
        commands.trigger_targets(SeedFromGlobal::<WyRand>::default(), target);
    }
}
```

Alternatively, one can provide a set seed to reseed all target entities with:

```rust ignore
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::observers::ReseedRng;

#[derive(Component)]
struct Target;

fn reseed_target_entities_from_set_seed(mut commands: Commands, mut q_targets: Query<Entity, With<Target>>) {
    let seed = u64::to_ne_bytes(42); 

    for target in &q_targets {
        commands.trigger_targets(ReseedRng::<WyRand>::new(seed), target);
    }
}
```

With these observers, you can initialise entropy components on all targetted entities simply by triggering a reseed event on them. As long as your entities have been spawned with a component you can target them with, they will automatically be given an `RngSeed` component (which stores the initial seed value) and an `EntropyComponent`.

Additionally, you can link entities to draw their seeds from other source entities instead of the global resources. So one `Source` entity can then seed many `Target` entities, and whenever the `Source` entity is updated with a new seed value, it then automatically pushes new seeds to its linked targets. Note: this is NOT a `bevy_hierarchy` relationship, and while the `Source` will have "child" entities, removing/despawning the source entity will *not* despawn the children entities. They will simply no longer have a valid "link". A new link can be established by triggering another "link" event.

```rust ignore
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::observers::{LinkRngSourceToTarget, SeedFromGlobal};

#[derive(Component)]
struct Source;

#[derive(Component, Clone, Copy)]
struct Target;

fn initial_setup(mut commands: Commands) {
    // Create the source entity and get the Entity id.
    let source = commands.spawn(Source).id();

    // Spawn many target entities
    commands.spawn_batch(vec![Target; 5]);

    // Link the target entities to the Source entity
    commands.trigger(LinkRngSourceToTarget::<Source, Target, WyRand>::default());

    // Initialise the Source entity to be an RNG source and then seed all its
    // linked entities.
    commands.trigger_targets(SeedFromGlobal::<WyRand>::default(), source);
}
```

Once the link has been created, child entities can also pull a new seed from its parent source. So if you want to reseed *one* entity from its parent source, but not all of the entities that have the same source, you can use the `SeedFromParent` observer event to achieve this.

```rust ignore
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::observers::SeedFromParent;

#[derive(Component)]
struct Source;

#[derive(Component, Clone, Copy)]
struct Target;

fn pull_seed_from_parent(mut commands: Commands, mut q_targets: Query<Entity, With<Target>>) {
    for target in &q_targets {
        commands.trigger_targets(SeedFromParent::<WyRand>::default(), target);
    }
}
```
