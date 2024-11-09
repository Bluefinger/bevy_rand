use std::fmt::Debug;

use crate::{
    component::EntropyComponent,
    seed::RngSeed,
    traits::{
        EcsEntropySource, ForkableAsRng, ForkableAsSeed, ForkableInnerRng, ForkableInnerSeed,
        ForkableRng, ForkableSeed,
    },
};
use bevy::{
    prelude::{Reflect, ReflectFromReflect, ReflectFromWorld, ReflectResource, Resource},
    reflect::Typed,
};
use bevy_prng::SeedableEntropySource;
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "thread_local_entropy")]
use crate::thread_local_entropy::ThreadLocalEntropy;

#[cfg(feature = "serialize")]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// A Global [`RngCore`] instance, meant for use as a Resource. Gets
/// created automatically with [`crate::plugin::EntropyPlugin`], or
/// can be created and added manually.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use bevy_prng::ChaCha8Rng;
/// use bevy_rand::prelude::GlobalEntropy;
/// use rand_core::RngCore;
///
/// fn print_random_value(mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
///   println!("Random value: {}", rng.next_u32());
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[cfg_attr(
    feature = "serialize",
    reflect(
        Debug,
        PartialEq,
        Resource,
        FromReflect,
        Serialize,
        Deserialize,
        FromWorld
    )
)]
#[cfg_attr(
    not(feature = "serialize"),
    reflect(Debug, PartialEq, Resource, FromReflect, FromWorld)
)]
#[cfg_attr(
    feature = "serialize",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(
    feature = "serialize",
    serde(bound(deserialize = "R: for<'a> Deserialize<'a>, R::Seed: for<'a> Deserialize<'a>"))
)]
#[cfg_attr(feature = "serialize", reflect(where R::Seed: PartialEq + Debug + Sync + Send + Clone + Serialize + Typed + for<'a> Deserialize<'a>))]
#[cfg_attr(not(feature = "serialize"), reflect(where R::Seed: PartialEq + Debug + Sync + Send + Clone + Typed))]
pub struct GlobalEntropy<R: SeedableEntropySource + 'static> {
    seed: R::Seed,
    rng: R,
}

impl<R: SeedableEntropySource + 'static> GlobalEntropy<R>
where
    R::Seed: Clone,
{
    #[inline]
    #[must_use]
    fn new(seed: R::Seed) -> Self {
        Self {
            seed: seed.clone(),
            rng: R::from_seed(seed),
        }
    }

    /// Reseeds the internal `RngCore` instance with a new seed.
    #[inline]
    pub fn reseed(&mut self, seed: R::Seed) {
        self.seed = seed.clone();
        self.rng = R::from_seed(seed);
    }

    /// Get a reference to the initial seed
    #[inline]
    pub fn get_seed(&self) -> &R::Seed {
        &self.seed
    }
}

impl<R: SeedableEntropySource + 'static> Default for GlobalEntropy<R>
where
    R::Seed: Clone,
{
    #[inline]
    fn default() -> Self {
        Self::from_entropy()
    }
}

impl<R: SeedableEntropySource + 'static> RngCore for GlobalEntropy<R> {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.rng.try_fill_bytes(dest)
    }
}

impl<R: SeedableEntropySource + 'static> SeedableRng for GlobalEntropy<R>
where
    R::Seed: Clone,
{
    type Seed = R::Seed;

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
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
        let mut seed = R::Seed::default();

        ThreadLocalEntropy::new().fill_bytes(seed.as_mut());

        Self::new(seed)
    }
}

impl<R: SeedableEntropySource + 'static> EcsEntropySource for GlobalEntropy<R> where R::Seed: Clone {}

impl<R> ForkableRng for GlobalEntropy<R>
where
    R: SeedableEntropySource + 'static,
    R::Seed: Clone,
{
    type Output = EntropyComponent<R>;
}

impl<R> ForkableAsRng for GlobalEntropy<R>
where
    R: SeedableEntropySource + 'static,
    R::Seed: Clone,
{
    type Output<T> = EntropyComponent<T> where T: SeedableEntropySource;
}

impl<R> ForkableInnerRng for GlobalEntropy<R>
where
    R: SeedableEntropySource + 'static,
    R::Seed: Clone,
{
    type Output = R;
}

impl<R> ForkableSeed<R> for GlobalEntropy<R>
where
    R: SeedableEntropySource + 'static,
    R::Seed: Send + Sync + Clone,
{
    type Output = RngSeed<R>;
}

impl<R> ForkableAsSeed<R> for GlobalEntropy<R>
where
    R: SeedableEntropySource + 'static,
    R::Seed: Clone,
{
    type Output<T> = RngSeed<T> where T: SeedableEntropySource, T::Seed: Send + Sync + Clone;
}

impl<R> ForkableInnerSeed<R> for GlobalEntropy<R>
where
    R: SeedableEntropySource + 'static,
    R::Seed: Send + Sync + Clone + AsMut<[u8]> + Default,
{
    type Output = R::Seed;
}

#[cfg(test)]
mod tests {
    use bevy::reflect::TypePath;
    use bevy_prng::{ChaCha12Rng, ChaCha8Rng, WyRand};

    use super::*;

    #[test]
    fn type_paths() {
        assert_eq!(
            "bevy_rand::resource::GlobalEntropy<bevy_prng::ChaCha8Rng>",
            GlobalEntropy::<ChaCha8Rng>::type_path()
        );

        assert_eq!(
            "GlobalEntropy<ChaCha8Rng>",
            GlobalEntropy::<ChaCha8Rng>::short_type_path()
        );
    }

    #[test]
    fn forking_into_component() {
        let mut source: GlobalEntropy<ChaCha8Rng> = GlobalEntropy::<ChaCha8Rng>::from_seed([1; 32]);

        let mut forked = source.fork_rng();

        let source_val = source.next_u32();

        let forked_val = forked.next_u32();

        assert_ne!(source_val, forked_val);
    }

    #[test]
    fn forking_as() {
        let mut rng1 = GlobalEntropy::<ChaCha12Rng>::from_entropy();

        let rng2 = rng1.fork_as::<WyRand>();

        let rng1 = format!("{:?}", rng1);
        let rng2 = format!("{:?}", rng2);

        assert_ne!(
            &rng1, &rng2,
            "GlobalEntropy should not match the forked component"
        );
    }

    #[test]
    fn forking_inner() {
        let mut rng1 = GlobalEntropy::<ChaCha8Rng>::from_entropy();

        let rng2 = rng1.fork_inner();

        assert_ne!(
            rng1.rng, rng2,
            "forked ChaCha8Rngs should not match each other"
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn rng_untyped_serialization() {
        use bevy::reflect::{
            serde::{ReflectDeserializer, ReflectSerializer},
            FromReflect, TypeRegistry,
        };
        use ron::ser::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistry::default();
        registry.register::<GlobalEntropy<ChaCha8Rng>>();

        let mut val = GlobalEntropy::<ChaCha8Rng>::from_seed([7; 32]);

        // Modify the state of the RNG instance
        val.next_u32();

        let ser = ReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(
            &serialized,
            "{\"bevy_rand::resource::GlobalEntropy<bevy_prng::ChaCha8Rng>\":(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),rng:((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)))}"
        );

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = ReflectDeserializer::new(&registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        let mut dynamic = GlobalEntropy::<ChaCha8Rng>::take_from_reflect(value).unwrap();

        // The two instances should be the same
        assert_eq!(
            val, dynamic,
            "The deserialized GlobalEntropy should equal the original"
        );
        // They should output the same numbers, as no state is lost between serialization and deserialization.
        assert_eq!(
            val.next_u32(),
            dynamic.next_u32(),
            "The deserialized GlobalEntropy should have the same output as original"
        );
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn rng_typed_serialization() {
        use bevy::reflect::{
            serde::{TypedReflectDeserializer, TypedReflectSerializer},
            FromReflect, GetTypeRegistration, TypeRegistry,
        };
        use ron::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistry::default();
        registry.register::<GlobalEntropy<ChaCha8Rng>>();

        let registered_type = GlobalEntropy::<ChaCha8Rng>::get_type_registration();

        let mut val = GlobalEntropy::<ChaCha8Rng>::from_seed([7; 32]);

        // Modify the state of the RNG instance
        val.next_u32();

        let ser = TypedReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(
            &serialized,
            "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),rng:((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)))"
        );

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = TypedReflectDeserializer::new(&registered_type, &registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        let mut dynamic = GlobalEntropy::<ChaCha8Rng>::take_from_reflect(value).unwrap();

        // The two instances should be the same
        assert_eq!(
            val, dynamic,
            "The deserialized GlobalEntropy should equal the original"
        );
        // They should output the same numbers, as no state is lost between serialization and deserialization.
        assert_eq!(
            val.next_u32(),
            dynamic.next_u32(),
            "The deserialized GlobalEntropy should have the same output as original"
        );
    }
}
