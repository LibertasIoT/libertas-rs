use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
#[libertas_llm_doc_trans(instruction = "Read the W2 exactly", validation = "@.facts")]
struct W2 {
    #[libertas_hidden]
    facts: Facts,
    #[libertas_llm_doc_field]
    name: String,
    #[libertas_default(0)]
    #[libertas_llm_doc_field(allow_empty = true)]
    tips: i64,
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
    };
    assert_eq!(form.facts.year, 2026);
    assert_eq!(form.name, "Sample Employee");
    assert_eq!(form.tips, 0);
}
