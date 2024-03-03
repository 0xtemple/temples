use gstd::ActorId;
use temple_types::primitive::Primitive;
#[test]
fn inner_value_getter_setter() {
    let mut primitive = Primitive::U8(None);
    primitive.set_u8(Some(1u8)).unwrap();
    assert_eq!(primitive.as_u8(), Some(1u8));
    assert_eq!(primitive.to_string(), String::from("u8"));
    assert_eq!(primitive.to_numeric(), 0);

    let mut primitive = Primitive::U16(None);
    primitive.set_u16(Some(1u16)).unwrap();
    assert_eq!(primitive.as_u16(), Some(1u16));
    assert_eq!(primitive.to_string(), String::from("u16"));
    assert_eq!(primitive.to_numeric(), 1);

    let mut primitive = Primitive::U32(None);
    primitive.set_u32(Some(1u32)).unwrap();
    assert_eq!(primitive.to_string(), String::from("u32"));
    assert_eq!(primitive.as_u32(), Some(1u32));
    assert_eq!(primitive.to_numeric(), 2);

    let mut primitive = Primitive::U64(None);
    primitive.set_u64(Some(1u64)).unwrap();
    assert_eq!(primitive.to_string(), String::from("u64"));
    assert_eq!(primitive.as_u64(), Some(1u64));
    assert_eq!(primitive.to_numeric(), 3);

    let mut primitive = Primitive::U128(None);
    primitive.set_u128(Some(1u128)).unwrap();
    assert_eq!(primitive.to_string(), String::from("u128"));
    assert_eq!(primitive.as_u128(), Some(1u128));
    assert_eq!(primitive.to_numeric(), 4);

    let mut primitive = Primitive::USize(None);
    primitive.set_usize(Some(1u32)).unwrap();
    assert_eq!(primitive.to_string(), String::from("usize"));
    assert_eq!(primitive.as_usize(), Some(1u32));
    assert_eq!(primitive.to_numeric(), 5);

    let mut primitive = Primitive::Bool(None);
    primitive.set_bool(Some(true)).unwrap();
    assert_eq!(primitive.to_string(), String::from("bool"));
    assert_eq!(primitive.as_bool(), Some(true));
    assert_eq!(primitive.to_numeric(), 6);

    let mut primitive = Primitive::ActorId(None);
    primitive.set_actor_id(Some(ActorId::from(0u64))).unwrap();
    assert_eq!(primitive.to_string(), String::from("actorid"));
    assert_eq!(primitive.as_actor_id(), Some(ActorId::from(0u64)));
    assert_eq!(primitive.to_numeric(), 7);
}
