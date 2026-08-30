use libertas_macros::{libertas_export, LibertasExport};

#[derive(LibertasExport)]
enum Protocol {
    #[libertas_request]
    #[libertas_exclude_ui]
    InternalRequest(u32),
    #[libertas_response]
    #[libertas_error]
    VisibleResponse(u32),
}

#[derive(LibertasExport)]
struct Server {
    #[libertas_endpoint_server]
    #[libertas_endpoint_schema(Protocol)]
    #[libertas_exclude_ui]
    endpoint: u32,
}

#[libertas_export]
fn configure(
    #[libertas_endpoint_server]
    #[libertas_exclude_ui]
    endpoint: u32,
) -> u32 {
    endpoint
}

#[test]
fn exclude_ui_is_accepted_as_parser_only_metadata() {
    let server = Server { endpoint: 7 };
    assert_eq!(configure(server.endpoint), 7);
    assert!(matches!(
        Protocol::InternalRequest(1),
        Protocol::InternalRequest(1)
    ));
    assert!(matches!(
        Protocol::VisibleResponse(2),
        Protocol::VisibleResponse(2)
    ));
}
