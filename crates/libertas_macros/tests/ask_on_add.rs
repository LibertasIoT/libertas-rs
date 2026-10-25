use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
enum DocumentForm {
    W2 { wages: i64 },
    Form1099Int { interest: i64 },
}

#[derive(LibertasExport)]
struct DocumentItem {
    #[libertas_ask_on_add]
    form: DocumentForm,
    #[libertas_ask_on_add]
    payer: String,
    note: String,
}

#[test]
fn ask_on_add_is_a_parser_only_field_helper() {
    let item = DocumentItem { form: DocumentForm::W2 { wages: 1 }, payer: "A".into(), note: String::new() };
    assert!(matches!(item.form, DocumentForm::W2 { wages: 1 }));
    assert_eq!(item.payer, "A");
    assert!(item.note.is_empty());
    let _ = DocumentForm::Form1099Int { interest: 0 };
}
