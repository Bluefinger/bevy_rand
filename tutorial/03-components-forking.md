# Entities as RNG sources with `Entropy`

In order to move beyond the restrictions placed by `GlobalEntropy` and achieve determinism *with parallelism*, where the RNG source lives has to go from a global source to one owned by the entities themselves. `Entropy` enables us to attach a PRNG to any given entity, and thus sidesteps not only forcing systems to run serially to each other, but also avoids the problem of queries not being stable in ordering. In fact, as ordering is no longer an issue, parallel iteration of queries is made possible as we avoid borrowing issues if each entity we queried owns its own RNG source.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;

#[derive(Component)]
struct Source;

fn setup_source(mut commands: Commands) {
    commands
        .spawn((
            Source,
            Entropy::<WyRand>::default(),
        ));
}
```

In the above example, we are creating an entity with a `Source` marker component and attaching an `Entropy` to it with the `WyRand` algorithm and a randomised seed. To then access this source, we simply query `Query<&mut Entropy<WyRand>, With<Source>>`. In this case, we are creating a single entity with an RNG source, but there's no reason why many more can't have an RNG source attached to them.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;

#[derive(Component)]
struct Npc;

fn setup_source(mut commands: Commands) {
    for _ in 0..10 {
        commands
            .spawn((
                Npc,
                Entropy::<WyRand>::default(),
            ));
    }
}
```

`GlobalEntropy` is basically the same thing! It's just an `Entity` with an `Entropy` component and `RngSeed`, combined with a `Global` marker component. `GlobalEntropy` itself is not a type, but an alias for a query: `Query<&mut Entropy<WyRand>, With<Global>>`.

We can also instantiate these components with set seeds, but there's then the danger that with all of them having the same seed, they'll output the same random numbers. But we want determinism without being easy to predict across many, many entities. How would one achieve this? By forking.

## Forking new sources from existing ones

Forking is the process of generating a new seed from an RNG source and creating a new RNG instance with it. If cloning creates a new instance with the same state from the old, forking creates a new instance with a new state, advancing the old instance's state in the process (as we used it to generate a new seed).

Because PRNG algorithms are deterministic, forking is a deterministic process, and it allows us to have one seed state create many "random" states while being hard to predict. `bevy_rand` makes it super easy to fork new `Entropy`s, allowing you to source new RNGs from `GlobalEntropy` or even other `Entropy`s!

```rust
use bevy_ecs::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::{Entropy, GlobalRng, ForkableRng};

#[derive(Component)]
struct Source;

fn setup_source(mut commands: Commands, mut global: Single<&mut Entropy<ChaCha8Rng>, With<GlobalRng>>) {
    commands
        .spawn((
            Source,
            global.fork_rng(), // This will yield an `Entropy<ChaCha8Rng>`
        ));
}
```

We can even fork to different PRNG algorithms.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::{ChaCha8Rng, WyRand};
use bevy_rand::prelude::{Entropy, GlobalRng, ForkableAsRng};

#[derive(Component)]
struct Source;

fn setup_source(mut commands: Commands, mut global: Single<&mut Entropy<ChaCha8Rng>, With<GlobalRng>>) {
    commands
        .spawn((
            Source,
            global.fork_as::<WyRand>(), // This will yield an `Entropy<WyRand>`
        ));
}
```

So we created a `Source` entity with an RNG source, let's use it to spawn more entities with RNG sources!

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
   for _ in 0..10 {
       commands
           .spawn((
               Npc,
               q_source.fork_rng() // This will yield a new `Entropy<WyRand>`
           ));
   }
}
```

Now that we have our `Npc` entities attached with RNG sources, when we query them, we can make use of their own sources when generating new random numbers from them.

```rust ignore
fn randomise_npc_stat(mut q_npc: Query<(&mut Stat, &mut Entropy<WyRand>), With<Npc>>) {
    for (mut stat, mut rng) in q_npc.iter_mut() {
        stat.0 = rng.next_u32();
    }
}
```

This way, no matter what order the query iterates, we can be assured that the resulting output is always deterministic. Other systems that access different entities with RNG sources that don't overlap with `Npc` entity systems will be able to run in parallel, and iterating the queries themselves can also be done in parallel with `.par_iter()`. We've ensured that each *access* is deterministic and owned to the entity itself.

As a final note: for both `GlobalEntropy` and `Entropy`s, one can fork the inner PRNG instance to use directly or pass into methods via `fork_inner()`.

## Pitfalls when Querying

In general, never do a `Query<&mut Entropy<T>>` without any query filters.

In basic usages, there's only *one* entity, the `Global` entity for the enabled RNG algorithm. The above query will yield the `Global` entity, same as using `GlobalEntropy` query helper. However, if you've spawned more than one source, the above query will yield *all* `Entropy` entities, global and non-global ones included. The ordering is also not guaranteed, so the first result out of that query is not guaranteed to be the global entity.

Therefore, always use something like `Single` to enforce access to a single source such as `Single<&mut Entropy<T>, With<Source>>`, or use query helpers like `GlobalEntropy` to access global sources, or use a suitable filter for a marker component to filter out other sources from the ones you are interested in: `Query<&mut Entropy, With<Source>>`.
