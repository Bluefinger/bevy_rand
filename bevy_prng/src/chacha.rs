use rand_core::SeedableRng;

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

macro_rules! chacha_impl {
    { #[feature = $feature:literal]
    $(#[$doc:meta]
    struct $name:ident($internal:ty);
    )+ } => {
        #[cfg(feature = "serialize")]
        #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
        struct ChaChaAbstractState {
            #[serde(
                serialize_with = "crate::utils::serialize_bytes",
                deserialize_with = "crate::utils::deserialize_bytes"
            )]
            state: ::chacha20::SerializedRngState,
        }

        $(
        #[cfg(feature = $feature)]
        #[derive(Debug, PartialEq, ::bevy_ecs::prelude::Component)]
        #[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
        #[cfg_attr(feature = "bevy_reflect", reflect(opaque))]
        #[cfg_attr(
            all(feature = "serialize", feature = "bevy_reflect"),
            reflect(
                Debug,
                Clone,
                Component,
                PartialEq,
                FromReflect,
                Serialize,
                Deserialize,
                RemoteRng,
            )
        )]
        #[cfg_attr(
            all(not(feature = "serialize"), feature = "bevy_reflect"),
            reflect(Debug, Clone, Component, PartialEq, FromReflect, RemoteRng)
        )]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        #[cfg_attr(feature = "bevy_reflect", type_path = "bevy_prng")]
        #[repr(transparent)]
        #[$doc]
        pub struct $name($internal);

        impl Clone for $name {
            fn clone(&self) -> Self {
                let mut rng = <$internal>::from_seed(self.0.get_seed());

                rng.set_stream(self.0.get_stream());
                rng.set_word_pos(self.0.get_word_pos());

                Self(rng)
            }
        }

        impl $name {
            #[doc = r" Create a new instance."]
            #[inline(always)]
            #[must_use]
            pub fn new(rng: $internal) -> Self {
                Self(rng)
            }
        }

        impl ::rand_core::TryRng for $name {
            type Error = ::core::convert::Infallible;
            #[inline(always)]
            fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
                ::rand_core::TryRng::try_next_u32(&mut self.0)
            }
            #[inline(always)]
            fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
                ::rand_core::TryRng::try_next_u64(&mut self.0)
            }
            #[inline(always)]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
                ::rand_core::TryRng::try_fill_bytes(&mut self.0, dest)
            }
        }

        #[cfg(feature = "compat_09")]
        impl ::rand_core_09::RngCore for $name {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                ::rand_core::Rng::next_u32(&mut self.0)
            }
            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                ::rand_core::Rng::next_u64(&mut self.0)
            }
            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                ::rand_core::Rng::fill_bytes(&mut self.0, dest)
            }
        }
        #[cfg(feature = "compat_06")]
        impl ::rand_core_06::RngCore for $name {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                ::rand_core::Rng::next_u32(&mut self.0)
            }
            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                ::rand_core::Rng::next_u64(&mut self.0)
            }
            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                ::rand_core::Rng::fill_bytes(&mut self.0, dest)
            }
            #[inline(always)]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core_06::Error> {
                ::rand_core::Rng::fill_bytes(&mut self.0, dest);
                Ok(())
            }
        }
        impl ::rand_core::SeedableRng for $name {
            type Seed = <$internal as ::rand_core::SeedableRng>::Seed;
            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self::new(<$internal>::from_seed(seed))
            }
            #[inline]
            fn from_rng<R: ::rand_core::Rng + ?Sized>(source: &mut R) -> Self {
                Self::new(<$internal>::from_rng(source))
            }
            #[inline]
            fn try_from_rng<R: ::rand_core::TryRng + ?Sized>(source: &mut R) -> Result<Self, R::Error> {
                Ok(Self::new(<$internal>::try_from_rng(source)?))
            }
        }
        impl Default for $name {
            fn default() -> Self {
                use ::rand_core::SeedableRng;
                #[cfg(feature = "thread_local_entropy")]
                {
                    let mut local = super::thread_local_entropy::ThreadLocalEntropy::get()
                        .expect("Unable to source entropy for initialisation");
                    Self::from_rng(&mut local)
                }
                #[cfg(not(feature = "thread_local_entropy"))]
                {
                    let mut seed: <$internal as ::rand_core::SeedableRng>::Seed =
                        Default::default();
                    ::getrandom::fill(seed.as_mut()).expect("Unable to source entropy for initialisation");
                    Self::from_seed(seed)
                }
            }
        }
        #[cfg(feature = "serialize")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ChaChaAbstractState::from(self).serialize(serializer)
            }
        }

        #[cfg(feature = "serialize")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                Ok(<$name>::from(ChaChaAbstractState::deserialize(
                    deserializer,
                )?))
            }
        }

        impl From<$internal> for $name {
            #[inline]
            fn from(value: $internal) -> Self {
                Self::new(value)
            }
        }
        impl crate::EntropySource for $name {}

        impl crate::RemoteRng for $name {}

        #[cfg(feature = "serialize")]
        impl From<&$name> for ChaChaAbstractState {
            fn from(value: &$name) -> Self {
                Self {
                    state: value.0.serialize_state(),
                }
            }
        }
        #[cfg(feature = "serialize")]
        impl From<ChaChaAbstractState> for $name {
            fn from(value: ChaChaAbstractState) -> Self {
                Self(<$internal>::deserialize_state(&value.state))
            }
        }
        )*
    };
}

chacha_impl! {
    #[feature = "chacha20"]

    /// A [`chacha20::ChaCha8Rng`] RNG component
    struct ChaCha8Rng(chacha20::ChaCha8Rng);

    /// A [`chacha20::ChaCha12Rng`] RNG component
    struct ChaCha12Rng(chacha20::ChaCha12Rng);

    /// A [`chacha20::ChaCha20Rng`] RNG component
    struct ChaCha20Rng(chacha20::ChaCha20Rng);
}
