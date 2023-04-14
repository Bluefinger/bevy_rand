use crate::{component::EntropyComponent, resource::GlobalEntropy, traits::SeedableEntropySource};
use bevy::prelude::{App, Plugin};
use rand_core::SeedableRng;

/// Plugin for integrating a PRNG that implements `RngCore` into
/// the bevy engine, registering types for a global resource and
/// entropy components.
///
/// ```
/// use bevy::prelude::*;
/// use bevy_rand::prelude::*;
/// use rand_core::RngCore;
/// use rand_chacha::{ChaCha8Rng, ChaCha12Rng};
///
/// fn main() {
///  App::new()
///    .add_plugin(EntropyPlugin::<ChaCha8Rng>::default())
///    .add_plugin(EntropyPlugin::<ChaCha12Rng>::default())
///    .add_system(print_random_value)
///    .run();
/// }
///
/// fn print_random_value(mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
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
