use libertas_macros::LibertasExport;
#[derive(LibertasExport)]
struct Schedule {
    #[libertas_ui_header]
    legacy: String,
    #[libertas_ui_header(use_parent_name = true)]
    parent: String,
    #[libertas_ui_header(use_parent_name = false)]
    field: String,
}
#[test]
fn header_options_preserve_payloads() {
    let value = Schedule {
        legacy: "old".into(),
        parent: "parent".into(),
        field: "field".into(),
    };
    assert_eq!(
        (
            value.legacy.as_str(),
            value.parent.as_str(),
            value.field.as_str()
        ),
        ("old", "parent", "field")
    );
}
