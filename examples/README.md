# Bevy Rand Examples

This folder contains various examples of `bevy_rand` usage, showcasing more advanced applications of `bevy_rand` that might not be as easy to see just from documentation and tutorials alone. Simple use-cases are already covered extensively by the existing documentation, so this folder will focus on providing more extensive/advanced cases to give users ideas on how to leverage `bevy_rand`.

## Available Examples

### `mine_clicker`

This is adapted from [Bevy's ECS Observers](https://github.com/bevyengine/bevy/blob/main/examples/ecs/observers.rs) example, but with some changes: Each `Mine` now has its own RNG state, and this state is linked to the `GlobalRng`. When a new `Mine` is spawned, it is given a relation to the `GlobalRng` so that when it is updated with a new seed, all linked `Mine`s get seeded with a new RNG as well. This then initialises the `Mine`'s position and size with random values generated from its own RNG state instead of the Global's state, allowing it to be far more robust and less sensitive to non-determinism within bevy's scheduling and event handling.

As a further addition, right clicking demonstrates the power of sending just one event to the GlobalRng to reseed, which then automatically reseeds all its linked RNGs in turn, doing so deterministically.

This example uses fully randomised initial state.

To run: `cargo run --release --bin mine_clicker`

### `turn_based`

This is adapted from Bevy's ECS Observer propagation example, but combined with my old example so now it incorporates three characters (a friendly and two enemy) that take turns attacking each other. Each `Character` not only has an RNG state on the main entity, but also RNG states on all child entities representing armor pieces.

During a turn, a `Character` uses its own RNG state to select a hostile target and which armor piece to attack. This targetted armor piece then has its own RNG state to roll a chance for the attack to glance off, doing no damage. The order of which `Character` does an attack is randomised per turn, and at the end of each turn, the RNG states of all entities are reseeded.

This example showcases how more complex relations can help maintain determinism, so that not just GlobalRng can be used to keep RNG states in sync, but also Parent-Child related entities as well.

This example uses a set seed to showcase deterministic outcomes for complex RNG relations/configurations.

To run: `cargo run --release --bin turn_based`

## Building/Running the examples with Bevy CLI

If you are using the [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli), you can run the examples directly using the CLI, and can even build them for the web. So for example, to run `mine_clicker` on the web with the CLI, use the following command:

```sh
bevy run --release --bin mine_clicker web
```

This should then compile and host the example to be run in the browser.
