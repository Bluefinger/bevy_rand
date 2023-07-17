use std::fmt::Debug;

use crate::traits::SeedableEntropySource;
use bevy::{
    prelude::{Reflect, ReflectFromReflect, ReflectResource, Resource},
    reflect::{utility::GenericTypePathCell, TypePath},
};
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
/// use bevy_rand::prelude::*;
/// use rand_core::RngCore;
/// use rand_chacha::ChaCha8Rng;
///
/// fn print_random_value(mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
///   println!("Random value: {}", rng.next_u32());
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect_value(type_path = false)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(bound(deserialize = "R: for<'a> Deserialize<'a>"))
)]
#[cfg_attr(
    feature = "serialize",
    reflect_value(Debug, PartialEq, Resource, FromReflect, Serialize, Deserialize)
)]
#[cfg_attr(
    not(feature = "serialize"),
    reflect_value(Debug, PartialEq, Resource, FromReflect)
)]
pub struct GlobalEntropy<R: SeedableEntropySource + 'static>(R);

impl<R: SeedableEntropySource + 'static> GlobalEntropy<R> {
    /// Create a new resource from a `RngCore` instance.
    #[inline]
    #[must_use]
    pub fn new(rng: R) -> Self {
        Self(rng)
    }
}

impl<R: SeedableEntropySource + 'static> GlobalEntropy<R> {
    /// Reseeds the internal `RngCore` instance with a new seed.
    #[inline]
    pub fn reseed(&mut self, seed: R::Seed) {
        self.0 = R::from_seed(seed);
    }
}

impl<R: SeedableEntropySource + 'static> TypePath for GlobalEntropy<R> {
    fn type_path() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn short_type_path() -> &'static str {
        static CELL: GenericTypePathCell = GenericTypePathCell::new();
        CELL.get_or_insert::<Self, _>(|| bevy::utils::get_short_name(std::any::type_name::<Self>()))
    }

    fn type_ident() -> Option<&'static str> {
        Some("GlobalEntropy")
    }

    fn crate_name() -> Option<&'static str> {
        Some("bevy_rand")
    }

    fn module_path() -> Option<&'static str> {
        Some("bevy_rand::resource")
    }
}

impl<R: SeedableEntropySource + 'static> Default for GlobalEntropy<R> {
    fn default() -> Self {
        Self::from_entropy()
    }
}

impl<R: SeedableEntropySource + 'static> RngCore for GlobalEntropy<R> {
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

impl<R: SeedableEntropySource + 'static> SeedableRng for GlobalEntropy<R> {
    type Seed = R::Seed;

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
        // rng instances for many resources at once.
        ThreadLocalEntropy.fill_bytes(seed.as_mut());

        Self::new(R::from_seed(seed))
    }
}

impl<R: SeedableEntropySource + 'static> From<R> for GlobalEntropy<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: SeedableEntropySource + 'static> From<&mut R> for GlobalEntropy<R> {
    fn from(value: &mut R) -> Self {
        Self::from_rng(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serialize")]
    #[test]
    fn rng_reflection() {
        use super::*;
        use bevy::reflect::{
            serde::{ReflectSerializer, UntypedReflectDeserializer},
            TypeRegistryInternal,
        };
        use rand_chacha::ChaCha8Rng;
        use ron::ser::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistryInternal::default();
        registry.register::<GlobalEntropy<ChaCha8Rng>>();

        let mut val = GlobalEntropy::<ChaCha8Rng>::from_seed([7; 32]);

        // Modify the state of the RNG instance
        val.next_u32();

        let ser = ReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(
            &serialized,
            "{\"bevy_rand::resource::GlobalEntropy<rand_chacha::chacha::ChaCha8Rng>\":((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1))}"
        );

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = UntypedReflectDeserializer::new(&registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        let mut dynamic = value.take::<GlobalEntropy<ChaCha8Rng>>().unwrap();

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
