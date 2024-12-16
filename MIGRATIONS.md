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
