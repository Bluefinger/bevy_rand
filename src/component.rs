use std::fmt::Debug;

use crate::{
    seed::RngSeed,
    traits::{
        EcsEntropy, ForkableAsRng, ForkableAsSeed, ForkableInnerRng, ForkableInnerSeed,
        ForkableRng, ForkableSeed,
    },
};
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_prng::EntropySource;
use bevy_reflect::{Reflect, ReflectFromReflect};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "thread_local_entropy")]
use crate::thread_local_entropy::ThreadLocalEntropy;

#[cfg(feature = "serialize")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

#[cfg(feature = "serialize")]
use serde::Deserialize;

/// An [`Entropy`] that wraps a random number generator that implements
/// [`RngCore`] & [`SeedableRng`].
///
/// ## Creating new [`Entropy`]s.
///
/// You can creates a new [`Entropy`] directly from anything that implements
/// [`RngCore`] or provides a mut reference to [`RngCore`], such as a
/// [`Component`], or from a [`RngCore`] source directly.
///
/// ## Examples
///
/// Randomised Component:
/// ```
/// use bevy_ecs::prelude::*;
/// use bevy_prng::WyRand;
/// use bevy_rand::prelude::Entropy;
///
/// #[derive(Component)]
/// struct Source;
///
/// fn setup_source(mut commands: Commands) {
///     commands
///         .spawn((
///             Source,
///             Entropy::<WyRand>::default(),
///         ));
/// }
/// ```
///
/// Seeded from a resource:
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
///
/// Seeded from a component:
/// ```
/// use bevy_ecs::prelude::*;
/// use bevy_prng::WyRand;
/// use bevy_rand::prelude::{Entropy, ForkableRng};
///
/// #[derive(Component)]
/// struct Npc;
/// #[derive(Component)]
/// struct Source;
///
/// fn setup_npc_from_source(
///    mut commands: Commands,
///    mut q_source: Query<&mut Entropy<WyRand>, (With<Source>, Without<Npc>)>,
/// ) {
///    let mut source = q_source.single_mut();
///
///    for _ in 0..2 {
///        commands
///            .spawn((
///                Npc,
///                source.fork_rng()
///            ));
///    }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
#[cfg_attr(
    feature = "serialize",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(
    feature = "serialize",
    serde(bound(deserialize = "R: for<'a> Deserialize<'a>"))
)]
#[cfg_attr(
    all(feature = "serialize"),
    reflect(Debug, PartialEq, Component, FromReflect, Serialize, Deserialize)
)]
#[cfg_attr(
    all(not(feature = "serialize")),
    reflect(Debug, PartialEq, Component, FromReflect)
)]
pub struct Entropy<R: EntropySource + 'static>(R);

impl<R: EntropySource + 'static> Entropy<R> {
    /// Create a new component from an `RngCore` instance.
    #[inline]
    #[must_use]
    pub fn new(rng: R) -> Self {
        Self(rng)
    }

    /// Reseeds the internal `RngCore` instance with a new seed.
    #[inline]
    pub fn reseed(&mut self, seed: R::Seed) {
        self.0 = R::from_seed(seed);
    }
}

impl<R: EntropySource + 'static> Default for Entropy<R> {
    #[inline]
    fn default() -> Self {
        Self::from_entropy()
    }
}

impl<R: EntropySource + 'static> RngCore for Entropy<R> {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl<R: EntropySource + 'static> SeedableRng for Entropy<R> {
    type Seed = R::Seed;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(R::from_seed(seed))
    }

    #[inline]
    fn from_rng<S: RngCore>(rng: S) -> Result<Self, rand_core::Error> {
        R::from_rng(rng).map(Self::new)
    }

    /// Creates a new instance of the RNG seeded via [`ThreadLocalEntropy`]. This method is the recommended way
    /// to construct non-deterministic PRNGs since it is convenient and secure. It overrides the standard
    /// [`SeedableRng::from_entropy`] method while the `thread_local_entropy` feature is enabled.
    ///
    /// # Panics
    ///
    /// If [`ThreadLocalEntropy`] cannot get initialised because `getrandom` is unable to provide secure entropy,
    /// this method will panic.
    #[cfg(feature = "thread_local_entropy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "thread_local_entropy")))]
    fn from_entropy() -> Self {
        // This operation should never yield Err on any supported PRNGs
        Self::from_rng(ThreadLocalEntropy::new()).unwrap()
    }
}

impl<R: EntropySource + 'static> EcsEntropy for Entropy<R> {}

impl<R> ForkableRng for Entropy<R>
where
    R: EntropySource + 'static,
{
    type Output = Entropy<R>;
}

impl<R> ForkableAsRng for Entropy<R>
where
    R: EntropySource + 'static,
{
    type Output<T>
        = Entropy<T>
    where
        T: EntropySource;
}

impl<R> ForkableInnerRng for Entropy<R>
where
    R: EntropySource + 'static,
{
    type Output = R;
}

impl<R> ForkableSeed<R> for Entropy<R>
where
    R: EntropySource + 'static,
    R::Seed: Send + Sync + Clone,
{
    type Output = RngSeed<R>;
}

impl<R> ForkableAsSeed<R> for Entropy<R>
where
    R: EntropySource + 'static,
{
    type Output<T>
        = RngSeed<T>
    where
        T: EntropySource,
        T::Seed: Send + Sync + Clone;
}

impl<R> ForkableInnerSeed<R> for Entropy<R>
where
    R: EntropySource + 'static,
    R::Seed: Send + Sync + Clone + AsMut<[u8]> + Default,
{
    type Output = R::Seed;
}

#[cfg(test)]
mod tests {
    use bevy_prng::{ChaCha12Rng, ChaCha8Rng};
    use bevy_reflect::TypePath;

    use super::*;

    #[test]
    fn forking() {
        let mut rng1 = Entropy::<ChaCha8Rng>::default();

        let rng2 = rng1.fork_rng();

        assert_ne!(rng1, rng2, "forked Entropys should not match each other");
    }

    #[test]
    fn forking_as() {
        let mut rng1 = Entropy::<ChaCha12Rng>::default();

        let rng2 = rng1.fork_as::<ChaCha8Rng>();

        let rng1 = format!("{:?}", rng1);
        let rng2 = format!("{:?}", rng2);

        assert_ne!(&rng1, &rng2, "forked Entropys should not match each other");
    }

    #[test]
    fn forking_inner() {
        let mut rng1 = Entropy::<ChaCha8Rng>::default();

        let rng2 = rng1.fork_inner();

        assert_ne!(
            rng1.0, rng2,
            "forked ChaCha8Rngs should not match each other"
        );
    }

    #[test]
    fn type_paths() {
        assert_eq!(
            "bevy_rand::component::Entropy<bevy_prng::ChaCha8Rng>",
            Entropy::<ChaCha8Rng>::type_path()
        );

        assert_eq!(
            "Entropy<ChaCha8Rng>",
            Entropy::<ChaCha8Rng>::short_type_path()
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn rng_untyped_serialization() {
        use bevy_reflect::{
            serde::{ReflectDeserializer, ReflectSerializer},
            FromReflect, TypeRegistry,
        };
        use ron::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistry::default();
        registry.register::<Entropy<ChaCha8Rng>>();

        let mut val: Entropy<ChaCha8Rng> = Entropy::from_seed([7; 32]);

        // Modify the state of the RNG instance
        val.next_u32();

        let ser = ReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(
            &serialized,
            "{\"bevy_rand::component::Entropy<bevy_prng::ChaCha8Rng>\":(((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)))}"
        );

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = ReflectDeserializer::new(&registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        let mut dynamic = Entropy::<ChaCha8Rng>::take_from_reflect(value).unwrap();

        // The two instances should be the same
        assert_eq!(
            val, dynamic,
            "The deserialized Entropy should equal the original"
        );
        // They should output the same numbers, as no state is lost between serialization and deserialization.
        assert_eq!(
            val.next_u32(),
            dynamic.next_u32(),
            "The deserialized Entropy should have the same output as original"
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn rng_typed_serialization() {
        use bevy_reflect::{
            serde::{TypedReflectDeserializer, TypedReflectSerializer},
            FromReflect, GetTypeRegistration, TypeRegistry,
        };
        use ron::ser::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistry::default();
        registry.register::<Entropy<ChaCha8Rng>>();

        let registered_type = Entropy::<ChaCha8Rng>::get_type_registration();

        let mut val = Entropy::<ChaCha8Rng>::from_seed([7; 32]);

        // Modify the state of the RNG instance
        val.next_u32();

        let ser = TypedReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(
            &serialized,
            "(((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)))"
        );

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = TypedReflectDeserializer::new(&registered_type, &registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        let mut dynamic = Entropy::<ChaCha8Rng>::take_from_reflect(value).unwrap();

        // The two instances should be the same
        assert_eq!(
            val, dynamic,
            "The deserialized Entropy should equal the original"
        );
        // They should output the same numbers, as no state is lost between serialization and deserialization.
        assert_eq!(
            val.next_u32(),
            dynamic.next_u32(),
            "The deserialized Entropy should have the same output as original"
        );
    }
}
