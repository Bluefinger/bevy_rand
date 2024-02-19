use crate::{component::EntropyComponent, resource::GlobalEntropy};
use bevy::prelude::{App, Plugin};
use bevy_prng::SeedableEntropySource;
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
    R::Seed: Send + Sync + Copy,
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
    R::Seed: Send + Sync + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R: SeedableEntropySource + 'static> Plugin for EntropyPlugin<R>
where
    R::Seed: Send + Sync + Copy,
{
    fn build(&self, app: &mut App) {
        app.register_type::<GlobalEntropy<R>>()
            .register_type::<EntropyComponent<R>>();

        if let Some(seed) = self.seed {
            app.insert_resource(GlobalEntropy::<R>::from_seed(seed));
        } else {
            app.init_resource::<GlobalEntropy<R>>();
        }
    }
}
