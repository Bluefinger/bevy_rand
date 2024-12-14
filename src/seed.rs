use std::marker::PhantomData;

use bevy_ecs::{
    component::{Immutable, StorageType},
    prelude::Component,
};
use bevy_prng::EntropySource;
use bevy_reflect::Reflect;
use rand_core::SeedableRng;

use crate::{component::Entropy, traits::SeedSource};

/// The initial seed/state for an [`EntropyComponent`]. Adding this component to an `Entity` will cause
/// an `EntropyComponent` to be initialised as well. To force a reseed, just insert this component to an
/// `Entity` to overwrite the old value, and the `EntropyComponent` will be overwritten with the new seed
/// in turn.
#[derive(Debug, Reflect)]
pub struct RngSeed<R: EntropySource> {
    seed: R::Seed,
    #[reflect(ignore)]
    rng: PhantomData<R>,
}

impl<R: EntropySource> SeedSource<R> for RngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    /// Create a new instance of [`RngSeed`] from a given `seed` value.
    #[inline]
    #[must_use]
    fn from_seed(seed: R::Seed) -> Self {
        Self {
            seed,
            rng: PhantomData,
        }
    }

    #[inline]
    fn get_seed(&self) -> &R::Seed {
        &self.seed
    }

    #[inline]
    fn clone_seed(&self) -> R::Seed {
        self.seed.clone()
    }
}

impl<R: EntropySource> Component for RngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
        hooks
            .on_insert(|mut world, entity, _| {
                let seed = world
                    .get::<RngSeed<R>>(entity)
                    .map(|seed| seed.clone_seed())
                    .unwrap();
                world
                    .commands()
                    .entity(entity)
                    .insert(Entropy::<R>::from_seed(seed));
            })
            .on_remove(|mut world, entity, _| {
                world.commands().entity(entity).remove::<Entropy<R>>();
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialize")]
    #[test]
    fn reflection_serialization_round_trip_works() {
        use bevy_prng::WyRand;
        use bevy_reflect::{
            serde::{TypedReflectDeserializer, TypedReflectSerializer},
            FromReflect, GetTypeRegistration, TypeRegistry,
        };
        use ron::to_string;
        use serde::de::DeserializeSeed;

        let mut registry = TypeRegistry::default();
        registry.register::<RngSeed<WyRand>>();
        registry.register::<[u8; 8]>();

        let registered_type = RngSeed::<WyRand>::get_type_registration();

        let val = RngSeed::<WyRand>::from_seed(u64::MAX.to_ne_bytes());

        let ser = TypedReflectSerializer::new(&val, &registry);

        let serialized = to_string(&ser).unwrap();

        assert_eq!(&serialized, "(seed:(255,255,255,255,255,255,255,255))");

        let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

        let de = TypedReflectDeserializer::new(&registered_type, &registry);

        let value = de.deserialize(&mut deserializer).unwrap();

        assert!(value.is_dynamic());
        assert!(value.represents::<RngSeed<WyRand>>());
        assert!(value.try_downcast_ref::<RngSeed<WyRand>>().is_none());

        let recreated = RngSeed::<WyRand>::from_reflect(value.as_ref()).unwrap();

        assert_eq!(val.clone_seed(), recreated.clone_seed());
    }
}
