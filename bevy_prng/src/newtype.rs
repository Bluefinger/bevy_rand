macro_rules! newtype_prng {
    ($newtype:tt, $rng:ty, $doc:tt, $feature:tt) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, ::bevy_ecs::prelude::Component)]
        #[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
        #[cfg_attr(feature = "bevy_reflect", reflect(opaque))]
        #[cfg_attr(
            feature = "serialize",
            derive(::serde::Serialize, ::serde::Deserialize)
        )]
        #[cfg_attr(
            all(feature = "serialize", feature = "bevy_reflect"),
            reflect(opaque, Debug, Clone, Component, PartialEq, FromReflect, Serialize, Deserialize)
        )]
        #[cfg_attr(
            all(not(feature = "serialize"), feature = "bevy_reflect"),
            reflect(opaque, Debug, Clone, Component, PartialEq, FromReflect)
        )]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        #[cfg_attr(feature = "bevy_reflect", type_path = "bevy_prng")]
        #[repr(transparent)]
        pub struct $newtype($rng);

        impl $newtype {
            /// Create a new instance.
            #[inline(always)]
            #[must_use]
            pub fn new(rng: $rng) -> Self {
                Self(rng)
            }
        }

        impl ::rand_core::RngCore for $newtype {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                ::rand_core::RngCore::next_u32(&mut self.0)
            }

            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                ::rand_core::RngCore::next_u64(&mut self.0)
            }

            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                ::rand_core::RngCore::fill_bytes(&mut self.0, dest)
            }
        }

        #[cfg(feature = "compat")]
        impl ::rand_core_06::RngCore for $newtype {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                ::rand_core::RngCore::next_u32(&mut self.0)
            }

            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                ::rand_core::RngCore::next_u64(&mut self.0)
            }

            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                ::rand_core::RngCore::fill_bytes(&mut self.0, dest)
            }

            #[inline(always)]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core_06::Error> {
                ::rand_core::RngCore::fill_bytes(&mut self.0, dest);
                Ok(())
            }
        }

        impl ::rand_core::SeedableRng for $newtype {
            type Seed = <$rng as ::rand_core::SeedableRng>::Seed;

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self::new(<$rng>::from_seed(seed))
            }

            #[inline]
            fn from_rng(source: &mut impl ::rand_core::RngCore) -> Self {
                Self::new(<$rng>::from_rng(source))
            }

            #[inline]
            fn try_from_rng<T: ::rand_core::TryRngCore>(source: &mut T) -> Result<Self, T::Error> {
                Ok(Self::new(<$rng>::try_from_rng(source)?))
            }
        }

        impl Default for $newtype {
            fn default() -> Self {
                #[cfg(feature = "thread_local_entropy")]
                {
                    let mut local =
                        super::thread_local_entropy::ThreadLocalEntropy::new().expect("Unable to source entropy for initialisation");
                    Self::from_rng(&mut local)
                }
                #[cfg(not(feature = "thread_local_entropy"))]
                {
                    Self::from_os_rng()
                }
            }
        }

        impl From<$rng> for $newtype {
            #[inline]
            fn from(value: $rng) -> Self {
                Self::new(value)
            }
        }

        impl crate::EntropySource for $newtype {}
    };
}

#[cfg(all(feature = "rand_xoshiro", feature = "bevy_reflect"))]
macro_rules! newtype_prng_remote {
    ($newtype:tt, $rng:ty, $seed:ty, $doc:tt, $feature:tt) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
        #[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
        #[cfg_attr(feature = "bevy_reflect", reflect(opaque))]
        #[cfg_attr(
            feature = "serialize",
            derive(::serde::Serialize, ::serde::Deserialize)
        )]
        #[cfg_attr(
            all(feature = "serialize", feature = "bevy_reflect"),
            reflect(opaque, Debug, Clone, Component, PartialEq, FromReflect, Serialize, Deserialize)
        )]
        #[cfg_attr(
            all(not(feature = "serialize"), feature = "bevy_reflect"),
            reflect(opaque, Debug, Clone, Component, PartialEq, FromReflect)
        )]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        #[cfg_attr(feature = "bevy_reflect", type_path = "bevy_prng")]
        #[repr(transparent)]
        pub struct $newtype($rng);

        impl $newtype {
            /// Create a new instance.
            #[inline(always)]
            #[must_use]
            pub fn new(rng: $rng) -> Self {
                Self(rng)
            }
        }

        impl ::rand_core::RngCore for $newtype {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                ::rand_core::RngCore::next_u32(&mut self.0)
            }

            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                ::rand_core::RngCore::next_u64(&mut self.0)
            }

            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                ::rand_core::RngCore::fill_bytes(&mut self.0, dest)
            }
        }

        #[cfg(feature = "compat")]
        impl ::rand_core_06::RngCore for $newtype {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                ::rand_core::RngCore::next_u32(&mut self.0)
            }

            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                ::rand_core::RngCore::next_u64(&mut self.0)
            }

            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                ::rand_core::RngCore::fill_bytes(&mut self.0, dest)
            }

            #[inline(always)]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core_06::Error> {
                ::rand_core::RngCore::fill_bytes(&mut self.0, dest);
                Ok(())
            }
        }

        impl ::rand_core::SeedableRng for $newtype {
            type Seed = $seed;

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self::new(<$rng>::from_seed(seed.0))
            }

            #[inline]
            fn from_rng(source: &mut impl ::rand_core::RngCore) -> Self {
                Self::new(<$rng>::from_rng(source))
            }

            #[inline]
            fn try_from_rng<T: ::rand_core::TryRngCore>(source: &mut T) -> Result<Self, T::Error> {
                Ok(Self::new(<$rng>::try_from_rng(source)?))
            }
        }

        impl From<$rng> for $newtype {
            #[inline]
            fn from(value: $rng) -> Self {
                Self::new(value)
            }
        }

        impl crate::EntropySource for $newtype {}
    };
}

pub(crate) use newtype_prng;
#[cfg(all(feature = "rand_xoshiro", feature = "bevy_reflect"))]
pub(crate) use newtype_prng_remote;
