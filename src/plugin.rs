#[cfg(feature = "experimental")]
use std::marker::PhantomData;

use crate::{component::EntropyComponent, resource::GlobalEntropy, seed::RngSeed};
#[cfg(feature = "experimental")]
use bevy_ecs::prelude::Component;
use bevy_app::{App, Plugin};
use bevy_prng::{EntropySeed, SeedableEntropySource};
use rand_core::SeedableRng;

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
        #[cfg(feature = "experimental")]
        app.add_observer(crate::observers::seed_from_global::<R>);
        app.world_mut().register_component_hooks::<RngSeed<R>>();
    }
}

/// Plugin for setting up linked RNG sources
#[cfg(feature = "experimental")]
pub struct LinkedEntropySources<Target: Component, Rng: SeedableEntropySource + 'static> {
    rng: PhantomData<Rng>,
    target: PhantomData<Target>,
}

#[cfg(feature = "experimental")]
impl<Target: Component, Rng: SeedableEntropySource + 'static> Default
    for LinkedEntropySources<Target, Rng>
{
    fn default() -> Self {
        Self {
            rng: PhantomData,
            target: PhantomData,
        }
    }
}

#[cfg(feature = "experimental")]
impl<Target: Component, Rng: SeedableEntropySource + 'static> Plugin
    for LinkedEntropySources<Target, Rng>
where
    Rng::Seed: Send + Sync + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_observer(crate::observers::seed_from_parent::<Rng>)
            .add_observer(crate::observers::seed_children::<Target, Rng>)
            .add_observer(crate::observers::link_targets::<Target, Rng>);
    }
}
