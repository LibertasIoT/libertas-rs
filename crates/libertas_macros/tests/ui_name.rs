use libertas_macros::{libertas_suppress_title, libertas_ui_name, LibertasExport};

#[derive(LibertasExport)]
pub struct Line {
    #[libertas_read_only]
    #[libertas_ui_name(amount)]
    pub label: String,
    pub amount: i64,
}

#[derive(LibertasExport)]
pub struct Report {
    #[libertas_suppress_title]
    pub lines: Vec<Line>,
}

#[libertas_ui_name(amount)]
pub type NamedLine = Line;

#[libertas_suppress_title]
pub type Lines = Vec<Line>;

#[test]
fn helpers_compile() {
    let _ = Report { lines: vec![Line { label: String::new(), amount: 0 }] };
}
