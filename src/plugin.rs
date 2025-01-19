#[cfg(feature = "experimental")]
use std::marker::PhantomData;

use crate::{component::Entropy, global::Global, seed::RngSeed, traits::SeedSource};
use bevy_app::{App, Plugin};
#[cfg(feature = "experimental")]
use bevy_ecs::prelude::Component;
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
pub struct EntropyPlugin<R: EntropySource + 'static> {
    seed: Option<R::Seed>,
}

impl<R: EntropySource + 'static> EntropyPlugin<R>
where
    R::Seed: Send + Sync + Clone,
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
    pub fn with_seed(seed: R::Seed) -> Self {
        Self { seed: Some(seed) }
    }
}

impl<R: EntropySource + 'static> Default for EntropyPlugin<R>
where
    R::Seed: Send + Sync + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R: EntropySource + 'static> Plugin for EntropyPlugin<R>
where
    R::Seed: EntropySeed,
{
    fn build(&self, app: &mut App) {
        app.register_type::<Entropy<R>>()
            .register_type::<RngSeed<R>>()
            .register_type::<R::Seed>();

        let world = app.world_mut();

        world.register_component_hooks::<RngSeed<R>>();

        world.spawn((
            self.seed
                .clone()
                .map_or_else(RngSeed::<R>::from_entropy, RngSeed::<R>::from_seed),
            Global,
        ));

        #[cfg(feature = "experimental")]
        {
            world.add_observer(crate::observers::seed_from_global::<R>);
            world.add_observer(crate::observers::reseed::<R>);
            world.add_observer(crate::observers::seed_from_parent::<R>);
            world.add_observer(crate::observers::seed_children::<R>);
            world.add_observer(crate::observers::trigger_seed_children::<R>);
        }

        world.flush();
    }
}

/// Plugin for setting up linked RNG sources
#[cfg(feature = "experimental")]
pub struct LinkedEntropySources<Source: Component, Target: Component, Rng: EntropySource + 'static>
{
    rng: PhantomData<Rng>,
    source: PhantomData<Source>,
    target: PhantomData<Target>,
}

#[cfg(feature = "experimental")]
impl<Source: Component, Target: Component, Rng: EntropySource + 'static> Default
    for LinkedEntropySources<Source, Target, Rng>
{
    fn default() -> Self {
        Self {
            rng: PhantomData,
            source: PhantomData,
            target: PhantomData,
        }
    }
}

#[cfg(feature = "experimental")]
impl<Source: Component, Target: Component, Rng: EntropySource + 'static> Plugin
    for LinkedEntropySources<Source, Target, Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_observer(crate::observers::link_targets::<Source, Target, Rng>);
    }
}
