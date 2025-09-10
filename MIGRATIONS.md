# Migration Notes

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

## Migrating from v0.5 to v0.6

As the `wyrand` dependency has been updated and contains a breaking output change, users of `bevy_rand` making use of the `wyrand` feature will need to update their code in cases where deterministic output from the old version is expected. The new `WyRand` output is considered to provide better entropy than the old version, so it is recommended to adopt the new version. In reality, this is likely to affect tests and serialised output rather than game code.

## Migrating from v0.7 to v0.8

`GlobalRngSeed` has been removed, instead being rolled into `GlobalEntropy`. This will allow better reflection tracking of the global rng source, and will allow for automatic reseeding without any custom system needing to be provided. Use the `reseed` method to reinstantiate the internal RNG source with the new seed, and `get_seed` to return a reference to the initial starting seed for the source. The serialized format of `GlobalEntropy` has changed and previously serialized instances are no longer compatible.

## Migrating from v0.8 to v0.9

The tutorial has been updated to reflect changes to the APIs and intended usages, so please do take a [look at them](https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/index.html).

`EntropyComponent` has been renamed to `Entropy`, and the trait `SeedableEntropySource` has been renamed to `EntropySource`. The change to `Entropy` also changes the `TypePath` definition, so this will change the serialised format of the component.

`GlobalEntropy` is no longer a resource, it is a query helper for accessing a `Global` `Entropy` source. It's all entities now, so for "global" and unique sources, they are entities created during plugin initialisation with a `Global` marker component. It is guaranteed to be a single instance per algorithm type, so accessing them is done via `Single` queries. In place of a resource access, there's now helper queries provided in case you need to access the source entity in question for a variety of purposes:

* `GlobalEntropy` is for accessing the `Entropy` component from `Global`.
* `GlobalSeed` is for accessing the `RngSeed` component from `Global`. This is read-only however, since `RngSeed` is an immutable component.
* `GlobalSource` is for getting the `Entity` of the `Global` source. You likely need this query if you want to reseed the global source, as you'll need to insert a new `RngSeed` component to the entity.

For most usages of `GlobalEntropy`, updating should be very straightforward:

```diff
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{GlobalEntropy, ForkableRng};

#[derive(Component)]
struct Source;

- fn setup_source(mut commands: Commands, mut global: ResMut<GlobalEntropy<WyRand>>) {
+ fn setup_source(mut commands: Commands, mut global: GlobalEntropy<WyRand>) {
    commands
        .spawn((
            Source,
            global.fork_rng(),
        ));
}
```

## Migrating from v0.9 to v0.10

To begin with, the `experimental` feature no longer does anything, as the observers/commands API is now exposed by default. The feature hasn't been removed, as it may be used for future experimental APIs. There is now a `wasm_js` feature to help configure `getrandom` for WASM, though there's additional steps needed to build for WASM [outlined here](README#usage-within-web-wasm-environments).

Due to the upgrade to `rand`/`rand_core` v0.9, a lot of crates in the wider `rand` ecosystem have yet to fully transition over to the latest version. As such, there's a new `compat` feature that enables the old `RngCore` trait implementations on the PRNGs in `bevy_prng`, allowing for backwards compatibility. Doing so will pull the older `rand_core` v0.6 as a dependency, but it is enabled without any default features. **NOTE:** This is currently enabled by default, due to `bevy_math` still using the older `rand` versions.

```toml
bevy_rand = { version = "0.10", features = ["wyrand", "compat"] }
```

`GlobalSource` and `GlobalSeed` have been removed and now is represented by a `GlobalRngEntity` SystemParam. All uses of `GlobalSource` & `GlobalSeed` can be replaced by `GlobalRngEntity`.

Various observer events have been removed and replaced with Bevy's relations APIs. `LinkRngSourceToTarget`, `ReseedRng`, `RngChildren` and `RngParent` no longer exist. Instead, for queries, `RngLinks` and `RngSource` are the relations components, and `SeedFromGlobal`, `SeedFromSource`, and `SeedLinked` are the new observer events. For the most part, it is recommended to use the new Commands APIs provided by `RngCommandsExt` & `RngEntityCommandsExt`.

To enable the full relations API and features, make sure to add the `EntropyRelationsPlugin` to your bevy app.

## Migrating from v0.10 to v0.11

The breaking change here is that when `bevy_rand` has `default-features` set to `false`, it won't bring in `bevy_reflect` support any more. This is so that for more resource constrained `no_std` platforms, reflection support can be opted out in order to conserve memory usage. Reflection support can then be added back in by explicitly specifying `bevy_reflect` as a feature:

```toml
bevy_rand = { version = "0.11", default-features = false, features = ["bevy_reflect", "wyrand"] }
```

This change does not affect default/`std` usage of `bevy_rand`, which includes `bevy_reflect` support out of the box.

## Migrating from v0.11 to v0.12

The breaking changes here are that for `RngEntityCommands`, `with_target_rng` and `with_target_rngs_as` no longer automatically send a reseed event after spawning. This will need to be done manually with `reseed_linked` or `reseed_linked_as` after a spawn, like so:

```diff
use bevy_ecs::prelude::*;
use bevy_rand::prelude::*;
use bevy_prng::{ChaCha8Rng, WyRand};

#[derive(Component)]
struct Source;
#[derive(Component)]
struct Target;

fn setup_rng_sources(mut global: GlobalRngEntity<ChaCha8Rng>) {
    global
        .rng_commands()
        .with_target_rngs_as::<WyRand>([(
            Source,
            RngLinks::<WyRand, WyRand>::spawn((
                Spawn(Target),
                Spawn(Target),
                Spawn(Target),
                Spawn(Target),
                Spawn(Target),
            )),
-       )]);
+       )])
+       .reseed_linked_as::<WyRand>();
}
```

The `Global` marker struct is now `GlobalRng`, and the type helper `GlobalEntropy` is no longer provided. Feel free to make your own type alias for the `Single` query.

```diff
+use bevy_ecs::prelude::*;
use bevy_prng::ChaCha8Rng;
-use bevy_rand::prelude::GlobalEntropy;
+use bevy_rand::prelude::{Entropy, GlobalRng};
use rand_core::RngCore;

-fn print_random_value(mut rng: GlobalEntropy<WyRand>) {
+fn print_random_value(mut rng: Single<&mut Entropy<ChaCha8Rng>, With<GlobalRng>>) {
    println!("Random value: {}", rng.next_u32());
}
```

Due to the Event Rearchitecture, `RngEntityCommands` was reworked to use `Commands` instead of `EntityCommands`, and `RngCommandsExt` has been merged into `RngEntityCommandsExt`. So for getting a `RngEntityCommands`, the changes you need to make are as follows:

```diff
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;

#[derive(Component)]
struct Target;

fn intialise_rng_entities(mut commands: Commands, mut q_targets: Query<Entity, With<Target>>) {
    for target in &q_targets {
-       commands.entity(target).rng::<WyRand>().reseed_from_os_rng();
+       commands.rng::<WyRand>(target).reseed_from_os_rng();
    }
}
```
