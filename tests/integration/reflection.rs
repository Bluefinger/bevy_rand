#[cfg(any(feature = "chacha20", feature = "wyrand", feature = "rand_pcg"))]
macro_rules! reflection_test {
    ($name:ident, $rng:ty, $seed:expr, $untyped:literal, $typed:literal, $seed_cmp:literal, $before:literal, $after:literal) => {
        mod $name {
            #[cfg(target_arch = "wasm32")]
            use wasm_bindgen_test::*;

            use bevy_prng::ReflectRemoteRng;
            use bevy_rand::{seed::RngSeed, traits::SeedSource};
            use bevy_reflect::{
                FromReflect, GetTypeRegistration, Reflect, TypeRegistry,
                serde::{
                    ReflectDeserializer, ReflectSerializer, TypedReflectDeserializer,
                    TypedReflectSerializer,
                },
            };
            use rand_core::{Rng, SeedableRng};
            use ron::to_string;
            use serde::de::DeserializeSeed;

            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            #[test]
            fn rng_untyped_serialization() {
                let mut registry = TypeRegistry::default();
                registry.register::<$rng>();

                let mut val: $rng = <$rng>::from_seed($seed);

                // Modify the state of the RNG instance
                val.next_u32();

                let ser = ReflectSerializer::new(&val, &registry);

                let serialized = to_string(&ser).unwrap();

                assert_eq!(&serialized, $untyped);

                let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

                let de = ReflectDeserializer::new(&registry);

                let value = de.deserialize(&mut deserializer).unwrap();

                assert!(value.represents::<$rng>());
                assert!(value.try_downcast_ref::<$rng>().is_some());

                let mut dynamic = <$rng>::take_from_reflect(value).unwrap();

                // The two instances should be the same
                assert_eq!(
                    val, dynamic,
                    "The deserialized Entropy should equal the original"
                );
                // They should output the same numbers, as no state is lost between serialization and deserialization.
                assert_eq!(
                    val.next_u32(),
                    dynamic.next_u32(),
                    "The deserialized Entropy should have the same output as original"
                );
                assert_ne!(dynamic.next_u32(), dynamic.next_u32());
            }

            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            #[test]
            fn rng_typed_serialization() {
                let mut registry = TypeRegistry::default();
                registry.register::<$rng>();

                let registered_type = <$rng>::get_type_registration();

                let mut val = <$rng>::from_seed($seed);

                // Modify the state of the RNG instance
                val.next_u32();

                let ser = TypedReflectSerializer::new(&val, &registry);

                let serialized = to_string(&ser).unwrap();

                assert_eq!(&serialized, $typed);

                let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

                let de = TypedReflectDeserializer::new(&registered_type, &registry);

                let value = de.deserialize(&mut deserializer).unwrap();

                assert!(value.represents::<$rng>());
                assert!(value.try_downcast_ref::<$rng>().is_some());

                let mut dynamic = <$rng>::take_from_reflect(value).unwrap();

                // The two instances should be the same
                assert_eq!(
                    val, dynamic,
                    "The deserialized Entropy should equal the original"
                );
                // They should output the same numbers, as no state is lost between serialization and deserialization.
                assert_eq!(
                    val.next_u32(),
                    dynamic.next_u32(),
                    "The deserialized Entropy should have the same output as original"
                );
                assert_ne!(dynamic.next_u32(), dynamic.next_u32());
            }

            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            #[test]
            fn seed_reflection_serialization_round_trip() {
                let mut registry = TypeRegistry::default();
                registry.register::<RngSeed<$rng>>();

                let registered_type = RngSeed::<$rng>::get_type_registration();

                let val = RngSeed::<$rng>::from_seed($seed);

                let ser = TypedReflectSerializer::new(&val, &registry);

                let serialized = to_string(&ser).unwrap();

                assert_eq!(&serialized, $seed_cmp);

                let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

                let de = TypedReflectDeserializer::new(&registered_type, &registry);

                let value = de.deserialize(&mut deserializer).unwrap();

                assert!(value.represents::<RngSeed<$rng>>());
                assert!(value.try_downcast_ref::<RngSeed<$rng>>().is_some());

                let recreated = RngSeed::<$rng>::from_reflect(value.as_ref()).unwrap();

                assert_eq!(val.clone_seed(), recreated.clone_seed());
            }

            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            #[test]
            fn remote_rng_reflection_works() {
                let mut registry = TypeRegistry::default();
                registry.register::<$rng>();
                registry.register_type_data::<$rng, ReflectRemoteRng>();

                let mut value: $rng = <$rng>::from_seed($seed);

                let before = value.next_u32();

                let mut reflected_value: Box<dyn Reflect> = Box::new(value);

                let id = reflected_value.reflect_type_info().type_id();
                let reflect_rng = registry.get_type_data::<ReflectRemoteRng>(id).unwrap();

                let next = reflect_rng
                    .get_mut(reflected_value.as_reflect_mut())
                    .unwrap();

                let after = next.next_u32();
                let after_after = next.next_u32();

                assert_eq!(before, $before);
                assert_eq!(after, $after);
                assert_ne!(after, after_after);
            }
        }
    };
}

#[cfg(feature = "chacha20")]
reflection_test!(
    chacha8,
    bevy_prng::ChaCha8Rng,
    [7; 32],
    "{\"bevy_prng::ChaCha8Rng\":(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)}",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    1506529508,
    958315583
);

#[cfg(feature = "chacha20")]
reflection_test!(
    chacha12,
    bevy_prng::ChaCha12Rng,
    [7; 32],
    "{\"bevy_prng::ChaCha12Rng\":(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)}",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    2022281974,
    550224005
);

#[cfg(feature = "chacha20")]
reflection_test!(
    chacha20,
    bevy_prng::ChaCha20Rng,
    [7; 32],
    "{\"bevy_prng::ChaCha20Rng\":(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)}",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1)",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    2022834420,
    1106684503
);

#[cfg(feature = "wyrand")]
reflection_test!(
    wyrand,
    bevy_prng::WyRand,
    u64::MAX.to_ne_bytes(),
    "{\"bevy_prng::WyRand\":((state:3257665815644502180))}",
    "((state:3257665815644502180))",
    "(seed:(255,255,255,255,255,255,255,255))",
    3811792030,
    1494683817
);

#[cfg(feature = "rand_pcg")]
reflection_test!(
    pcg32,
    bevy_prng::Pcg32,
    [7; 16],
    "{\"bevy_prng::Pcg32\":((state:15254884040922037504,increment:506381209866536711))}",
    "((state:15254884040922037504,increment:506381209866536711))",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    2404370353,
    2688997533
);

#[cfg(feature = "rand_pcg")]
reflection_test!(
    pcg64,
    bevy_prng::Pcg64,
    [7; 32],
    "{\"bevy_prng::Pcg64\":((state:95725369878262934946898689617630105672,increment:9341084582143408800955381380479911687))}",
    "((state:95725369878262934946898689617630105672,increment:9341084582143408800955381380479911687))",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    3536411370,
    1127007220
);

#[cfg(feature = "rand_pcg")]
reflection_test!(
    pcg64mcg,
    bevy_prng::Pcg64Mcg,
    [7; 16],
    "{\"bevy_prng::Pcg64Mcg\":((state:185530775039669764831119355247077203683))}",
    "((state:185530775039669764831119355247077203683))",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    526624330,
    1067774782
);
