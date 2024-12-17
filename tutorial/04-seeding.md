# Seeding

So, why cover seeding at this point for its own chapter? Firstly, seeding can be an issue to get right. Nearly all RNGs are actually *pseudo*random, and are in fact just clever algorithms that output sequences of numbers that *look* random. Unless you are pulling your random numbers from a hardware source which samples from things like thermal noise, etc, your RNG is likely to be a *pseudorandom* generator. Very few sources are *true* random sources, and usually, such sources are actually quite *slow*, due to the need to collect enough entropy.

`bevy_rand` only really deals with PRNGs, and PRNGs *need* a seed value when initialised in order for the algorithm to do its thing. But why not just use `0` as a default? Because then the resulting sequence will always be the same. It'll be a *random* looking sequence, but every time you run the program, that sequence will be replayed. Because *pseudorandom* generators are deterministic.

This is a good property to have though! It means the algorithm is *testable*, and you can also test with it. And if the output is the same across different platforms, then it is *portable*. Even better! But it does mean you can't really use a *default* value. If you are using an RNG, you don't really *want* the same sequence every time you run your program. It would be too predictable, and in the context of a game, that can have some not so great consequences if players notice this.

## Where to get seeds?

`bevy_rand` provides two ways to give an `Entropy` component a seed:

1. Default: Pulling from a thread-local or OS source (Random)
2. Providing a set seed (Deterministic)

Wait, the first option is a default? It's true that library crates like `rand_chacha` don't implement their algorithms with defaults, but that's because they can't make assumptions about what sources are available by default and which platforms the algorithm will be used on. `bevy_rand` *can* make some assumptions however, because:

* `bevy_rand` is being used in the context of making games/applications in `bevy`, so we are assuming this will be used on platforms with the capability to support/run `bevy` apps (std or no-std).
* These platforms will have the ability to provide OS/hardware sources or allow for user-space sources.

In the rare occasion these assumptions cannot be upheld, there's still an escape hatch for this, but it involves getting stuck into `getrandom`. For those cases, I defer to the [getrandom documentation](https://docs.rs/getrandom/0.2.15/getrandom/macro.register_custom_getrandom.html) on how to enable support for these particular platforms.

Otherwise for most users, `bevy_rand` makes use of defaults to pull in random seeds from platform/user-space sources, when you aren't concerned about what seed but need it to be random enough that there's no chance that it can be predicted. As long as the above infrastructure is in place, it'll do this automatically. Once you have at least *one* random seed, it also becomes possible to use one source to generate new seeds for more sources, allowing for a *deterministic* distribution of seeds.

However, sometimes you want to make sure things are indeed random, or that what was distributed was deterministic, etc. That's where `RngSeed` component comes in.

## Knowing what seed you got

In the `rand` ecosystem, it has been decided for a long while that for security reasons, it should not be possible to observe the internal states of PRNGs via `Debug` or `Display`. There are good reasons for this "black box" approach, but in the context of game dev and needing to iterate and be able to observe the state of your game, it then makes debugging a very difficult prospect. You'd have to constantly serialise the RNG in order to gain insight to its internal state and that adds overhead.

But for most purposes, you don't actually *need* to know its exact internal state. Given these are *pseudorandom* sources, it is enough to observe the *initial state*, aka the seed value. `RngSeed` provides the ability to store the initial seed in a way that is observable via reflection and `Debug`. This way, you can observe your source entities without needing to worry about constantly serialising your RNGs.

`RngSeed` does more though. It also *instantiates* an `Entropy` component automatically with the provided seed value when it is inserted on an entity. For example: This means that instead of transmitting the serialised state of an RNG from a server to a client, you could just transmit the `RngSeed` component, and when it is instantiated on the client, it'll setup `Entropy` automatically.

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
            // This will yield a random `RngSeed<WyRand>` and then an `Entropy<WyRand>`
            // with the same random seed
            Entropy::<WyRand>::default(),
        ));
}
```

It also means all the previous examples about forking `Entropy` components can be directly adapted into forking *seeds* instead.

```rust
use bevy_ecs::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{Entropy, GlobalEntropy, ForkableSeed};

#[derive(Component)]
struct Source;

fn setup_source(mut commands: Commands, mut global: GlobalEntropy<WyRand>) {
    commands
        .spawn((
            Source,
            // This will yield a `RngSeed<WyRand>` and then an `Entropy<WyRand>`
            global.fork_seed(),
        ));
}
```

The `SeedSource` trait provides the methods needed to get access to the wrapped seed value, though `RngSeed` does implement `Deref` as well.

## Pitfalls when Querying

In general, never do a `Query<&mut RngSeed<T>>` without any query filters.

In basic usages, there's only *one* entity, the `Global` entity for the enabled RNG algorithm. The above query will yield the `Global` entity, same as using `GlobalSeed` query helper. However, if you've spawned more than one source, the above query will yield *all* `RngSeed` entities, global and non-global ones included. The ordering is also not guaranteed, so the first result out of that query is not guaranteed to be the global entity.

Therefore, always use something like `Single` to enforce access to a single source such as `Single<&mut RngSeed<T>, With<Source>>`, or use query helpers like `GlobalSeed` to access global sources, or use a suitable filter for a marker component to filter out other sources from the ones you are interested in: `Query<&mut RngSeed<T>, With<Source>>`.
