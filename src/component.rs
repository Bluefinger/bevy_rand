use std::fmt::Debug;

use crate::{resource::GlobalEntropy, traits::SeedableEntropySource};
use bevy::prelude::{Component, Mut, Reflect, ReflectComponent, ReflectFromReflect, ResMut};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "thread_local_entropy")]
use crate::thread_local_entropy::ThreadLocalEntropy;

#[cfg(feature = "serialize")]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// An [`EntropyComponent`] that wraps a random number generator that implements
/// [`RngCore`] & [`SeedableRng`].
///
/// ## Creating new [`EntropyComponent`]s.
///
/// You can creates a new [`EntropyComponent`] directly from anything that implements
/// [`RngCore`] or provides a mut reference to [`RngCore`], such as [`ResMut`] or a
/// [`Component`], or from a [`RngCore`] source directly.
///
/// ## Examples
///
/// Randomised Component:
/// ```
/// use bevy::prelude::*;
/// use bevy_rand::prelude::*;
/// use bevy_prng::ChaCha8Rng;
///
/// #[derive(Component)]
/// struct Source;
///
/// fn setup_source(mut commands: Commands) {
///     commands
///         .spawn((
///             Source,
///             EntropyComponent::<ChaCha8Rng>::default(),
///         ));
/// }
/// ```
///
/// Seeded from a resource:
/// ```
/// use bevy::prelude::*;
/// use bevy_rand::prelude::*;
/// use bevy_prng::ChaCha8Rng;
///
/// #[derive(Component)]
/// struct Source;
///
/// fn setup_source(mut commands: Commands, mut global: ResMut<GlobalEntropy<ChaCha8Rng>>) {
///     commands
///         .spawn((
///             Source,
///             EntropyComponent::from(&mut global),
///         ));
/// }
/// ```
///
/// Seeded from a component:
/// ```
/// use bevy::prelude::*;
/// use bevy_rand::prelude::*;
/// use bevy_prng::ChaCha8Rng;
///
/// #[derive(Component)]
/// struct Npc;
/// #[derive(Component)]
/// struct Source;
///
/// fn setup_npc_from_source(
///    mut commands: Commands,
///    mut q_source: Query<&mut EntropyComponent<ChaCha8Rng>, (With<Source>, Without<Npc>)>,
/// ) {
///    let mut source = q_source.single_mut();
///
///    for _ in 0..2 {
///        commands
///            .spawn((
///                Npc,
///                EntropyComponent::from(&mut source)
///            ));
///    }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
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
pub struct EntropyComponent<R: SeedableEntropySource + 'static>(R);

impl<R: SeedableEntropySource + 'static> EntropyComponent<R> {
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

impl<R: SeedableEntropySource + 'static> Default for EntropyComponent<R> {
    fn default() -> Self {
        Self::from_entropy()
    }
}

impl<R: SeedableEntropySource + 'static> RngCore for EntropyComponent<R> {
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

impl<R: SeedableEntropySource + 'static> SeedableRng for EntropyComponent<R> {
    type Seed = R::Seed;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(R::from_seed(seed))
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
        let mut seed = Self::Seed::default();

        // Source entropy from thread local user-space RNG instead of
        // system entropy source to reduce overhead when creating many
        // rng instances for many entities at once.
        ThreadLocalEntropy::new().fill_bytes(seed.as_mut());

        Self::from_seed(seed)
    }
}

impl<R: SeedableEntropySource + 'static> From<R> for EntropyComponent<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: SeedableEntropySource + 'static> From<&mut EntropyComponent<R>> for EntropyComponent<R> {
    fn from(rng: &mut EntropyComponent<R>) -> Self {
        Self::from_rng(rng).unwrap()
    }
}

impl<R: SeedableEntropySource + 'static> From<&mut Mut<'_, EntropyComponent<R>>>
    for EntropyComponent<R>
{
    fn from(rng: &mut Mut<'_, EntropyComponent<R>>) -> Self {
        Self::from(rng.as_mut())
    }
}

impl<R: SeedableEntropySource + 'static> From<&mut ResMut<'_, GlobalEntropy<R>>>
    for EntropyComponent<R>
{
    fn from(rng: &mut ResMut<'_, GlobalEntropy<R>>) -> Self {
        Self::from_rng(rng.as_mut()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use bevy::reflect::TypePath;
    use bevy_prng::ChaCha8Rng;

    use super::*;

    #[test]
    fn forking() {
        let mut rng1 = EntropyComponent::<ChaCha8Rng>::default();

        let rng2 = EntropyComponent::from(&mut rng1);

        assert_ne!(
            rng1, rng2,
            "forked EntropyComponents should not match each other"
        );
    }

    #[test]
    fn type_paths() {
        assert_eq!(
            "bevy_rand::component::EntropyComponent<bevy_prng::ChaCha8Rng>",
            EntropyComponent::<ChaCha8Rng>::type_path()
        );

        assert_eq!(
            "EntropyComponent<ChaCha8Rng>",
            EntropyComponent::<ChaCha8Rng>::short_type_path()
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn rng_untyped_serialization() {
        use bevy::reflect::{
            serde::{ReflectSerializer, UntypedReflectDeserializer},
            TypeRegistryInternal,
        };
        use ron::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistryInternal::default();
        registry.register::<EntropyComponent<ChaCha8Rng>>();

        let mut val: EntropyComponent<ChaCha8Rng> = EntropyComponent::from_seed([7; 32]);

        // Modify the state of the RNG instance
        val.next_u32();

        let ser = ReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(
            &serialized,
            "{\"bevy_rand::component::EntropyComponent<bevy_prng::ChaCha8Rng>\":(((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)))}"
        );

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = UntypedReflectDeserializer::new(&registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        let mut dynamic = value.take::<EntropyComponent<ChaCha8Rng>>().unwrap();

        // The two instances should be the same
        assert_eq!(
            val, dynamic,
            "The deserialized EntropyComponent should equal the original"
        );
        // They should output the same numbers, as no state is lost between serialization and deserialization.
        assert_eq!(
            val.next_u32(),
            dynamic.next_u32(),
            "The deserialized EntropyComponent should have the same output as original"
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn rng_typed_serialization() {
        use bevy::reflect::{
            serde::{TypedReflectDeserializer, TypedReflectSerializer},
            GetTypeRegistration, TypeRegistryInternal,
        };
        use ron::ser::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistryInternal::default();
        registry.register::<EntropyComponent<ChaCha8Rng>>();

        let registered_type = EntropyComponent::<ChaCha8Rng>::get_type_registration();

        let mut val = EntropyComponent::<ChaCha8Rng>::from_seed([7; 32]);

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

        let mut dynamic = value.take::<EntropyComponent<ChaCha8Rng>>().unwrap();

        // The two instances should be the same
        assert_eq!(
            val, dynamic,
            "The deserialized EntropyComponent should equal the original"
        );
        // They should output the same numbers, as no state is lost between serialization and deserialization.
        assert_eq!(
            val.next_u32(),
            dynamic.next_u32(),
            "The deserialized EntropyComponent should have the same output as original"
        );
    }
}
