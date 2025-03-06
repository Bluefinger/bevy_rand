use crate::newtype::{newtype_prng, newtype_prng_remote};

use bevy_reflect::{Reflect, ReflectFromReflect, reflect_remote, std_traits::ReflectDefault};

#[cfg(feature = "serialize")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

/// Remote reflected version of [`rand_xoshiro::Seed512`], needed to support
/// proper reflection for the 512 bit variants of the Xoshiro PRNG.
#[reflect_remote(::rand_xoshiro::Seed512)]
#[derive(Debug, Default, Clone)]
#[reflect(Debug, Default)]
pub struct Seed512(pub [u8; 64]);

impl AsRef<[u8]> for Seed512 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for Seed512 {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

newtype_prng_remote!(
    Xoshiro512StarStar,
    ::rand_xoshiro::Xoshiro512StarStar,
    Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng_remote!(
    Xoshiro512PlusPlus,
    ::rand_xoshiro::Xoshiro512PlusPlus,
    Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng_remote!(
    Xoshiro512Plus,
    ::rand_xoshiro::Xoshiro512Plus,
    Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256StarStar,
    ::rand_xoshiro::Xoshiro256StarStar,
    "A newtyped [`rand_xoshiro::Xoshiro256StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256PlusPlus,
    ::rand_xoshiro::Xoshiro256PlusPlus,
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256Plus,
    ::rand_xoshiro::Xoshiro256Plus,
    "A newtyped [`rand_xoshiro::Xoshiro256Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128StarStar,
    ::rand_xoshiro::Xoroshiro128StarStar,
    "A newtyped [`rand_xoshiro::Xoshiro128StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128PlusPlus,
    ::rand_xoshiro::Xoroshiro128PlusPlus,
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128Plus,
    ::rand_xoshiro::Xoroshiro128Plus,
    "A newtyped [`rand_xoshiro::Xoshiro128Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128StarStar,
    ::rand_xoshiro::Xoshiro128StarStar,
    "A newtyped [`rand_xoshiro::Xoshiro128StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128PlusPlus,
    ::rand_xoshiro::Xoshiro128PlusPlus,
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128Plus,
    ::rand_xoshiro::Xoshiro128Plus,
    "A newtyped [`rand_xoshiro::Xoshiro128Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro64StarStar,
    ::rand_xoshiro::Xoroshiro64StarStar,
    "A newtyped [`rand_xoshiro::Xoroshiro64StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro64Star,
    ::rand_xoshiro::Xoroshiro64Star,
    "A newtyped [`rand_xoshiro::Xoroshiro64Star`] RNG",
    "rand_xoshiro"
);
