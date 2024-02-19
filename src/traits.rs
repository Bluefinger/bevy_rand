use bevy_prng::SeedableEntropySource;
use rand_core::{RngCore, SeedableRng};

/// Trait for implementing Forking behaviour for [`crate::component::EntropyComponent`] and [`crate::resource::GlobalEntropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic.
pub trait ForkableRng: EcsEntropySource {
    /// The type of instance that is to be forked from the original source.
    type Output: EcsEntropySource;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method preserves the RNG algorithm between original and forked instances.
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_prng::ChaCha8Rng;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableRng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_rng(),
    ///         ));
    /// }
    /// ```
    fn fork_rng(&mut self) -> Self::Output {
        Self::Output::from_rng(self).unwrap()
    }
}

/// Trait for implementing Forking behaviour for [`crate::component::EntropyComponent`] and [`crate::resource::GlobalEntropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking between different PRNG algorithm types.
pub trait ForkableAsRng: EcsEntropySource {
    /// The type of instance that is to be forked from the original source.
    type Output<R>: EcsEntropySource
    where
        R: SeedableEntropySource;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method allows one to specify the RNG algorithm to be used for the forked instance.
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableAsRng};
    /// use bevy_prng::{ChaCha8Rng, ChaCha12Rng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: ResMut<GlobalEntropy<ChaCha12Rng>>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_as::<ChaCha8Rng>(),
    ///         ));
    /// }
    /// ```
    fn fork_as<T: SeedableEntropySource>(&mut self) -> Self::Output<T> {
        Self::Output::<_>::from_rng(self).unwrap()
    }
}

/// Trait for implementing Forking behaviour for [`crate::component::EntropyComponent`] and [`crate::resource::GlobalEntropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking the inner PRNG instance of the source component/resource.
pub trait ForkableInnerRng: EcsEntropySource {
    /// The type of instance that is to be forked from the original source.
    type Output: SeedableEntropySource;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method yields the inner PRNG instance directly as a forked instance.
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableInnerRng};
    /// use bevy_prng::ChaCha8Rng;
    /// use rand_core::RngCore;
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn do_random_action(source: &mut ChaCha8Rng) {
    ///     println!("Random value: {}", source.next_u32());
    /// }
    ///
    /// fn access_source(mut global: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    ///     let mut source = global.fork_inner();
    ///
    ///     do_random_action(&mut source);
    /// }
    /// ```
    fn fork_inner(&mut self) -> Self::Output {
        Self::Output::from_rng(self).unwrap()
    }
}

/// A marker trait for [`crate::component::EntropyComponent`] and [`crate::resource::GlobalEntropy`].
/// This is a sealed trait and cannot be consumed by downstream.
pub trait EcsEntropySource: RngCore + SeedableRng + private::SealedSource {}

mod private {
    pub trait SealedSource {}

    impl<T: super::EcsEntropySource> SealedSource for T {}
}
