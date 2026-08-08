use libertas_macros::libertas_foreign_type;

#[libertas_foreign_type(origin-shared::OfficialValue)]
type CompatibleValue = u32;

#[test]
fn foreign_type_accepts_a_published_compatible_type_alias() {
    let value: CompatibleValue = 42;
    assert_eq!(value, 42);
}
