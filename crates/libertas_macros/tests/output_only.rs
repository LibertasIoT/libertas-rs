use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
#[libertas_request]
struct Page {
    name: String,
    #[libertas_read_only]
    #[libertas_output_only]
    summary: String,
    #[libertas_read_only]
    #[libertas_output_only]
    rows: Vec<String>,
}

#[test]
fn output_only_helper_preserves_the_field_values() {
    let page = Page { name: "a".into(), summary: "shown".into(), rows: vec!["x".into()] };
    assert_eq!(page.name, "a");
    assert_eq!(page.summary, "shown");
    assert_eq!(page.rows, vec!["x".to_string()]);
}
