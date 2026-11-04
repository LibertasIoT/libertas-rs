use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
#[libertas_llm_doc_trans(instruction = "Read the W2 exactly", validation = "@.facts", form_date_order = "MonthDayYear")]
struct W2 {
    #[libertas_hidden]
    facts: Facts,
    #[libertas_llm_doc_field]
    name: String,
    #[libertas_default(0)]
    #[libertas_llm_doc_field(allow_empty = true)]
    tips: i64,
    #[libertas_llm_doc_field]
    filing: Filing,
}

#[derive(LibertasExport)]
enum Filing {
    #[libertas_form_code("F")]
    ByAgreement,
    #[libertas_form_code("G")]
    Other,
}

#[derive(LibertasExport)]
struct Facts {
    year: u16,
}

#[test]
fn document_helpers_preserve_native_structs() {
    let form = W2 {
        facts: Facts { year: 2026 },
        name: "Sample Employee".into(),
        tips: 0,
        filing: Filing::ByAgreement,
    };
    assert_eq!(form.facts.year, 2026);
    assert_eq!(form.name, "Sample Employee");
    assert_eq!(form.tips, 0);
    assert!(matches!(form.filing, Filing::ByAgreement));
    let _ = Filing::Other;
}
