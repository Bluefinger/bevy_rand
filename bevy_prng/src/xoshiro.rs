use crate::{newtype::newtype_prng, SeedableEntropySource};

use bevy::prelude::{Reflect, ReflectFromReflect};
use rand_core::{RngCore, SeedableRng};

#[cfg(feature = "serialize")]
use bevy::prelude::{ReflectDeserialize, ReflectSerialize};

newtype_prng!(
    Xoshiro512StarStar,
    ::rand_xoshiro::Xoshiro512StarStar,
    "A newtyped [`rand_xoshiro::Xoshiro512StarStar`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro512PlusPlus,
    ::rand_xoshiro::Xoshiro512PlusPlus,
    "A newtyped [`rand_xoshiro::Xoshiro512PlusPlus`] RNG",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro512Plus,
    ::rand_xoshiro::Xoshiro512Plus,
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
