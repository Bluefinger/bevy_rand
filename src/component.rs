use core::fmt::Debug;

use crate::{
    seed::RngSeed,
    traits::{
        EcsEntropy, ForkableAsRng, ForkableAsSeed, ForkableInnerRng, ForkableInnerSeed,
        ForkableRng, ForkableSeed,
    },
};
use bevy_ecs::prelude::Component;
#[cfg(feature = "bevy_reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_prng::EntropySource;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect, prelude::ReflectDefault};
use rand_core::{RngCore, SeedableRng, TryRngCore};

#[cfg(feature = "thread_local_entropy")]
use crate::thread_local_entropy::ThreadLocalEntropy;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
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
///    mut q_source: Single<&mut Entropy<WyRand>, (With<Source>, Without<Npc>)>,
/// ) {
///    let mut source = q_source.into_inner();
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
#[derive(Debug, Clone, PartialEq, Eq, Component)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(bound(deserialize = "R: for<'a> Deserialize<'a>"))
)]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(
        Debug,
        PartialEq,
        Component,
        FromReflect,
        Default,
        Clone,
        Serialize,
        Deserialize
    )
)]
#[cfg_attr(
    all(not(feature = "serialize"), feature = "bevy_reflect"),
    reflect(Debug, PartialEq, Component, FromReflect, Default, Clone)
)]
pub struct Entropy<R: EntropySource>(R);

impl<R: EntropySource> Entropy<R> {
    /// Create a new component from an `RngCore` instance.
    #[inline]
    #[must_use]
    pub fn new(rng: R) -> Self {
        Self(rng)
    }

    /// Reseeds the internal `RngCore` instance with a new seed.
    #[inline]
    #[deprecated = "Make use of `RngSeed` component instead for reseeding."]
    pub fn reseed(&mut self, seed: R::Seed) {
        self.0 = R::from_seed(seed);
    }
}

impl<R: EntropySource> Default for Entropy<R> {
    #[inline]
    fn default() -> Self {
        #[cfg(feature = "thread_local_entropy")]
        {
            let mut local =
                ThreadLocalEntropy::new().expect("Unable to source entropy for initialisation");
            Self::from_rng(&mut local)
        }
        #[cfg(not(feature = "thread_local_entropy"))]
        {
            Self::from_os_rng()
        }
    }
}

impl<R: EntropySource> RngCore for Entropy<R> {
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
}

#[cfg(feature = "compat")]
impl<R: EntropySource> rand_core_06::RngCore for Entropy<R> {
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
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core_06::Error> {
        self.0.fill_bytes(dest);
        Ok(())
    }
}

impl<R: EntropySource> SeedableRng for Entropy<R> {
    type Seed = R::Seed;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(R::from_seed(seed))
    }

    #[inline]
    fn from_rng(rng: &mut impl RngCore) -> Self {
        Self::new(R::from_rng(rng))
    }

    #[inline]
    fn try_from_rng<T: TryRngCore>(rng: &mut T) -> Result<Self, T::Error> {
        Ok(Self::new(R::try_from_rng(rng)?))
    }
}

impl<R: EntropySource> EcsEntropy for Entropy<R> {}

impl<R> ForkableRng for Entropy<R>
where
    R: EntropySource,
{
    type Output = Entropy<R>;
}

impl<R> ForkableAsRng for Entropy<R>
where
    R: EntropySource,
{
    type Output<T>
        = Entropy<T>
    where
        T: EntropySource;
}

impl<R> ForkableInnerRng for Entropy<R>
where
    R: EntropySource,
{
    type Output = R;
}

impl<R> ForkableSeed<R> for Entropy<R>
where
    R: EntropySource,
    R::Seed: Send + Sync + Clone,
{
    type Output = RngSeed<R>;
}

impl<R> ForkableAsSeed<R> for Entropy<R>
where
    R: EntropySource,
{
    type Output<T>
        = RngSeed<T>
    where
        T: EntropySource,
        T::Seed: Send + Sync + Clone;
}

impl<R> ForkableInnerSeed<R> for Entropy<R>
where
    R: EntropySource,
    R::Seed: Send + Sync + Clone + AsMut<[u8]> + Default,
{
    type Output = R::Seed;
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use bevy_prng::{ChaCha8Rng, ChaCha12Rng};

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

        let rng1 = format!("{rng1:?}");
        let rng2 = format!("{rng2:?}");

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

    #[cfg(feature = "bevy_reflect")]
    #[test]
    fn type_paths() {
        use bevy_reflect::TypePath;

        assert_eq!(
            "bevy_rand::component::Entropy<bevy_prng::ChaCha8Rng>",
            Entropy::<ChaCha8Rng>::type_path()
        );

        assert_eq!(
            "Entropy<ChaCha8Rng>",
            Entropy::<ChaCha8Rng>::short_type_path()
        );
    }

    #[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
    #[test]
    fn rng_untyped_serialization() {
        use bevy_reflect::{
            FromReflect, TypeRegistry,
            serde::{ReflectDeserializer, ReflectSerializer},
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

    #[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
    #[test]
    fn rng_typed_serialization() {
        use bevy_reflect::{
            FromReflect, GetTypeRegistration, TypeRegistry,
            serde::{TypedReflectDeserializer, TypedReflectSerializer},
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
