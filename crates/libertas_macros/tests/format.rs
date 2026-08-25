use libertas_macros::{LibertasExport, libertas_export, libertas_format};

#[libertas_format("#,##0.00")]
type Amount = f64;

#[derive(LibertasExport)]
struct FormattedValues {
    #[libertas_format("#,##0.00")]
    amount: Amount,
}

#[libertas_export]
fn configure(#[libertas_format("#,##0.00")] amount: Amount) -> FormattedValues {
    FormattedValues { amount }
}

#[test]
fn format_is_accepted_as_parser_only_metadata() {
    let values = configure(1234.5);
    assert_eq!(values.amount, 1234.5);
}
