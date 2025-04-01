use bevy_ecs::{
    query::{QuerySingleError, With},
    world::World,
};
use bevy_prng::EntropySource;
use rand_core::{OsRng, RngCore, SeedableRng, TryRngCore};

use crate::{global::Global, prelude::Entropy, seed::RngSeed};

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
        Self::Output::from_rng(self)
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
        Self::Output::<_>::from_rng(self)
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
        Self::Output::from_rng(self)
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
    /// user-space RNG source.
    ///
    /// # Panics
    ///
    /// This method panics if for whatever reason it is unable to source entropy
    /// from the User-Space/OS/Hardware source.
    #[deprecated = "use either `SeedSource::from_local_entropy` or `SeedSource::from_os_rng` instead"]
    fn from_entropy() -> Self
    where
        Self: Sized,
    {
        #[cfg(feature = "thread_local_entropy")]
        {
            Self::from_local_entropy()
        }
        #[cfg(not(feature = "thread_local_entropy"))]
        {
            Self::from_os_rng()
        }
    }

    /// Initialize a [`SeedSource`] from a `seed` value obtained from a
    /// user-space RNG source. This is usually much, much faster than sourcing
    /// entropy from OS/Hardware sources.
    #[cfg(feature = "thread_local_entropy")]
    fn try_from_local_entropy() -> Result<Self, std::thread::AccessError>
    where
        Self: Sized,
    {
        let mut dest = R::Seed::default();

        crate::thread_local_entropy::ThreadLocalEntropy::new()?.fill_bytes(dest.as_mut());

        Ok(Self::from_seed(dest))
    }

    /// Initialize a [`SeedSource`] from a `seed` value obtained from a
    /// user-space RNG source. This is usually much, much faster than sourcing
    /// entropy from OS/Hardware sources.
    ///
    /// # Panics
    ///
    /// This method panics if for whatever reason it is unable to source entropy
    /// from the User-Space source.
    #[cfg(feature = "thread_local_entropy")]
    fn from_local_entropy() -> Self
    where
        Self: Sized,
    {
        Self::try_from_local_entropy().expect("Unable to source user-space entropy for seeding")
    }

    /// Initialize a [`SeedSource`] from a `seed` value obtained from an
    /// OS/Hardware RNG source.
    fn try_from_os_rng() -> Result<Self, rand_core::OsError>
    where
        Self: Sized,
    {
        let mut dest = R::Seed::default();

        OsRng.try_fill_bytes(dest.as_mut())?;

        Ok(Self::from_seed(dest))
    }

    /// Initialize a [`SeedSource`] from a `seed` value obtained from an
    /// OS/Hardware RNG source.
    ///
    /// # Panics
    ///
    /// This method panics if for whatever reason it is unable to source entropy
    /// from an OS/Hardware source.
    fn from_os_rng() -> Self
    where
        Self: Sized,
    {
        Self::try_from_os_rng().expect("Unable to source os/hardware entropy for seeding")
    }
}

/// Extension trait to allow implementing forking on more types. By default, it is implemented
/// for `&mut World` which sources from [`Global`] source, though this can be manually implemented for more.
pub trait ForkRngExt {
    /// The Error type returned for the queries used to extract and fork from.
    type Error: core::error::Error;
    /// The Output type for the resulting fork methods. Usually will be a `Result`.
    type Output<Rng>;

    /// Forks an [`Entropy`] component from the source.
    fn fork_rng<Target: EntropySource>(&mut self) -> Self::Output<Entropy<Target>>;
    /// Forks an [`Entropy`] component from the source as the given `Target` Rng kind.
    fn fork_as<Source: EntropySource, Target: EntropySource>(
        &mut self,
    ) -> Self::Output<Entropy<Target>>;
    /// Forks the inner Rng from the source.
    fn fork_inner<Target: EntropySource>(&mut self) -> Self::Output<Target>;
}

/// Extension trait to allow implementing forking seeds on more types. By default, it is implemented
/// for `&mut World` which sources from [`Global`] source, though this can be manually implemented for more.
pub trait ForkSeedExt {
    /// The Error type returned for the queries used to extract and fork from.
    type Error: core::error::Error;
    /// The Output type for the resulting fork methods. Usually will be a `Result`.
    type Output<Rng>;

    /// Forks a [`RngSeed`] component from the source.
    fn fork_seed<Target: EntropySource>(&mut self) -> Self::Output<RngSeed<Target>>;
    /// Forks an [`RngSeed`] component from the source as the given `Target` Rng kind.
    fn fork_as_seed<Source: EntropySource, Target: EntropySource>(
        &mut self,
    ) -> Self::Output<RngSeed<Target>>;
    /// Forks a new Seed from the source.
    fn fork_inner_seed<Target: EntropySource>(&mut self) -> Self::Output<Target::Seed>;
}

impl ForkRngExt for &mut World {
    type Error = QuerySingleError;
    type Output<Rng> = Result<Rng, Self::Error>;

    /// Forks an [`Entropy`] component from the [`Global`] source.
    fn fork_rng<Target: EntropySource>(&mut self) -> Self::Output<Entropy<Target>> {
        self.fork_as::<Target, Target>()
    }

    /// Forks an [`Entropy`] component from the [`Global`] source as the given `Target` Rng kind.
    fn fork_as<Source: EntropySource, Target: EntropySource>(
        &mut self,
    ) -> Self::Output<Entropy<Target>> {
        self.query_filtered::<&mut Entropy<Source>, With<Global>>()
            .single_mut(self)
            .map(|mut global| global.fork_as::<Target>())
    }

    /// Forks the inner Rng from the [`Global`] source.
    fn fork_inner<Target: EntropySource>(&mut self) -> Self::Output<Target> {
        self.query_filtered::<&mut Entropy<Target>, With<Global>>()
            .single_mut(self)
            .map(|mut global| global.fork_inner())
    }
}

impl ForkSeedExt for &mut World {
    type Error = QuerySingleError;
    type Output<Rng> = Result<Rng, Self::Error>;

    /// Forks a [`RngSeed`] component from the [`Global`] source.
    fn fork_seed<Target: EntropySource>(&mut self) -> Self::Output<RngSeed<Target>> {
        self.fork_as_seed::<Target, Target>()
    }

    /// Forks an [`RngSeed`] component from the [`Global`] source as the given `Target` Rng kind.
    fn fork_as_seed<Source: EntropySource, Target: EntropySource>(
        &mut self,
    ) -> Self::Output<RngSeed<Target>> {
        self.query_filtered::<&mut Entropy<Source>, With<Global>>()
            .single_mut(self)
            .map(|mut global| global.fork_as_seed::<Target>())
    }

    /// Forks a new Seed from the [`Global`] source.
    fn fork_inner_seed<Target: EntropySource>(&mut self) -> Self::Output<Target::Seed> {
        self.query_filtered::<&mut Entropy<Target>, With<Global>>()
            .single_mut(self)
            .map(|mut global| global.fork_inner_seed())
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
