macro_rules! newtype_prng {
    { #[feature = $feature:literal]
    $(#[$doc:meta]
    struct $newtype:ident($rng:ty);
    )+ } => {
        $(
        #[derive(Debug, Clone, PartialEq, ::bevy_ecs::prelude::Component)]
        #[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
        #[cfg_attr(feature = "bevy_reflect", reflect(opaque))]
        #[cfg_attr(
            feature = "serialize",
            derive(::serde::Serialize, ::serde::Deserialize)
        )]
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
        pub struct $newtype($rng);

        impl $newtype {
            /// Create a new instance.
            #[inline(always)]
            #[must_use]
            pub fn new(rng: $rng) -> Self {
                Self(rng)
            }
        }

        impl ::rand_core::TryRng for $newtype {
            type Error = core::convert::Infallible;

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
        impl ::rand_core_09::RngCore for $newtype {
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
        impl ::rand_core_06::RngCore for $newtype {
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

        impl ::rand_core::SeedableRng for $newtype {
            type Seed = <$rng as ::rand_core::SeedableRng>::Seed;

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self::new(<$rng>::from_seed(seed))
            }

            #[inline]
            fn from_rng<R: ::rand_core::Rng + ?Sized>(source: &mut R) -> Self {
                Self::new(<$rng>::from_rng(source))
            }

            #[inline]
            fn try_from_rng<R: ::rand_core::TryRng + ?Sized>(
                source: &mut R,
            ) -> Result<Self, R::Error> {
                Ok(Self::new(<$rng>::try_from_rng(source)?))
            }
        }

        impl Default for $newtype {
            fn default() -> Self {
                use rand_core::SeedableRng;

                #[cfg(feature = "thread_local_entropy")]
                {
                    let mut local = super::thread_local_entropy::ThreadLocalEntropy::get()
                        .expect("Unable to source entropy for initialisation");
                    Self::from_rng(&mut local)
                }
                #[cfg(not(feature = "thread_local_entropy"))]
                {
                    let mut seed: <$rng as ::rand_core::SeedableRng>::Seed = Default::default();

                    getrandom::fill(seed.as_mut())
                        .expect("Unable to source entropy for initialisation");

                    Self::from_seed(seed)
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

        impl crate::RemoteRng for $newtype {}
    )*
    };
}

#[cfg(feature = "rand_xoshiro")]
macro_rules! newtype_prng_remote {
    { #[feature = $feature:literal]
    #[seed = $seed:ty]
    $(#[$doc:meta]
    struct $newtype:ident($rng:ty);)+
    } => {
        $(#[cfg(feature = "bevy_reflect")]
        #[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
        #[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
        #[cfg_attr(feature = "bevy_reflect", reflect(opaque))]
        #[cfg_attr(
            feature = "serialize",
            derive(::serde::Serialize, ::serde::Deserialize)
        )]
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
                RemoteRng
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
        pub struct $newtype($rng);

        #[cfg(feature = "bevy_reflect")]
        impl $newtype {
            /// Create a new instance.
            #[inline(always)]
            #[must_use]
            pub fn new(rng: $rng) -> Self {
                Self(rng)
            }
        }

        #[cfg(feature = "bevy_reflect")]
        impl ::rand_core::TryRng for $newtype {
            type Error = core::convert::Infallible;

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

        #[cfg(all(feature = "compat_06", feature = "rand_xoshiro"))]
        impl ::rand_core_06::RngCore for $newtype {
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

        #[cfg(all(feature = "compat_09", feature = "rand_xoshiro"))]
        impl ::rand_core_09::RngCore for $newtype {
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

        #[cfg(feature = "bevy_reflect")]
        impl ::rand_core::SeedableRng for $newtype {
            type Seed = $seed;

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self::new(<$rng>::from_seed(seed.0))
            }

            #[inline]
            fn from_rng<R: ::rand_core::Rng + ?Sized>(source: &mut R) -> Self {
                Self::new(<$rng>::from_rng(source))
            }

            #[inline]
            fn try_from_rng<R: ::rand_core::TryRng + ?Sized>(
                source: &mut R,
            ) -> Result<Self, R::Error> {
                Ok(Self::new(<$rng>::try_from_rng(source)?))
            }
        }

        #[cfg(feature = "bevy_reflect")]
        impl Default for $newtype {
            fn default() -> Self {
                use rand_core::SeedableRng;

                #[cfg(feature = "thread_local_entropy")]
                {
                    let mut local = super::thread_local_entropy::ThreadLocalEntropy::get()
                        .expect("Unable to source entropy for initialisation");
                    Self::from_rng(&mut local)
                }
                #[cfg(not(feature = "thread_local_entropy"))]
                {
                    Self::try_from_rng(&mut getrandom::SysRng)
                        .expect("Unable to source entropy for initialisation")
                }
            }
        }

        #[cfg(feature = "bevy_reflect")]
        impl From<$rng> for $newtype {
            #[inline]
            fn from(value: $rng) -> Self {
                Self::new(value)
            }
        }

        #[cfg(feature = "bevy_reflect")]
        impl crate::EntropySource for $newtype {}

        #[cfg(feature = "bevy_reflect")]
        impl crate::RemoteRng for $newtype {}

        #[cfg(not(feature = "bevy_reflect"))]
        crate::newtype::newtype_prng! {
            #[feature = "rand_xoshiro"]
            #[$doc]
            struct $newtype($rng);
        }
        )*
    };
}

pub(crate) use newtype_prng;
#[cfg(feature = "rand_xoshiro")]
pub(crate) use newtype_prng_remote;
