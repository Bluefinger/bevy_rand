#[cfg(all(feature = "serialize", feature = "chacha20"))]
pub(crate) fn serialize_bytes<const BYTES: usize, S>(
    value: &[u8; BYTES],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_bytes(value)
}

#[cfg(all(feature = "serialize", feature = "chacha20"))]
pub(crate) fn deserialize_bytes<'de, const BYTES: usize, D>(deserializer: D) -> Result<[u8; BYTES], D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct ByteArrayVisitor<const LEN: usize>(core::marker::PhantomData<[(); LEN]>);

    impl<'de, const LEN: usize> serde::de::Visitor<'de> for ByteArrayVisitor<LEN> {
        type Value = [u8; LEN];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "Expected an array of length {}", LEN)
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.try_into()
                .map_err(|_| serde::de::Error::invalid_length(v.len(), &self))
        }
    }

    deserializer.deserialize_bytes(ByteArrayVisitor(core::marker::PhantomData))
}
