use core::{marker::PhantomData, ops::Deref};

use bevy_ecs::{
    component::{Immutable, StorageType},
    prelude::Component,
};
use bevy_prng::EntropySource;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;
use rand_core::SeedableRng;

use crate::{
    component::{Entropy, FastEntropy},
    prngs::{FastRngBackend, FastSeed},
    traits::SeedSource,
};

/// The initial seed/state for an [`Entropy`]. Adding this component to an `Entity` will cause
/// an `Entropy` to be initialised as well. To force a reseed, just insert this component to an
/// `Entity` to overwrite the old value, and the `Entropy` will be overwritten with the new seed
/// in turn.
///
/// ## Examples
///
/// Randomised Seed via `Default`:
/// ```
/// use bevy_ecs::prelude::*;
/// use bevy_prng::WyRand;
/// use bevy_rand::prelude::RngSeed;
///
/// #[derive(Component)]
/// struct Source;
///
/// fn setup_source(mut commands: Commands) {
///     commands
///         .spawn((
///             Source,
///             RngSeed::<WyRand>::default(),
///         ));
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct RngSeed<R: EntropySource> {
    seed: R::Seed,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    rng: PhantomData<R>,
}

impl<R: EntropySource> SeedSource<R> for RngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    /// Create a new instance of [`RngSeed`] from a given `seed` value.
    #[inline]
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

impl<R: EntropySource + 'static> Component for RngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    fn on_insert() -> Option<bevy_ecs::component::ComponentHook> {
        Some(|mut world, context| {
            let seed = world
                .get::<RngSeed<R>>(context.entity)
                .map(|seed| seed.clone_seed())
                .unwrap();
            world
                .commands()
                .entity(context.entity)
                .insert(Entropy::<R>::from_seed(seed));
        })
    }

    fn on_remove() -> Option<bevy_ecs::component::ComponentHook> {
        Some(|mut world, context| {
            world
                .commands()
                .entity(context.entity)
                .remove::<Entropy<R>>();
        })
    }
}

impl<R: EntropySource> Default for RngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    #[inline]
    fn default() -> Self {
        #[cfg(feature = "thread_local_entropy")]
        {
            Self::from_local_entropy()
        }
        #[cfg(not(feature = "thread_local_entropy"))]
        {
            Self::from_os_rng()
        }
    }
}

impl<R: EntropySource> Deref for RngSeed<R>
where
    R::Seed: Sync + Send + Clone,
{
    type Target = R::Seed;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get_seed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct FastRngSeed {
    seed: FastSeed,
}

impl FastRngSeed {
    /// Create a new instance of [`FastRngSeed`] from a given `seed` value.
    #[inline]
    pub fn from_seed(seed: FastSeed) -> Self {
        Self { seed }
    }

    pub fn from_entropy<V: Fn(Prng::Seed, PhantomData<Prng>) -> FastSeed, Prng: SeedableRng>(
        seed: V,
    ) -> Self {
        Self {
            seed: FastSeed::with_entropy(seed),
        }
    }

    #[inline]
    pub fn get_seed(&self) -> &FastSeed {
        &self.seed
    }

    #[inline]
    pub fn clone_seed(&self) -> FastSeed {
        self.seed.clone()
    }
}

impl Component for FastRngSeed {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    fn on_insert() -> Option<bevy_ecs::component::ComponentHook> {
        Some(|mut world, context| {
            let seed = world
                .get::<FastRngSeed>(context.entity)
                .map(|seed| seed.clone_seed())
                .unwrap();
            world.commands().entity(context.entity).insert(FastEntropy {
                backend: FastRngBackend::from(seed),
            });
        })
    }

    fn on_remove() -> Option<bevy_ecs::component::ComponentHook> {
        Some(|mut world, context| {
            world
                .commands()
                .entity(context.entity)
                .remove::<FastEntropy>();
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
    #[test]
    fn reflection_serialization_round_trip_works() {
        use super::*;

        use bevy_prng::WyRand;
        use bevy_reflect::{
            FromReflect, GetTypeRegistration, TypeRegistry,
            serde::{TypedReflectDeserializer, TypedReflectSerializer},
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
