use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
#[libertas_read_only]
struct ReviewItem {
    label: String,
    #[libertas_hidden]
    #[libertas_reveal_path]
    field: Option<String>,
}

#[test]
fn reveal_path_is_schema_metadata_and_preserves_the_nullable_path() {
    let item = ReviewItem {
        label: "Choose a credit".into(),
        field: Some("[value/students/0/method]".into()),
    };
    assert_eq!(item.label, "Choose a credit");
    assert_eq!(item.field.as_deref(), Some("[value/students/0/method]"));
    let item = ReviewItem {
        label: "Review".into(),
        field: None,
    };
    assert_eq!(item.field, None);
}
