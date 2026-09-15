use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
struct LargePage {
    #[libertas_cache_identity]
    workflow: String,
    #[libertas_cache_identity]
    revision: u32,
    entries: Vec<String>,
}

#[test]
fn cache_identity_is_accepted_as_parser_metadata_on_multiple_fields() {
    let page = LargePage { workflow: "test".into(), revision: 3, entries: vec![] };
    assert_eq!((page.workflow.as_str(), page.revision, page.entries.len()), ("test", 3, 0));
}
