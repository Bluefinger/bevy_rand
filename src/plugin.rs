use core::marker::PhantomData;

use crate::{global::GlobalRng, seed::RngSeed, traits::SeedSource};
use bevy_app::{App, Plugin};
use bevy_prng::{EntropySeed, EntropySource};

/// Plugin for integrating a PRNG that implements `RngCore` into
/// the bevy engine, registering types for a global resource and
/// entropy components.
///
/// ```
/// use bevy_app::prelude::*;
/// use bevy_ecs::prelude::*;
/// use bevy_prng::{ChaCha8Rng, WyRand};
/// use bevy_rand::prelude::{EntropyPlugin, GlobalRng};
/// use rand_core::RngCore;
///
/// fn main() {
///  App::new()
///    .add_plugins((
///        EntropyPlugin::<ChaCha8Rng>::default(),
///        EntropyPlugin::<WyRand>::default()
///    ))
///    .add_systems(Update, print_random_value)
///    .run();
/// }
///
/// fn print_random_value(mut rng: Single<&mut WyRand, With<GlobalRng>>) {
///   println!("Random value: {}", rng.next_u32());
/// }
/// ```
pub struct EntropyPlugin<Rng: EntropySource + 'static> {
    seed: Option<Rng::Seed>,
}

impl<Rng: EntropySource + 'static> EntropyPlugin<Rng> {
    /// Creates a new plugin instance configured for randomised,
    /// non-deterministic seeding of the global entropy resource.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { seed: None }
    }

    /// Configures the plugin instance to have a set seed for the
    /// global entropy resource.
    #[inline]
    pub fn with_seed(seed: Rng::Seed) -> Self {
        Self { seed: Some(seed) }
    }
}

impl<Rng: EntropySource + 'static> Default for EntropyPlugin<Rng> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Rng: EntropySource + 'static> Plugin for EntropyPlugin<Rng>
where
    Rng::Seed: EntropySeed,
{
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_reflect")]
        app.register_type::<Rng>()
            .register_type::<RngSeed<Rng>>()
            .register_type::<Rng::Seed>();

        let world = app.world_mut();

        world.spawn((
            self.seed
                .clone()
                .map_or_else(RngSeed::<Rng>::default, RngSeed::<Rng>::from_seed),
            GlobalRng,
        ));

        world.add_observer(crate::observers::seed_from_global::<Rng, Rng>);
        world.add_observer(crate::observers::seed_from_parent::<Rng, Rng>);
        world.add_observer(crate::observers::seed_linked::<Rng, Rng>);
        world.add_observer(crate::observers::trigger_seed_linked::<Rng, Rng>);

        world.flush();
    }
}

/// [`Plugin`] for setting up relations/observers for handling related Rngs. It takes two generic parameters,
/// the first is the `Source` Rng, which is the algorithm for the source Rng entity, and then the second
/// is the `Target` Rng, which is the algorithm for the targets. It follows a One to One/Many relationship
/// model, going from `Source` to `Target`, where `Source` can have one or many `Target`s.
///
/// Note: This is for RNG algorithms, not Components. For more information, please read the
/// [tutorial](https://docs.rs/bevy_rand/latest/bevy_rand/tutorial/ch05_observer_driven_reseeding/index.html).
///
/// ```
/// use bevy_app::prelude::*;
/// use bevy_prng::{ChaCha8Rng, WyRand};
/// use bevy_rand::prelude::{EntropyPlugin, EntropyRelationsPlugin};
///
/// App::new()
///     .add_plugins((
///         // First initialise the RNGs. This also initialises observers for WyRand -> WyRand
///         // and ChaCha8Rng -> ChaCha8Rng seeding relations
///         EntropyPlugin::<ChaCha8Rng>::default(),
///         EntropyPlugin::<WyRand>::default(),
///         // You only need to explicitly provide the relations plugin for cross PRNG relations.
///         // For example: This initialises observers for ChaCha8Rng -> WyRand seeding relations
///         EntropyRelationsPlugin::<ChaCha8Rng, WyRand>::default(),
///     ))
///     .run();
/// ```
pub struct EntropyRelationsPlugin<Source, Target> {
    _source: PhantomData<Source>,
    _target: PhantomData<Target>,
}

impl<Source: EntropySource, Target: EntropySource> Default
    for EntropyRelationsPlugin<Source, Target>
{
    fn default() -> Self {
        Self {
            _source: PhantomData,
            _target: PhantomData,
        }
    }
}

impl<Source: EntropySource, Target: EntropySource> Plugin
    for EntropyRelationsPlugin<Source, Target>
{
    fn build(&self, app: &mut App) {
        let world = app.world_mut();

        world.add_observer(crate::observers::seed_from_global::<Source, Target>);
        world.add_observer(crate::observers::seed_from_parent::<Source, Target>);
        world.add_observer(crate::observers::seed_linked::<Source, Target>);
        world.add_observer(crate::observers::trigger_seed_linked::<Source, Target>);

        world.flush();
    }
}
