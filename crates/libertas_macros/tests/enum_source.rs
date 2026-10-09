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

#[derive(LibertasExport)]
struct Report {
    #[libertas_key]
    #[libertas_unique]
    #[libertas_hidden]
    code: String,
    name: String,
}

#[derive(LibertasExport)]
struct ReportSettings {
    reports: Vec<Report>,
    #[libertas_enum_source("$.reports")]
    selected_report: String,
}

#[test]
fn key_is_accepted_as_parser_only_metadata() {
    let settings = ReportSettings {
        reports: vec![Report {
            code: "w2".to_string(),
            name: "Wages".to_string(),
        }],
        selected_report: "w2".to_string(),
    };
    let chosen = settings
        .reports
        .iter()
        .find(|report| report.code == settings.selected_report)
        .unwrap();
    assert_eq!(chosen.name, "Wages");
}
