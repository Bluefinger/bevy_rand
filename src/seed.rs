use std::marker::PhantomData;

use bevy::{
    app::App,
    ecs::system::Resource,
    reflect::{FromReflect, GetTypeRegistration, Reflect, TypePath},
};
use bevy_prng::SeedableEntropySource;
use rand_core::RngCore;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Resource, Reflect)]
#[cfg_attr(
    feature = "serialize",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(
    feature = "serialize",
    serde(bound(deserialize = "R::Seed: Serialize + for<'a> Deserialize<'a>"))
)]
/// Resource for storing the initial seed used to initialize a [`crate::resource::GlobalEntropy`].
/// Useful for tracking the starting seed or for forcing [`crate::resource::GlobalEntropy`] to reseed.
pub struct GlobalRngSeed<R: SeedableEntropySource> {
    seed: R::Seed,
    #[reflect(ignore)]
    rng: PhantomData<R>,
}

impl<R: SeedableEntropySource> GlobalRngSeed<R>
where
    R::Seed: Sync + Send + Clone + Reflect + GetTypeRegistration + FromReflect + TypePath,
{
    /// Helper method to register the necessary types for [`Reflect`] purposes. Ensures
    /// that not only the main type is registered, but also the correct seed type for the
    /// PRNG.
    pub fn register_type(app: &mut App) {
        app.register_type::<Self>();
        app.register_type::<R::Seed>();
    }
}

impl<R: SeedableEntropySource> GlobalRngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    /// Create a new instance of [`GlobalRngSeed`].
    #[inline]
    #[must_use]
    pub fn new(seed: R::Seed) -> Self {
        Self {
            seed,
            rng: PhantomData,
        }
    }

    /// Returns a cloned instance of the seed value.
    #[inline]
    pub fn get_seed(&self) -> R::Seed {
        self.seed.clone()
    }

    /// Set the global seed to a new value
    pub fn set_seed(&mut self, seed: R::Seed) {
        self.seed = seed;
    }

    /// Initializes an instance of [`GlobalRngSeed`] with a randomised seed
    /// value, drawn from thread-local or OS sources.
    #[inline]
    pub fn from_entropy() -> Self {
        let mut seed = Self::new(R::Seed::default());

        #[cfg(feature = "thread_local_entropy")]
        {
            use crate::thread_local_entropy::ThreadLocalEntropy;

            ThreadLocalEntropy::new().fill_bytes(seed.as_mut());
        }
        #[cfg(not(feature = "thread_local_entropy"))]
        {
            use getrandom::getrandom;

            getrandom(seed.as_mut()).expect("Unable to source entropy for seeding");
        }

        seed
    }
}

impl<R: SeedableEntropySource> Default for GlobalRngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    #[inline]
    fn default() -> Self {
        Self::from_entropy()
    }
}

impl<R: SeedableEntropySource> AsMut<[u8]> for GlobalRngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.seed.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialize")]
    #[test]
    fn reflection_serialization_round_trip_works() {
        use bevy::reflect::{
            serde::{TypedReflectDeserializer, TypedReflectSerializer},
            GetTypeRegistration, TypeRegistry,
        };
        use bevy_prng::WyRand;
        use ron::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistry::default();
        registry.register::<GlobalRngSeed<WyRand>>();
        registry.register::<[u8; 8]>();

        let registered_type = GlobalRngSeed::<WyRand>::get_type_registration();

        let val = GlobalRngSeed::<WyRand>::new(u64::MAX.to_ne_bytes());

        let ser = TypedReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(&serialized, "(seed:(255,255,255,255,255,255,255,255))");

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = TypedReflectDeserializer::new(&registered_type, &registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        assert!(value.is_dynamic());
        assert!(value.represents::<GlobalRngSeed<WyRand>>());
        assert!(!value.is::<GlobalRngSeed<WyRand>>());

        let recreated = GlobalRngSeed::<WyRand>::from_reflect(value.as_reflect()).unwrap();

        assert_eq!(val.get_seed(), recreated.get_seed());
    }
}
