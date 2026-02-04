use rand_core::SeedableRng;

#[cfg(feature = "bevy_reflect")]
use crate::ReflectRemoteRng;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{Reflect, ReflectFromReflect};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

#[doc = "A [`chacha20::ChaCha8Rng`] RNG component"]
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
#[cfg_attr(docsrs, doc(cfg(feature = "chacha20")))]
#[cfg_attr(feature = "bevy_reflect", type_path = "bevy_prng")]
#[repr(transparent)]
pub struct ChaCha8Rng(::chacha20::ChaCha8Rng);

impl Clone for ChaCha8Rng {
    fn clone(&self) -> Self {
        let mut rng = ::chacha20::ChaCha8Rng::from_seed(self.0.get_seed());

        rng.set_stream(self.0.get_stream());
        rng.set_word_pos(self.0.get_word_pos());

        Self(rng)
    }
}

impl ChaCha8Rng {
    #[doc = r" Create a new instance."]
    #[inline(always)]
    #[must_use]
    pub fn new(rng: ::chacha20::ChaCha8Rng) -> Self {
        Self(rng)
    }
}
impl ::rand_core::TryRng for ChaCha8Rng {
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
impl ::rand_core_09::RngCore for ChaCha8Rng {
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
impl ::rand_core_06::RngCore for ChaCha8Rng {
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
impl ::rand_core::SeedableRng for ChaCha8Rng {
    type Seed = <::chacha20::ChaCha8Rng as ::rand_core::SeedableRng>::Seed;
    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(<::chacha20::ChaCha8Rng>::from_seed(seed))
    }
    #[inline]
    fn from_rng<R: ::rand_core::Rng + ?Sized>(source: &mut R) -> Self {
        Self::new(<::chacha20::ChaCha8Rng>::from_rng(source))
    }
    #[inline]
    fn try_from_rng<R: ::rand_core::TryRng + ?Sized>(source: &mut R) -> Result<Self, R::Error> {
        Ok(Self::new(<::chacha20::ChaCha8Rng>::try_from_rng(source)?))
    }
}
impl Default for ChaCha8Rng {
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
            let mut seed: <::chacha20::ChaCha8Rng as ::rand_core::SeedableRng>::Seed =
                Default::default();
            getrandom::fill(seed.as_mut()).expect("Unable to source entropy for initialisation");
            Self::from_seed(seed)
        }
    }
}

#[cfg(feature = "serialize")]
#[derive(serde::Serialize, serde::Deserialize)]
struct ChaCha8Core {
    seed: [u8; 32],
    stream: u64,
    word_pos: u128,
}

#[cfg(feature = "serialize")]
impl From<&ChaCha8Rng> for ChaCha8Core {
    #[inline]
    fn from(value: &ChaCha8Rng) -> Self {
        Self {
            seed: value.0.get_seed(),
            stream: value.0.get_stream(),
            word_pos: value.0.get_word_pos(),
        }
    }
}

#[cfg(feature = "serialize")]
impl From<ChaCha8Core> for ChaCha8Rng {
    #[inline]
    fn from(value: ChaCha8Core) -> Self {
        let mut rng = chacha20::ChaCha8Rng::from_seed(value.seed);

        rng.set_stream(value.stream);
        rng.set_word_pos(value.word_pos);

        Self(rng)
    }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for ChaCha8Rng {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ChaCha8Core::from(self).serialize(serializer)
    }
}

#[cfg(feature = "serialize")]
impl<'de> serde::Deserialize<'de> for ChaCha8Rng {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ChaCha8Rng::from(ChaCha8Core::deserialize(deserializer)?))
    }
}

impl From<::chacha20::ChaCha8Rng> for ChaCha8Rng {
    #[inline]
    fn from(value: ::chacha20::ChaCha8Rng) -> Self {
        Self::new(value)
    }
}
impl crate::EntropySource for ChaCha8Rng {}

impl crate::RemoteRng for ChaCha8Rng {}

#[doc = "A [`chacha20::ChaCha12Rng`] RNG component"]
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
#[cfg_attr(docsrs, doc(cfg(feature = "chacha20")))]
#[cfg_attr(feature = "bevy_reflect", type_path = "bevy_prng")]
#[repr(transparent)]
pub struct ChaCha12Rng(::chacha20::ChaCha12Rng);

impl Clone for ChaCha12Rng {
    fn clone(&self) -> Self {
        let mut rng = ::chacha20::ChaCha12Rng::from_seed(self.0.get_seed());

        rng.set_stream(self.0.get_stream());
        rng.set_word_pos(self.0.get_word_pos());

        Self(rng)
    }
}

impl ChaCha12Rng {
    #[doc = r" Create a new instance."]
    #[inline(always)]
    #[must_use]
    pub fn new(rng: ::chacha20::ChaCha12Rng) -> Self {
        Self(rng)
    }
}
impl ::rand_core::TryRng for ChaCha12Rng {
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
impl ::rand_core_09::RngCore for ChaCha12Rng {
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
impl ::rand_core_06::RngCore for ChaCha12Rng {
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
impl ::rand_core::SeedableRng for ChaCha12Rng {
    type Seed = <::chacha20::ChaCha8Rng as ::rand_core::SeedableRng>::Seed;
    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(<::chacha20::ChaCha12Rng>::from_seed(seed))
    }
    #[inline]
    fn from_rng<R: ::rand_core::Rng + ?Sized>(source: &mut R) -> Self {
        Self::new(<::chacha20::ChaCha12Rng>::from_rng(source))
    }
    #[inline]
    fn try_from_rng<R: ::rand_core::TryRng + ?Sized>(source: &mut R) -> Result<Self, R::Error> {
        Ok(Self::new(<::chacha20::ChaCha12Rng>::try_from_rng(source)?))
    }
}
impl Default for ChaCha12Rng {
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
            let mut seed: <::chacha20::ChaCha8Rng as ::rand_core::SeedableRng>::Seed =
                Default::default();
            getrandom::fill(seed.as_mut()).expect("Unable to source entropy for initialisation");
            Self::from_seed(seed)
        }
    }
}

#[cfg(feature = "serialize")]
#[derive(serde::Serialize, serde::Deserialize)]
struct ChaCha12Core {
    seed: [u8; 32],
    stream: u64,
    word_pos: u128,
}

#[cfg(feature = "serialize")]
impl From<&ChaCha12Rng> for ChaCha12Core {
    #[inline]
    fn from(value: &ChaCha12Rng) -> Self {
        Self {
            seed: value.0.get_seed(),
            stream: value.0.get_stream(),
            word_pos: value.0.get_word_pos(),
        }
    }
}

#[cfg(feature = "serialize")]
impl From<ChaCha12Core> for ChaCha12Rng {
    #[inline]
    fn from(value: ChaCha12Core) -> Self {
        let mut rng = chacha20::ChaCha12Rng::from_seed(value.seed);

        rng.set_stream(value.stream);
        rng.set_word_pos(value.word_pos);

        Self(rng)
    }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for ChaCha12Rng {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ChaCha12Core::from(self).serialize(serializer)
    }
}

#[cfg(feature = "serialize")]
impl<'de> serde::Deserialize<'de> for ChaCha12Rng {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ChaCha12Rng::from(ChaCha12Core::deserialize(deserializer)?))
    }
}

impl From<::chacha20::ChaCha12Rng> for ChaCha12Rng {
    #[inline]
    fn from(value: ::chacha20::ChaCha12Rng) -> Self {
        Self::new(value)
    }
}
impl crate::EntropySource for ChaCha12Rng {}

impl crate::RemoteRng for ChaCha12Rng {}

#[doc = "A [`chacha20::ChaCha20Rng`] RNG component"]
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
#[cfg_attr(docsrs, doc(cfg(feature = "chacha20")))]
#[cfg_attr(feature = "bevy_reflect", type_path = "bevy_prng")]
#[repr(transparent)]
pub struct ChaCha20Rng(::chacha20::ChaCha20Rng);

impl Clone for ChaCha20Rng {
    fn clone(&self) -> Self {
        let mut rng = ::chacha20::ChaCha20Rng::from_seed(self.0.get_seed());

        rng.set_stream(self.0.get_stream());
        rng.set_word_pos(self.0.get_word_pos());

        Self(rng)
    }
}

impl ChaCha20Rng {
    #[doc = r" Create a new instance."]
    #[inline(always)]
    #[must_use]
    pub fn new(rng: ::chacha20::ChaCha20Rng) -> Self {
        Self(rng)
    }
}
impl ::rand_core::TryRng for ChaCha20Rng {
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
impl ::rand_core_09::RngCore for ChaCha20Rng {
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
impl ::rand_core_06::RngCore for ChaCha20Rng {
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
impl ::rand_core::SeedableRng for ChaCha20Rng {
    type Seed = <::chacha20::ChaCha20Rng as ::rand_core::SeedableRng>::Seed;
    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(<::chacha20::ChaCha20Rng>::from_seed(seed))
    }
    #[inline]
    fn from_rng<R: ::rand_core::Rng + ?Sized>(source: &mut R) -> Self {
        Self::new(<::chacha20::ChaCha20Rng>::from_rng(source))
    }
    #[inline]
    fn try_from_rng<R: ::rand_core::TryRng + ?Sized>(source: &mut R) -> Result<Self, R::Error> {
        Ok(Self::new(<::chacha20::ChaCha20Rng>::try_from_rng(source)?))
    }
}
impl Default for ChaCha20Rng {
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
            let mut seed: <::chacha20::ChaCha8Rng as ::rand_core::SeedableRng>::Seed =
                Default::default();
            getrandom::fill(seed.as_mut()).expect("Unable to source entropy for initialisation");
            Self::from_seed(seed)
        }
    }
}

#[cfg(feature = "serialize")]
#[derive(serde::Serialize, serde::Deserialize)]
struct ChaCha20Core {
    seed: [u8; 32],
    stream: u64,
    word_pos: u128,
}

#[cfg(feature = "serialize")]
impl From<&ChaCha20Rng> for ChaCha20Core {
    #[inline]
    fn from(value: &ChaCha20Rng) -> Self {
        Self {
            seed: value.0.get_seed(),
            stream: value.0.get_stream(),
            word_pos: value.0.get_word_pos(),
        }
    }
}

#[cfg(feature = "serialize")]
impl From<ChaCha20Core> for ChaCha20Rng {
    #[inline]
    fn from(value: ChaCha20Core) -> Self {
        let mut rng = chacha20::ChaCha20Rng::from_seed(value.seed);

        rng.set_stream(value.stream);
        rng.set_word_pos(value.word_pos);

        Self(rng)
    }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for ChaCha20Rng {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ChaCha20Core::from(self).serialize(serializer)
    }
}

#[cfg(feature = "serialize")]
impl<'de> serde::Deserialize<'de> for ChaCha20Rng {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ChaCha20Rng::from(ChaCha20Core::deserialize(deserializer)?))
    }
}

impl From<::chacha20::ChaCha20Rng> for ChaCha20Rng {
    #[inline]
    fn from(value: ::chacha20::ChaCha20Rng) -> Self {
        Self::new(value)
    }
}
impl crate::EntropySource for ChaCha20Rng {}

impl crate::RemoteRng for ChaCha20Rng {}
