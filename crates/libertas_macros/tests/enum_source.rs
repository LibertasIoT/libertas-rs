use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
struct Settings {
    choices: Vec<String>,
    #[libertas_enum_source("$.choices")]
    selected_choice: u16,
}

#[test]
fn enum_source_is_accepted_as_parser_only_metadata() {
    let settings = Settings {
        choices: vec!["first".to_string()],
        selected_choice: 0,
    };
    assert_eq!(settings.choices[settings.selected_choice as usize], "first");
}
