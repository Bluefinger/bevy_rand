use bevy_ecs::{
    query::{QuerySingleError, With},
    world::World,
};
use bevy_prng::EntropySource;
use rand_core::SeedableRng;

use crate::{global::GlobalRng, seed::RngSeed};

/// Trait for implementing Forking behaviour for [`EntropySource`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic.
pub trait ForkableRng: EntropySource {
    /// The type of instance that is to be forked from the original source.
    type Output: EntropySource;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method preserves the RNG algorithm between original and forked instances.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_prng::ChaCha8Rng;
    /// use bevy_rand::prelude::{GlobalRng, ForkableRng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: Single<&mut ChaCha8Rng, With<GlobalRng>>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_rng(),
    ///         ));
    /// }
    /// ```
    #[inline]
    fn fork_rng(&mut self) -> Self::Output {
        Self::Output::from_rng(self)
    }
}

/// Trait for implementing Forking behaviour for [`EntropySource`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking between different PRNG algorithm types.
pub trait ForkableAsRng: EntropySource {
    /// The type of instance that is to be forked from the original source.
    type Output<R>: EntropySource
    where
        R: EntropySource;

    /// Fork the original instance to yield a new instance with a generated seed.
    /// This method allows one to specify the RNG algorithm to be used for the forked instance.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_rand::prelude::{ForkableAsRng, GlobalRng};
    /// use bevy_prng::{ChaCha8Rng, ChaCha12Rng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: Single<&mut ChaCha12Rng, With<GlobalRng>>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_as::<ChaCha8Rng>(),
    ///         ));
    /// }
    /// ```
    #[inline]
    fn fork_as<T: EntropySource>(&mut self) -> Self::Output<T> {
        Self::Output::<T>::from_rng(self)
    }
}

/// Trait for implementing forking behaviour for [`EntropySource`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking from an entropy source to a seed component.
pub trait ForkableSeed<S: EntropySource>: EntropySource {
    /// The type of seed component that is to be forked from the original source.
    type Output: SeedSource<S>;

    /// Fork a new seed from the original entropy source.
    /// This method preserves the RNG algorithm between original instance and forked seed.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_prng::ChaCha8Rng;
    /// use bevy_rand::prelude::{ForkableSeed, GlobalRng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: Single<&mut ChaCha8Rng, With<GlobalRng>>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_seed(),
    ///         ));
    /// }
    /// ```
    #[inline]
    fn fork_seed(&mut self) -> Self::Output {
        let mut seed = S::Seed::default();

        self.fill_bytes(seed.as_mut());

        Self::Output::from_seed(seed)
    }
}

/// Trait for implementing Forking behaviour for [`EntropySource`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking from an entropy source to a seed component of a different
/// PRNG algorithm.
pub trait ForkableAsSeed<S: EntropySource>: EntropySource {
    /// The type of seed component that is to be forked from the original source.
    type Output<T>: SeedSource<T>
    where
        T: EntropySource;

    /// Fork a new seed from the original entropy source.
    /// This method allows one to specify the RNG algorithm to be used for the forked seed.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_rand::prelude::{ForkableAsSeed, GlobalRng};
    /// use bevy_prng::{ChaCha8Rng, ChaCha12Rng};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: Single<&mut ChaCha8Rng, With<GlobalRng>>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             global.fork_as_seed::<ChaCha8Rng>(),
    ///         ));
    /// }
    /// ```
    #[inline]
    fn fork_as_seed<T: EntropySource>(&mut self) -> Self::Output<T> {
        let mut seed = T::Seed::default();

        self.fill_bytes(seed.as_mut());

        Self::Output::<T>::from_seed(seed)
    }
}

/// Trait for implementing forking behaviour for [`EntropySource`].
/// Forking creates a new RNG instance using a generated seed from the original source. If the original is seeded with a known
/// seed, this process is deterministic. This trait enables forking from an entropy source to the RNG's seed type.
pub trait ForkableInnerSeed<S: EntropySource>: EntropySource {
    /// The type of seed component that is to be forked from the original source.
    type Output: Send + Sync + Clone + AsMut<[u8]> + Default;

    /// Fork a new seed from the original entropy source.
    /// This method preserves the RNG algorithm between original instance and forked seed.
    /// ```
    /// use bevy_ecs::prelude::*;
    /// use bevy_prng::ChaCha8Rng;
    /// use bevy_rand::prelude::{ForkableInnerSeed, GlobalRng, SeedSource, RngSeed};
    ///
    /// #[derive(Component)]
    /// struct Source;
    ///
    /// fn setup_source(mut commands: Commands, mut global: Single<&mut ChaCha8Rng, With<GlobalRng>>) {
    ///     commands
    ///         .spawn((
    ///             Source,
    ///             RngSeed::<ChaCha8Rng>::from_seed(global.fork_inner_seed()),
    ///         ));
    /// }
    /// ```
    #[inline]
    fn fork_inner_seed(&mut self) -> Self::Output {
        let mut seed = Self::Output::default();

        self.fill_bytes(seed.as_mut());

        seed
    }
}

/// A trait for providing [`crate::seed::RngSeed`] with
/// common initialization strategies. This trait is not object safe and is also a sealed trait.
pub trait SeedSource<R: EntropySource>: private::SealedSeed<R> {
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
        use rand_core::Rng;

        let mut dest = R::Seed::default();

        bevy_prng::ThreadLocalEntropy::get()?.fill_bytes(dest.as_mut());

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
    #[inline]
    fn from_local_entropy() -> Self
    where
        Self: Sized,
    {
        Self::try_from_local_entropy().expect("Unable to source user-space entropy for seeding")
    }

    /// Initialize a [`SeedSource`] from a `seed` value obtained from an
    /// OS/Hardware RNG source.
    fn try_from_os_rng() -> Result<Self, getrandom::Error>
    where
        Self: Sized,
    {
        let mut dest = R::Seed::default();

        getrandom::fill(dest.as_mut())?;

        Ok(Self::from_seed(dest))
    }

    /// Initialize a [`SeedSource`] from a `seed` value obtained from an
    /// OS/Hardware RNG source.
    ///
    /// # Panics
    ///
    /// This method panics if for whatever reason it is unable to source entropy
    /// from an OS/Hardware source.
    #[inline]
    fn from_os_rng() -> Self
    where
        Self: Sized,
    {
        Self::try_from_os_rng().expect("Unable to source os/hardware entropy for seeding")
    }
}

/// Extension trait to allow implementing forking on more types. By default, it is implemented
/// for `&mut World` which sources from [`GlobalRng`] source, though this can be manually implemented for more.
pub trait ForkRngExt {
    /// The Error type returned for the queries used to extract and fork from.
    type Error: core::error::Error;
    /// The Output type for the resulting fork methods. Usually will be a `Result`.
    type Output<Rng>;

    /// Forks an [`EntropySource`] component from the source.
    fn fork_rng<Target: EntropySource>(&mut self) -> Self::Output<Target>;
    /// Forks an [`EntropySource`] component from the source as the given `Target` Rng kind.
    fn fork_as<Source: EntropySource, Target: EntropySource>(&mut self) -> Self::Output<Target>;
}

/// Extension trait to allow implementing forking seeds on more types. By default, it is implemented
/// for `&mut World` which sources from [`GlobalRng`] source, though this can be manually implemented for more.
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

    /// Forks an [`EntropySource`] component from the [`GlobalRng`] source.
    #[inline]
    fn fork_rng<Target: EntropySource>(&mut self) -> Self::Output<Target> {
        self.fork_as::<Target, Target>()
    }

    /// Forks an [`EntropySource`] component from the [`GlobalRng`] source as the given `Target` Rng kind.
    fn fork_as<Source: EntropySource, Target: EntropySource>(&mut self) -> Self::Output<Target> {
        self.query_filtered::<&mut Source, With<GlobalRng>>()
            .single_mut(self)
            .map(|mut global| global.fork_as::<Target>())
    }
}

impl ForkSeedExt for &mut World {
    type Error = QuerySingleError;
    type Output<Rng> = Result<Rng, Self::Error>;

    /// Forks a [`RngSeed`] component from the [`GlobalRng`] source.
    #[inline]
    fn fork_seed<Target: EntropySource>(&mut self) -> Self::Output<RngSeed<Target>> {
        self.fork_as_seed::<Target, Target>()
    }

    /// Forks an [`RngSeed`] component from the [`GlobalRng`] source as the given `Target` Rng kind.
    fn fork_as_seed<Source: EntropySource, Target: EntropySource>(
        &mut self,
    ) -> Self::Output<RngSeed<Target>> {
        self.query_filtered::<&mut Source, With<GlobalRng>>()
            .single_mut(self)
            .map(|mut global| global.fork_as_seed::<Target>())
    }

    /// Forks a new Seed from the [`GlobalRng`] source.
    fn fork_inner_seed<Target: EntropySource>(&mut self) -> Self::Output<Target::Seed> {
        self.query_filtered::<&mut Target, With<GlobalRng>>()
            .single_mut(self)
            .map(|mut global| global.fork_inner_seed())
    }
}

impl<R> ForkableRng for R
where
    R: EntropySource,
{
    type Output = R;
}

impl<R> ForkableAsRng for R
where
    R: EntropySource,
{
    type Output<T>
        = T
    where
        T: EntropySource;
}

impl<R> ForkableSeed<R> for R
where
    R: EntropySource,
    R::Seed: Send + Sync + Clone,
{
    type Output = RngSeed<R>;
}

impl<R> ForkableAsSeed<R> for R
where
    R: EntropySource,
{
    type Output<T>
        = RngSeed<T>
    where
        T: EntropySource,
        T::Seed: Send + Sync + Clone;
}

impl<R> ForkableInnerSeed<R> for R
where
    R: EntropySource,
    R::Seed: Send + Sync + Clone + AsMut<[u8]> + Default,
{
    type Output = R::Seed;
}

// /// A marker trait for [`crate::component::Entropy`].
// /// This is a sealed trait and cannot be consumed by downstream.
// pub trait EcsEntropy: RngCore + SeedableRng + private::SealedSource {}

mod private {
    use super::{EntropySource, SeedSource};

    pub trait SealedSeed<R>
    where
        R: EntropySource,
    {
    }

    impl<R, T> SealedSeed<R> for T
    where
        T: SeedSource<R>,
        R: EntropySource,
        R::Seed: Send + Sync + Clone,
    {
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use bevy_prng::{ChaCha8Rng, ChaCha12Rng};

    use super::*;

    #[test]
    fn forking() {
        let mut rng1 = ChaCha8Rng::default();

        let rng2 = rng1.fork_rng();

        assert_ne!(rng1, rng2, "forked Entropys should not match each other");
    }

    #[test]
    fn forking_as() {
        let mut rng1 = ChaCha12Rng::default();

        let rng2 = rng1.fork_as::<ChaCha8Rng>();

        let rng1 = format!("{rng1:?}");
        let rng2 = format!("{rng2:?}");

        assert_ne!(&rng1, &rng2, "forked Entropys should not match each other");
    }

    #[cfg(feature = "bevy_reflect")]
    #[test]
    fn type_paths() {
        use bevy_reflect::TypePath;

        assert_eq!("bevy_prng::ChaCha8Rng", ChaCha8Rng::type_path());

        assert_eq!("ChaCha8Rng", ChaCha8Rng::short_type_path());
    }
}
