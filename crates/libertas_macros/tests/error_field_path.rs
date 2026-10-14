use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
#[libertas_response]
#[libertas_error]
struct Refused {
    #[libertas_error_field_path]
    field: Option<String>,
}

#[test]
fn error_field_path_helper_preserves_nullable_string_answers() {
    let mut reply = Refused { field: None };
    assert_eq!(reply.field, None);
    reply.field = Some("[incomes/2/box12/0/amount]".into());
    assert_eq!(reply.field.as_deref(), Some("[incomes/2/box12/0/amount]"));
}
