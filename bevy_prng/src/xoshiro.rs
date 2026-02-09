use crate::newtype::newtype_prng;

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;

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
impl PartialEq for Seed512 {
    fn eq(&self, other: &Self) -> bool {
        self.0.0 == other.0.0
    }
}

#[cfg(feature = "bevy_reflect")]
impl Eq for Seed512 {}

#[cfg(feature = "bevy_reflect")]
impl From<[u8; 64]> for Seed512 {
    fn from(value: [u8; 64]) -> Self {
        Self(::rand_xoshiro::Seed512(value))
    }
}

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

newtype_prng_remote! {
    #[feature = "rand_xoshiro"]
    #[seed = Seed512]
    /// A [`rand_xoshiro::Xoshiro512StarStar`] RNG component
    struct Xoshiro512StarStar(rand_xoshiro::Xoshiro512StarStar);

    /// A [`rand_xoshiro::Xoshiro512PlusPlus`] RNG component
    struct Xoshiro512PlusPlus(rand_xoshiro::Xoshiro512PlusPlus);

    /// A [`rand_xoshiro::Xoshiro512Plus`] RNG component
    struct Xoshiro512Plus(rand_xoshiro::Xoshiro512Plus);
}

newtype_prng! {
    #[feature = "rand_xoshiro"]

    /// A [`rand_xoshiro::Xoshiro256StarStar`] RNG component
    struct Xoshiro256StarStar(rand_xoshiro::Xoshiro256StarStar);

    /// A [`rand_xoshiro::Xoshiro256PlusPlus`] RNG component
    struct Xoshiro256PlusPlus(rand_xoshiro::Xoshiro256PlusPlus);

    /// A [`rand_xoshiro::Xoshiro256Plus`] RNG component
    struct Xoshiro256Plus(rand_xoshiro::Xoshiro256Plus);

    /// A [`rand_xoshiro::Xoshiro128StarStar`] RNG component
    struct Xoroshiro128StarStar(rand_xoshiro::Xoroshiro128StarStar);

    /// A [`rand_xoshiro::Xoshiro256PlusPlus`] RNG component
    struct Xoroshiro128PlusPlus(rand_xoshiro::Xoroshiro128PlusPlus);

    /// A [`rand_xoshiro::Xoshiro128Plus`] RNG component
    struct Xoroshiro128Plus(rand_xoshiro::Xoroshiro128Plus);

    /// A [`rand_xoshiro::Xoshiro128StarStar`] RNG component
    struct Xoshiro128StarStar(rand_xoshiro::Xoshiro128StarStar);

    /// A [`rand_xoshiro::Xoshiro256PlusPlus`] RNG component
    struct Xoshiro128PlusPlus(rand_xoshiro::Xoshiro128PlusPlus);

    /// A [`rand_xoshiro::Xoshiro128Plus`] RNG component
    struct Xoshiro128Plus(rand_xoshiro::Xoshiro128Plus);

    /// A [`rand_xoshiro::Xoroshiro64StarStar`] RNG component
    struct Xoroshiro64StarStar(rand_xoshiro::Xoroshiro64StarStar);

    /// A [`rand_xoshiro::Xoroshiro64Star`] RNG component
    struct Xoroshiro64Star(rand_xoshiro::Xoroshiro64Star);
}
