use std::marker::PhantomData;

use crate::{
    observers::{LinkRngSourceToTarget, SeedFromGlobal},
    component::EntropyComponent,
    resource::GlobalEntropy,
    seed::RngSeed,
};
use bevy::prelude::{App, Component, Plugin};
use bevy_prng::{EntropySeed, SeedableEntropySource};
use rand_core::SeedableRng;

/// Plugin for integrating a PRNG that implements `RngCore` into
/// the bevy engine, registering types for a global resource and
/// entropy components.
///
/// ```
/// use bevy::prelude::*;
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
/// fn print_random_value(mut rng: ResMut<GlobalEntropy<WyRand>>) {
///   println!("Random value: {}", rng.next_u32());
/// }
/// ```
pub struct EntropyPlugin<R: SeedableEntropySource + 'static> {
    seed: Option<R::Seed>,
}

impl<R: SeedableEntropySource + 'static> EntropyPlugin<R>
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

impl<R: SeedableEntropySource + 'static> Default for EntropyPlugin<R>
where
    R::Seed: Send + Sync + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R: SeedableEntropySource + 'static> Plugin for EntropyPlugin<R>
where
    R::Seed: EntropySeed,
{
    fn build(&self, app: &mut App) {
        app.register_type::<GlobalEntropy<R>>()
            .register_type::<EntropyComponent<R>>()
            .register_type::<R::Seed>();

        if let Some(seed) = self.seed.as_ref() {
            app.insert_resource(GlobalEntropy::<R>::from_seed(seed.clone()));
        } else {
            app.init_resource::<GlobalEntropy<R>>();
        }

        app.observe(SeedFromGlobal::<R>::seed_from_global)
            .world_mut()
            .register_component_hooks::<RngSeed<R>>();
    }
}

pub struct ObserveEntropySources<Target: Component, Rng: SeedableEntropySource + 'static> {
    rng: PhantomData<Rng>,
    target: PhantomData<Target>,
}

impl<Target: Component, Rng: SeedableEntropySource + 'static> ObserveEntropySources<Target, Rng> {
    pub fn new() -> Self {
        Self { rng: PhantomData, target: PhantomData }
    }
}

impl<Target: Component, Rng: SeedableEntropySource + 'static> Plugin
    for ObserveEntropySources<Target, Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    fn build(&self, app: &mut App) {
        LinkRngSourceToTarget::<Target, Rng>::initialize(app);
    }
}
