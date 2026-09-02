use libertas_macros::{libertas_export, libertas_ui_include_description, LibertasExport};

#[libertas_ui_include_description]
type Mode = u32;

#[derive(LibertasExport)]
struct Settings {
    #[libertas_ui_include_description]
    mode: Mode,
}

#[libertas_export]
fn configure(#[libertas_ui_include_description] mode: Mode) -> Mode {
    mode
}

#[test]
fn ui_include_description_is_accepted_as_client_presentation_metadata() {
    let settings = Settings { mode: 7 };
    assert_eq!(configure(settings.mode), 7);
}
