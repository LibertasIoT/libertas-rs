use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
#[libertas_request]
struct Page {
    #[libertas_hidden]
    #[libertas_reveal_path]
    fragment: Option<String>,
    #[libertas_hidden]
    #[libertas_landing_path]
    landing: String,
}

#[test]
fn page_path_helpers_preserve_server_values() {
    let page = Page {
        fragment: Some("[answer]".into()),
        landing: "[fallback]".into(),
    };
    assert_eq!(page.fragment.as_deref(), Some("[answer]"));
    assert_eq!(page.landing, "[fallback]");
}
