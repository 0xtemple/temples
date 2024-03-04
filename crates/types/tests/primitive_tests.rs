use gstd::{ActorId, TypeInfo};
use temple_types::utils::parse_type;

#[test]
fn parse_type_works() {
    #[derive(TypeInfo)]
    struct V1 {
        t1: u128,
        t2: u128,
        t3: String,
    }
    assert_eq!(
        (vec!["value".into()], vec!["U8".into()]),
        parse_type::<u8>()
    );
    assert_eq!(
        (vec!["value".into()], vec!["U16".into()]),
        parse_type::<u16>()
    );
    assert_eq!(
        (vec!["value".into()], vec!["U128".into()]),
        parse_type::<u128>()
    );
    assert_eq!(
        (vec!["value".into()], vec!["Bool".into()]),
        parse_type::<bool>()
    );
    assert_eq!(
        (vec!["value".into()], vec!["Bool".into()]),
        parse_type::<Vec<u8>>()
    );
    assert_eq!(
        (vec!["value".into()], vec!["ActorId".into()]),
        parse_type::<ActorId>()
    );
    assert_eq!(
        (
            vec!["t1".into(), "t2".into(), "t3".into()],
            vec!["u128".into(), "u128".into(), "String".into()]
        ),
        parse_type::<V1>()
    );
}
