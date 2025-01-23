use core::marker::PhantomData;

use crate::{component::Entropy, global::Global, seed::RngSeed, traits::SeedSource};
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
/// use bevy_rand::prelude::{EntropyPlugin, GlobalEntropy};
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
/// fn print_random_value(mut rng: GlobalEntropy<WyRand>) {
///   println!("Random value: {}", rng.next_u32());
/// }
/// ```
pub struct EntropyPlugin<Rng: EntropySource + 'static> {
    seed: Option<Rng::Seed>,
}

impl<Rng: EntropySource + 'static> EntropyPlugin<Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
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

impl<Rng: EntropySource + 'static> Default for EntropyPlugin<Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Rng: EntropySource + 'static> Plugin for EntropyPlugin<Rng>
where
    Rng::Seed: EntropySeed,
{
    fn build(&self, app: &mut App) {
        app.register_type::<Entropy<Rng>>()
            .register_type::<RngSeed<Rng>>()
            .register_type::<Rng::Seed>();

        let world = app.world_mut();

        world.register_component_hooks::<RngSeed<Rng>>();

        world.spawn((
            self.seed
                .clone()
                .map_or_else(RngSeed::<Rng>::from_entropy, RngSeed::<Rng>::from_seed),
            Global,
        ));

        world.add_observer(crate::observers::seed_from_global::<Rng>);

        world.flush();
    }
}

/// [`Plugin`] for setting up observers for handling related Rngs.
pub struct EntropyRelationPlugin<Rng: EntropySource> {
    _rng: PhantomData<Rng>,
}

impl<Rng: EntropySource> Default for EntropyRelationPlugin<Rng> {
    fn default() -> Self {
        Self { _rng: PhantomData }
    }
}

impl<Rng: EntropySource> Plugin for EntropyRelationPlugin<Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    fn build(&self, app: &mut App) {
        let world = app.world_mut();

        world.add_observer(crate::observers::seed_from_parent::<Rng>);
        world.add_observer(crate::observers::seed_linked::<Rng>);
        world.add_observer(crate::observers::trigger_seed_linked::<Rng>);

        world.flush();
    }
}
