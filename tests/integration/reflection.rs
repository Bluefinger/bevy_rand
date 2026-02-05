#[cfg(any(
    feature = "chacha20",
    feature = "wyrand",
    feature = "rand_pcg",
    feature = "rand_xoshiro"
))]
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
                val.next_u64();

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
                    val.next_u64(),
                    dynamic.next_u64(),
                    "The deserialized Entropy should have the same output as original"
                );
                assert_ne!(dynamic.next_u64(), dynamic.next_u64());
            }

            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            #[test]
            fn rng_typed_serialization() {
                let mut registry = TypeRegistry::default();
                registry.register::<$rng>();

                let registered_type = <$rng>::get_type_registration();

                let mut val = <$rng>::from_seed($seed);

                // Modify the state of the RNG instance
                val.next_u64();

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
                    val.next_u64(),
                    dynamic.next_u64(),
                    "The deserialized Entropy should have the same output as original"
                );
                assert_ne!(dynamic.next_u64(), dynamic.next_u64());
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

                let before = value.next_u64();

                let mut reflected_value: Box<dyn Reflect> = Box::new(value);

                let id = reflected_value.reflect_type_info().type_id();
                let reflect_rng = registry.get_type_data::<ReflectRemoteRng>(id).unwrap();

                let next = reflect_rng
                    .get_mut(reflected_value.as_reflect_mut())
                    .unwrap();

                let after = next.next_u64();
                let after_after = next.next_u64();

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
    "{\"bevy_prng::ChaCha8Rng\":(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:2)}",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:2)",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    4115934089738703076,
    15345232379140719590
);

#[cfg(feature = "chacha20")]
reflection_test!(
    chacha12,
    bevy_prng::ChaCha12Rng,
    [7; 32],
    "{\"bevy_prng::ChaCha12Rng\":(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:2)}",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:2)",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    2363194108971422454,
    13552751203817743523
);

#[cfg(feature = "chacha20")]
reflection_test!(
    chacha20,
    bevy_prng::ChaCha20Rng,
    [7; 32],
    "{\"bevy_prng::ChaCha20Rng\":(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:2)}",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:2)",
    "(seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7))",
    4753173749397848308,
    8104706558872646932
);

#[cfg(feature = "wyrand")]
reflection_test!(
    wyrand,
    bevy_prng::WyRand,
    u64::MAX.to_ne_bytes(),
    "{\"bevy_prng::WyRand\":((state:3257665815644502180))}",
    "((state:3257665815644502180))",
    "(seed:(255,255,255,255,255,255,255,255))",
    1205299102744794270,
    2332786255384219817
);

#[cfg(feature = "rand_pcg")]
reflection_test!(
    pcg32,
    bevy_prng::Pcg32,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    "{\"bevy_prng::Pcg32\":((state:15033853422540656993,increment:1157159078456920585))}",
    "((state:15033853422540656993,increment:1157159078456920585))",
    "(seed:(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16))",
    1204678643940597513,
    12029084591851635269
);

#[cfg(feature = "rand_pcg")]
reflection_test!(
    pcg64,
    bevy_prng::Pcg64,
    [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32
    ],
    "{\"bevy_prng::Pcg64\":((state:172305881977888272371905305222824952168,increment:42696867846335054569745073772176806417))}",
    "((state:172305881977888272371905305222824952168,increment:42696867846335054569745073772176806417))",
    "(seed:(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32))",
    8740028313290271629,
    10342282812839511965
);

#[cfg(feature = "rand_pcg")]
reflection_test!(
    pcg64mcg,
    bevy_prng::Pcg64Mcg,
    42u128.to_ne_bytes(),
    "{\"bevy_prng::Pcg64Mcg\":((state:320716815976818922153327884990172454295))}",
    "((state:320716815976818922153327884990172454295))",
    "(seed:(42,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0))",
    0x63b4a3a813ce700a,
    0x382954200617ab24
);

#[cfg(feature = "rand_pcg")]
reflection_test!(
    pcg64dxsm,
    bevy_prng::Pcg64Dxsm,
    [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32
    ],
    "{\"bevy_prng::Pcg64Dxsm\":((state:248028475877024480770638062163604013976,increment:42696867846335054569745073772176806417))}",
    "((state:248028475877024480770638062163604013976,increment:42696867846335054569745073772176806417))",
    "(seed:(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32))",
    12201417210360370199,
    1479060906603667107
);

#[cfg(feature = "rand_xoshiro")]
reflection_test!(
    xoshiro512starstar,
    ::bevy_prng::Xoshiro512StarStar,
    ::bevy_prng::Seed512::from([
        1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0,
        0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0,
        0, 0, 0, 0
    ]),
    "{\"bevy_prng::Xoshiro512StarStar\":((s:(6,0,2,1,1,4,4107,25165824)))}",
    "((s:(6,0,2,1,1,4,4107,25165824)))",
    "(seed:((1,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,5,0,0,0,0,0,0,0,6,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0)))",
    11520,
    0
);

#[cfg(feature = "rand_xoshiro")]
reflection_test!(
    xoshiro256starstar,
    bevy_prng::Xoshiro256StarStar,
    [
        1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0,
        0, 0,
    ],
    "{\"bevy_prng::Xoshiro256StarStar\":((s:(7,0,262146,211106232532992)))}",
    "((s:(7,0,262146,211106232532992)))",
    "(seed:(1,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0))",
    11520,
    0
);
