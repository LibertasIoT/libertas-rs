use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
enum Topic {
    #[libertas_protocol_variant("EditWages")]
    Wages = 7,
    #[libertas_protocol_variant("EditRetirement")]
    Retirement = 42,
}

#[test]
fn protocol_variant_preserves_enum_discriminants() {
    assert_eq!(Topic::Wages as i32, 7);
    assert_eq!(Topic::Retirement as i32, 42);
}
