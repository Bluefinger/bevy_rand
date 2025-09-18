use bevy_prng::ChaCha8Rng;

use rand_core::{RngCore, SeedableRng};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn rng_untyped_serialization() {
    use bevy_reflect::{
        FromReflect, TypeRegistry,
        serde::{ReflectDeserializer, ReflectSerializer},
    };
    use ron::to_string;
    use serde::de::DeserializeSeed;

    let mut registry = TypeRegistry::default();
    registry.register::<ChaCha8Rng>();

    let mut val: ChaCha8Rng = ChaCha8Rng::from_seed([7; 32]);

    // Modify the state of the RNG instance
    val.next_u32();

    let ser = ReflectSerializer::new(&val, &registry);

    let serialized = to_string(&ser).unwrap();

    assert_eq!(
        &serialized,
        "{\"bevy_prng::ChaCha8Rng\":((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1))}"
    );

    let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

    let de = ReflectDeserializer::new(&registry);

    let value = de.deserialize(&mut deserializer).unwrap();

    let mut dynamic = ChaCha8Rng::take_from_reflect(value).unwrap();

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
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn rng_typed_serialization() {
    use bevy_reflect::{
        FromReflect, GetTypeRegistration, TypeRegistry,
        serde::{TypedReflectDeserializer, TypedReflectSerializer},
    };
    use ron::ser::to_string;
    use serde::de::DeserializeSeed;

    let mut registry = TypeRegistry::default();
    registry.register::<ChaCha8Rng>();

    let registered_type = ChaCha8Rng::get_type_registration();

    let mut val = ChaCha8Rng::from_seed([7; 32]);

    // Modify the state of the RNG instance
    val.next_u32();

    let ser = TypedReflectSerializer::new(&val, &registry);

    let serialized = to_string(&ser).unwrap();

    assert_eq!(
        &serialized,
        "((seed:(7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7),stream:0,word_pos:1))"
    );

    let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

    let de = TypedReflectDeserializer::new(&registered_type, &registry);

    let value = de.deserialize(&mut deserializer).unwrap();

    let mut dynamic = ChaCha8Rng::take_from_reflect(value).unwrap();

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
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn seed_reflection_serialization_round_trip() {
    use bevy_prng::WyRand;
    use bevy_rand::prelude::{RngSeed, SeedSource};
    use bevy_reflect::{
        FromReflect, GetTypeRegistration, TypeRegistry,
        serde::{TypedReflectDeserializer, TypedReflectSerializer},
    };
    use ron::to_string;
    use serde::de::DeserializeSeed;

    let mut registry = TypeRegistry::default();
    registry.register::<RngSeed<WyRand>>();

    let registered_type = RngSeed::<WyRand>::get_type_registration();

    let val = RngSeed::<WyRand>::from_seed(u64::MAX.to_ne_bytes());

    let ser = TypedReflectSerializer::new(&val, &registry);

    let serialized = to_string(&ser).unwrap();

    assert_eq!(&serialized, "(seed:(255,255,255,255,255,255,255,255))");

    let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

    let de = TypedReflectDeserializer::new(&registered_type, &registry);

    let value = de.deserialize(&mut deserializer).unwrap();

    assert!(value.is_dynamic());
    assert!(value.represents::<RngSeed<WyRand>>());
    assert!(value.try_downcast_ref::<RngSeed<WyRand>>().is_none());

    let recreated = RngSeed::<WyRand>::from_reflect(value.as_ref()).unwrap();

    assert_eq!(val.clone_seed(), recreated.clone_seed());
}
