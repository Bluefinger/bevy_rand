macro_rules! newtype_prng {
    ($newtype:tt, $rng:ty, $seed:ty, $doc:tt, $feature:tt) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Reflect)]
        #[cfg_attr(
            feature = "serialize",
            derive(::serde_derive::Serialize, ::serde_derive::Deserialize)
        )]
        #[cfg_attr(
            all(feature = "serialize"),
            reflect_value(Debug, PartialEq, FromReflect, Serialize, Deserialize)
        )]
        #[cfg_attr(
            all(not(feature = "serialize")),
            reflect_value(Debug, PartialEq, FromReflect)
        )]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        #[type_path = "bevy_prng"]
        #[repr(transparent)]
        pub struct $newtype($rng);

        impl $newtype {
            /// Create a new instance.
            #[inline]
            #[must_use]
            pub fn new(rng: $rng) -> Self {
                Self(rng)
            }
        }

        impl RngCore for $newtype {
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                self.0.next_u32()
            }

            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                self.0.next_u64()
            }

            #[inline]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                self.0.fill_bytes(dest)
            }

            #[inline]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
                self.0.try_fill_bytes(dest)
            }
        }

        impl SeedableRng for $newtype {
            type Seed = $seed;

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self::new(<$rng>::from_seed(seed))
            }
        }

        impl From<$rng> for $newtype {
            #[inline]
            fn from(value: $rng) -> Self {
                Self::new(value)
            }
        }

        impl SeedableEntropySource for $newtype {}
    };
}

pub(crate) use newtype_prng;
