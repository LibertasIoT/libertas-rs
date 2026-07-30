use libertas_macros::{libertas_export, LibertasExport};

#[derive(LibertasExport)]
struct Status {
    #[libertas_formatted_text]
    message: String,
    #[libertas_formatted_text]
    payload: Vec<u8>,
}

#[libertas_export]
fn show(#[libertas_formatted_text] message: String) -> String {
    message
}

#[test]
fn formatted_text_is_accepted_as_parser_only_metadata() {
    let status = Status {
        message: show("Ready".to_string()),
        payload: vec![b'O', b'K'],
    };
    assert_eq!(status.message, "Ready");
    assert_eq!(status.payload, b"OK");
}
