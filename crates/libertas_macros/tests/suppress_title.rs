use libertas_macros::{LibertasExport, libertas_suppress_title};

#[derive(LibertasExport)]
pub struct Line {
    #[libertas_read_only]
    pub label: String,
    pub amount: i64,
}

#[derive(LibertasExport)]
pub struct Report {
    #[libertas_suppress_title]
    pub lines: Vec<Line>,
}

#[libertas_suppress_title]
pub type Lines = Vec<Line>;

#[test]
fn suppress_title_preserves_native_types() {
    let lines: Lines = vec![Line {
        label: "Wages".into(),
        amount: 100,
    }];
    let report = Report { lines };
    assert_eq!(report.lines[0].label, "Wages");
    assert_eq!(report.lines[0].amount, 100);
}
