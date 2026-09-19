use libertas_macros::libertas_ui_header;

#[libertas_ui_header]
type FlatList = Vec<u32>;

#[libertas_ui_header(use_parent_name = false)]
type FlatListWithFalse = Vec<u32>;

#[test]
fn header_aliases_preserve_native_types() {
    let original: Vec<u32> = vec![1, 2];
    let flat: FlatList = original;
    let explicit_false: FlatListWithFalse = flat;
    assert_eq!(explicit_false, [1, 2]);
}
