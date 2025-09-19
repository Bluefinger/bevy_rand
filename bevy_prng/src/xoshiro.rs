use crate::newtype::newtype_prng;

use rand_core::SeedableRng;

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;

#[cfg(feature = "bevy_reflect")]
use crate::newtype::newtype_prng_remote;

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect, reflect_remote, std_traits::ReflectDefault};

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

#[cfg(feature = "bevy_reflect")]
/// Remote reflected version of [`rand_xoshiro::Seed512`], needed to support
/// proper reflection for the 512 bit variants of the Xoshiro PRNG.
#[cfg_attr(feature = "bevy_reflect", reflect_remote(::rand_xoshiro::Seed512))]
#[derive(Debug, Default, Clone)]
#[reflect(Debug, Default)]
pub struct Seed512(pub [u8; 64]);

#[cfg(feature = "bevy_reflect")]
impl AsRef<[u8]> for Seed512 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(feature = "bevy_reflect")]
impl AsMut<[u8]> for Seed512 {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

#[cfg(feature = "bevy_reflect")]
newtype_prng_remote!(
    Xoshiro512StarStar,
    ::rand_xoshiro::Xoshiro512StarStar,
    Seed512,
    "A [`rand_xoshiro::Xoshiro512StarStar`] RNG component",
    "rand_xoshiro"
);

#[cfg(not(feature = "bevy_reflect"))]
newtype_prng!(
    Xoshiro512StarStar,
    ::rand_xoshiro::Xoshiro512StarStar,
    "A [`rand_xoshiro::Xoshiro512StarStar`] RNG component",
    "rand_xoshiro"
);

#[cfg(feature = "bevy_reflect")]
newtype_prng_remote!(
    Xoshiro512PlusPlus,
    ::rand_xoshiro::Xoshiro512PlusPlus,
    Seed512,
    "A [`rand_xoshiro::Xoshiro512PlusPlus`] RNG component",
    "rand_xoshiro"
);

#[cfg(not(feature = "bevy_reflect"))]
newtype_prng!(
    Xoshiro512PlusPlus,
    ::rand_xoshiro::Xoshiro512PlusPlus,
    "A [`rand_xoshiro::Xoshiro512PlusPlus`] RNG component",
    "rand_xoshiro"
);

#[cfg(feature = "bevy_reflect")]
newtype_prng_remote!(
    Xoshiro512Plus,
    ::rand_xoshiro::Xoshiro512Plus,
    Seed512,
    "A [`rand_xoshiro::Xoshiro512Plus`] RNG component",
    "rand_xoshiro"
);

#[cfg(not(feature = "bevy_reflect"))]
newtype_prng!(
    Xoshiro512Plus,
    ::rand_xoshiro::Xoshiro512Plus,
    "A [`rand_xoshiro::Xoshiro512Plus`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256StarStar,
    ::rand_xoshiro::Xoshiro256StarStar,
    "A [`rand_xoshiro::Xoshiro256StarStar`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256PlusPlus,
    ::rand_xoshiro::Xoshiro256PlusPlus,
    "A [`rand_xoshiro::Xoshiro256PlusPlus`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro256Plus,
    ::rand_xoshiro::Xoshiro256Plus,
    "A [`rand_xoshiro::Xoshiro256Plus`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128StarStar,
    ::rand_xoshiro::Xoroshiro128StarStar,
    "A [`rand_xoshiro::Xoshiro128StarStar`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128PlusPlus,
    ::rand_xoshiro::Xoroshiro128PlusPlus,
    "A [`rand_xoshiro::Xoshiro256PlusPlus`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro128Plus,
    ::rand_xoshiro::Xoroshiro128Plus,
    "A [`rand_xoshiro::Xoshiro128Plus`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128StarStar,
    ::rand_xoshiro::Xoshiro128StarStar,
    "A [`rand_xoshiro::Xoshiro128StarStar`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128PlusPlus,
    ::rand_xoshiro::Xoshiro128PlusPlus,
    "A [`rand_xoshiro::Xoshiro256PlusPlus`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoshiro128Plus,
    ::rand_xoshiro::Xoshiro128Plus,
    "A [`rand_xoshiro::Xoshiro128Plus`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro64StarStar,
    ::rand_xoshiro::Xoroshiro64StarStar,
    "A [`rand_xoshiro::Xoroshiro64StarStar`] RNG component",
    "rand_xoshiro"
);

newtype_prng!(
    Xoroshiro64Star,
    ::rand_xoshiro::Xoroshiro64Star,
    "A [`rand_xoshiro::Xoroshiro64Star`] RNG component",
    "rand_xoshiro"
);
