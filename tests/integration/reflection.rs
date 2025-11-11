use bevy_prng::{ChaCha8Rng, ReflectRemoteRng, WyRand};
use bevy_rand::{seed::RngSeed, traits::SeedSource};
use bevy_reflect::{
    FromReflect, GetTypeRegistration, Reflect, TypeRegistry,
    serde::{
        ReflectDeserializer, ReflectSerializer, TypedReflectDeserializer, TypedReflectSerializer,
    },
};
use rand_core::{RngCore, SeedableRng};
use ron::to_string;
use serde::de::DeserializeSeed;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn rng_untyped_serialization() {
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

    assert!(value.represents::<RngSeed<WyRand>>());
    assert!(value.try_downcast_ref::<RngSeed<WyRand>>().is_some());

    let recreated = RngSeed::<WyRand>::from_reflect(value.as_ref()).unwrap();

    assert_eq!(val.clone_seed(), recreated.clone_seed());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[test]
fn remote_rng_reflection_works() {
    let mut registry = TypeRegistry::default();
    registry.register::<ChaCha8Rng>();
    registry.register_type_data::<ChaCha8Rng, ReflectRemoteRng>();

    let mut value: ChaCha8Rng = ChaCha8Rng::from_seed([7; 32]);

    let before = value.next_u32();

    let mut reflected_value: Box<dyn Reflect> = Box::new(value);

    let id = reflected_value.reflect_type_info().type_id();
    let reflect_rng = registry.get_type_data::<ReflectRemoteRng>(id).unwrap();

    let next = reflect_rng
        .get_mut(reflected_value.as_reflect_mut())
        .unwrap();

    let after = next.next_u32();

    assert_eq!(before, 1506529508);
    assert_eq!(after, 958315583);
}
