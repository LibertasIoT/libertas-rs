use libertas_macros::{LibertasExport, libertas_export, libertas_validation_rules};

#[libertas_validation_rules("[] == null || [] >= 0;")]
type NonNegative = i32;

#[derive(LibertasExport)]
struct Settings {
    #[libertas_validation_rules("[] == null || [] <= 100;")]
    percentage: NonNegative,
}

#[libertas_export]
#[libertas_validation_rules("[minimum] == null || [maximum] == null || [minimum] <= [maximum];")]
fn configure(
    #[libertas_validation_rules("[] == null || [] >= 0;")] minimum: i32,
    maximum: i32,
) -> Settings {
    Settings {
        percentage: minimum.min(maximum),
    }
}

#[test]
fn validation_rule_attributes_are_compile_time_only() {
    assert_eq!(configure(20, 30).percentage, 20);
}
