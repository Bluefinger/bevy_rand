use crate::{newtype::newtype_prng, SeedableEntropySource};

use bevy::prelude::{Reflect, ReflectFromReflect};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "serialize")]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    Xoshiro512StarStar,
    ::rand_xoshiro::Xoshiro512StarStar,
    ::rand_xoshiro::Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro512PlusPlus,
    ::rand_xoshiro::Xoshiro512PlusPlus,
    ::rand_xoshiro::Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro512Plus,
    ::rand_xoshiro::Xoshiro512Plus,
    ::rand_xoshiro::Seed512,
    "A newtyped [`rand_xoshiro::Xoshiro512Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256StarStar,
    ::rand_xoshiro::Xoshiro256StarStar,
    [u8; 32],
    "A newtyped [`rand_xoshiro::Xoshiro256StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256PlusPlus,
    ::rand_xoshiro::Xoshiro256PlusPlus,
    [u8; 32],
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256Plus,
    ::rand_xoshiro::Xoshiro256Plus,
    [u8; 32],
    "A newtyped [`rand_xoshiro::Xoshiro256Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128StarStar,
    ::rand_xoshiro::Xoroshiro128StarStar,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128PlusPlus,
    ::rand_xoshiro::Xoroshiro128PlusPlus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128Plus,
    ::rand_xoshiro::Xoroshiro128Plus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128StarStar,
    ::rand_xoshiro::Xoshiro128StarStar,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128PlusPlus,
    ::rand_xoshiro::Xoshiro128PlusPlus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro256PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128Plus,
    ::rand_xoshiro::Xoshiro128Plus,
    [u8; 16],
    "A newtyped [`rand_xoshiro::Xoshiro128Plus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro64StarStar,
    ::rand_xoshiro::Xoroshiro64StarStar,
    [u8; 8],
    "A newtyped [`rand_xoshiro::Xoroshiro64StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro64Star,
    ::rand_xoshiro::Xoroshiro64Star,
    [u8; 8],
    "A newtyped [`rand_xoshiro::Xoroshiro64Star`] RNG",
    "rand_xoshiro"
);
