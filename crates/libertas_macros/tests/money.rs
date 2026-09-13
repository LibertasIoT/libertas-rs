use libertas_macros::{LibertasExport, libertas_export, libertas_money};

#[libertas_money("USD", 2)]
type Usd = i64;

#[derive(LibertasExport)]
struct Amounts {
    #[libertas_money("USD", 2)]
    amount: Usd,
}

#[libertas_export]
fn configure(#[libertas_money("USD", 2)] amount: Usd) -> Amounts {
    Amounts { amount }
}

#[test]
fn money_preserves_exact_minor_units() {
    assert_eq!(configure(i64::MAX).amount, i64::MAX);
    assert_eq!(configure(-12345).amount, -12345);
}
