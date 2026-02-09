# Overview

Games often use randomness as a core mechanic. For example, card games generate a random deck for each game and killing monsters in an RPG often rewards players with a random item. While randomness makes games more interesting and increases replayability, it also makes games harder to test and prevents advanced techniques such as [deterministic lockstep](https://gafferongames.com/post/deterministic_lockstep/).

Let's pretend you are creating a poker game where a human player can play against the computer. The computer's poker logic is very simple: when the computer has a good hand, it bets all of its money. To make sure the behavior works, you write a test to first check the computer's hand and if it is good confirm that all its money is bet. If the test passes does it ensure the computer behaves as intended? Sadly, no.

Because the deck is randomly shuffled for each game (without doing so the player would already know the card order from the previous game), it is not guaranteed that the computer player gets a good hand and thus the betting logic goes unchecked. While there are ways around this (a fake deck that is not shuffled, running the test many times to increase confidence, breaking the logic into units and testing those) it would be very helpful to have randomness as well as a way to make it _less_ random.

Luckily, when a computer needs a random number it doesn't use real randomness and instead uses a [pseudorandom number generator](https://en.wikipedia.org/wiki/Pseudorandom_number_generator). Popular Rust libraries containing pseudorandom number generators are [`rand`](https://crates.io/crates/rand) and [`fastrand`](https://crates.io/crates/fastrand).

Pseudorandom number generators require a source of [entropy](https://en.wikipedia.org/wiki/Entropy) called a [random seed](https://en.wikipedia.org/wiki/Random_seed). The random seed is used as input to generate numbers that _appear_ random but are instead in a specific and deterministic order. For the same random seed, a pseudorandom number generator always returns the same numbers in the same order.

For example, let's say you seed a pseudorandom number generator with `1234`. You then ask for a random number between `10` and `99` and the pseudorandom number generator returns `12`. If you run the program again with the same seed (`1234`) and ask for another random number between `1` and `99`, you will again get `12`. If you then change the seed to `4567` and run the program, more than likely the result will not be `12` and will instead be a different number. If you run the program again with the `4567` seed, you should see the same number from the previous `4567`-seeded

There are many types of pseudorandom number generators each with their own strengths and weaknesses. Because of this, Bevy does not include a pseudorandom number generator. Instead, the `bevy_rand` plugin includes a source of entropy to use as a random seed for your chosen pseudorandom number generator.

Note that Bevy currently has [other sources of non-determinism](https://github.com/bevyengine/bevy/discussions/2480) unrelated to pseudorandom number generators.

# Why and when to use Bevy_Rand?

So you need to use some randomness in your game/application. In a lot of very simple cases where there is no requirement for portability (needing to run on many different platforms) and/or no need for determinism (being able to run with predetermined algorithm & seeds/states), you can simple use `rand` or `fastrand` directly. Maybe your case is just to quickly randomise some instantiation of entities/components, and there's not much need for anything more than that. For these simple cases, `bevy_rand` is a bit overkill as a solution. However, the moment you begin to care/need determinism and portability, then it makes sense to use `bevy_rand`.

## Portability

This is the first concern of `bevy_rand`. The standard `StdRng`/`SmallRng` types from `rand` _are not portable_, meaning they are not expected to remain the same algorithm between different versions of the `rand` crate, or even the same PRNG algorithm between platforms. This is the case for `SmallRng`, which utilises different versions of the Xoshiro algorithm depending on whether you are on a 32-bit platform or 64-bit platform.

The `rand` crate itself notes and states that if users are concerned about the portability of their PRNGs, then they should be using the algorithm crates directly instead, so pulling in `chacha20` or `rand_xoshiro`. A non-portable PRNG means you'll potentially be dealing with different randomness behaviour between your desktop version of your application and web version. Portability resolves this issue by forcing the PRNG algorithm to always be the same, no matter the platform or configuration.

## Determinism

This is the second and most important concern of `bevy_rand`. `StdRng`/`SmallRng` types from `rand` might be _seedable_, but the lack of portability means they are _not deterministic_. The algorithm changing either from different versions of the `rand` crate or being on a different platform is _not deterministic_ behaviour. This property is highly important when it comes to ensuring correct behaviour across all versions of your game on whatever platform. But why would one consider determinism an important property?

### Testing

Being able to "know" that your tests will always output with a set value despite dealing with randomness will allow you to detect changes in how your code is dealing with RNG. It might have an adverse effect later down the line, such as breaking stability assurances from serialized game states. Being able to track how RNG is handled by your game code can increase and mitigate any breaking changes from occuring.

### Saving/Loading/Synchronising States

If you want to have a replay system that can show all the player's moves, including attacks that had critical hits, activated procs, or other randomised effects in the exact order they played out? That requires being able to go through the game's recorded state and rerun through it. If you have a deterministic simulation, just replaying the player's inputs would lead to the simulation result being the exact same as when it played out. By having the PRNG sources being deterministic as well in not just the setup (the seed), but in its usage, then it would still yield the deterministic output desired.

This principle can be extended to saving/loading game states to disk, not needing to worry about reinitialising the states of your PRNGs as their states can just be serialised and deserialised. Or synchronising clients with the same PRNG states to ensure their simulations are in agreement with each other.

### Parallelising RNG usage

Using a PRNG global source not only makes it very difficult to maintain determinism, but it also makes it impossible to parallelise systems that access it. For larger games with lots of systems, this could become a bottleneck and cause your game to underutilise modern hardware, given the proliferation of CPUs with large core/thread counts. `bevy_rand` introduces concepts and strategies that can be used to avoid needing to use a global source, which then not only potentially allows for parallelising systems, but also parallelising query iterations. One could parallelise RNG usage/access with thread local sources, but you then *lose* determinism as the systems and queries are no longer guaranteed to run on the same thread each time.

### And much more

`bevy_rand` might introduce some extra complexity with handling PRNGs within Bevy, but it is primarily for unblocking purposes that require that complexity. The abstractions necessary to do all this are not that difficult to understand or use, but do require thinking away from "global" sources and more towards "per entity" sources.
