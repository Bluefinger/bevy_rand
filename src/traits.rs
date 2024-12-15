use bevy_prng::EntropySource;
use rand_core::{RngCore, SeedableRng};

/// Trait for implementing Forking behaviour for [`crate::component::Entropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic.
pub trait ForkableRng: EcsEntropy {
    /// The type of instance that is to be forked from the original source.
    type Output: EcsEntropy;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method preserves the RNG algorithm between original and forked instances.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_prng::ChaCha8Rng;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableRng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: GlobalEntropy<ChaCha8Rng>) {
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

/// Trait for implementing Forking behaviour for [`crate::component::Entropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking between different PRNG algorithm types.
pub trait ForkableAsRng: EcsEntropy {
    /// The type of instance that is to be forked from the original source.
    type Output<R>: EcsEntropy
    where
        R: EntropySource;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method allows one to specify the RNG algorithm to be used for the forked instance.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableAsRng};
    /// use bevy_prng::{ChaCha8Rng, ChaCha12Rng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: GlobalEntropy<ChaCha12Rng>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_as::<ChaCha8Rng>(),
    ///         ));
    /// }
    /// ```
    fn fork_as<T: EntropySource>(&mut self) -> Self::Output<T> {
        Self::Output::<_>::from_rng(self).unwrap()
    }
}

/// Trait for implementing Forking behaviour for [`crate::component::Entropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking the inner PRNG instance of the source component/resource.
pub trait ForkableInnerRng: EcsEntropy {
    /// The type of instance that is to be forked from the original source.
    type Output: EntropySource;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method yields the inner PRNG instance directly as a forked instance.
    /// ```
    /// use bevy_ecs::prelude::*;
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
    /// fn access_source(mut global: GlobalEntropy<ChaCha8Rng>) {
    ///     let mut source = global.fork_inner();
    ///
    ///     do_random_action(&mut source);
    /// }
    /// ```
    fn fork_inner(&mut self) -> Self::Output {
        Self::Output::from_rng(self).unwrap()
    }
}

/// Trait for implementing forking behaviour for [`crate::component::Entropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking from an entropy source to a seed component.
pub trait ForkableSeed<S: EntropySource>: EcsEntropy
where
    S::Seed: Send + Sync + Clone,
{
    /// The type of seed component that is to be forked from the original source.
    type Output: SeedSource<S>;

    /// Fork a new seed from the original entropy source.
    /// This method preserves the RNG algorithm between original instance and forked seed.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_prng::ChaCha8Rng;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableSeed};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: GlobalEntropy<ChaCha8Rng>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_seed(),
    ///         ));
    /// }
    /// ```
    fn fork_seed(&mut self) -> Self::Output {
        let mut seed = S::Seed::default();

        self.fill_bytes(seed.as_mut());

        Self::Output::from_seed(seed)
    }
}

/// Trait for implementing Forking behaviour for [`crate::component::Entropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking from an entropy source to a seed component of a different
/// PRNG algorithm.
pub trait ForkableAsSeed<S: EntropySource>: EcsEntropy {
    /// The type of seed component that is to be forked from the original source.
    type Output<T>: SeedSource<T>
    where
        T: EntropySource,
        T::Seed: Send + Sync + Clone;

    /// Fork a new seed from the original entropy source.
    /// This method allows one to specify the RNG algorithm to be used for the forked seed.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableAsSeed};
    /// use bevy_prng::{ChaCha8Rng, ChaCha12Rng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: GlobalEntropy<ChaCha12Rng>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_as_seed::<ChaCha8Rng>(),
    ///         ));
    /// }
    /// ```
    fn fork_as_seed<T: EntropySource>(&mut self) -> Self::Output<T>
    where
        T::Seed: Send + Sync + Clone,
    {
        let mut seed = T::Seed::default();

        self.fill_bytes(seed.as_mut());

        Self::Output::<T>::from_seed(seed)
    }
}

/// Trait for implementing forking behaviour for [`crate::component::Entropy`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking from an entropy source to the RNG's seed type.
pub trait ForkableInnerSeed<S: EntropySource>: EcsEntropy
where
    S::Seed: Send + Sync + Clone + AsMut<[u8]> + Default,
{
    /// The type of seed component that is to be forked from the original source.
    type Output: Send + Sync + Clone + AsMut<[u8]> + Default;

    /// Fork a new seed from the original entropy source.
    /// This method preserves the RNG algorithm between original instance and forked seed.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_prng::ChaCha8Rng;
    /// use bevy_rand::prelude::{GlobalEntropy, ForkableInnerSeed, SeedSource, RngSeed};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: GlobalEntropy<ChaCha8Rng>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             RngSeed::<ChaCha8Rng>::from_seed(global.fork_inner_seed()),
    ///         ));
    /// }
    /// ```
    fn fork_inner_seed(&mut self) -> Self::Output {
        let mut seed = Self::Output::default();

        self.fill_bytes(seed.as_mut());

        seed
    }
}

/// A trait for providing [`crate::seed::RngSeed`] with
/// common initialization strategies. This trait is not object safe and is also a sealed trait.
pub trait SeedSource<R: EntropySource>: private::SealedSeed<R>
where
    R::Seed: Send + Sync + Clone,
{
    /// Initialize a [`SeedSource`] from a given `seed` value.
    fn from_seed(seed: R::Seed) -> Self;

    /// Returns a reference of the seed value.
    fn get_seed(&self) -> &R::Seed;

    /// Returns a cloned instance of the seed value.
    fn clone_seed(&self) -> R::Seed;

    /// Initialize a [`SeedSource`] from a `seed` value obtained from a
    /// OS-level or user-space RNG source.
    fn from_entropy() -> Self
    where
        Self: Sized,
    {
        let mut dest = R::Seed::default();

        #[cfg(feature = "thread_local_entropy")]
        {
            use crate::thread_local_entropy::ThreadLocalEntropy;

            ThreadLocalEntropy::new().fill_bytes(dest.as_mut());
        }
        #[cfg(not(feature = "thread_local_entropy"))]
        {
            use getrandom::getrandom;

            getrandom(seed.as_mut()).expect("Unable to source entropy for seeding");
        }

        Self::from_seed(dest)
    }
}

/// A marker trait for [`crate::component::Entropy`].
/// This is a sealed trait and cannot be consumed by downstream.
pub trait EcsEntropy: RngCore + SeedableRng + private::SealedSource {}

mod private {
    use super::{EcsEntropy, EntropySource, SeedSource};

    pub trait SealedSource {}
    pub trait SealedSeed<R>
    where
        R: EntropySource,
    {
    }

    impl<T> SealedSource for T where T: EcsEntropy {}
    impl<R, T> SealedSeed<R> for T
    where
        T: SeedSource<R>,
        R: EntropySource,
        R::Seed: Send + Sync + Clone,
    {
    }
}
