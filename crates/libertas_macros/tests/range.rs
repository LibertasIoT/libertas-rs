use libertas_macros::{LibertasExport, libertas_export, libertas_range_max, libertas_range_min};

#[libertas_range_min("{$.minimum} | null")]
#[libertas_range_max("{$.maximum} | null")]
type Percentage = i32;

const MINIMUM: i32 = 0;

#[derive(LibertasExport)]
struct Settings {
    #[libertas_number(min=$MINIMUM, max="[^/maximum]")]
    percentage: Percentage,
    maximum: i32,
}

#[libertas_export]
fn configure(
    #[libertas_range_min("0")]
    #[libertas_range_max("100")]
    percentage: Percentage,
) -> Settings {
    Settings { percentage, maximum: 100 }
}

#[test]
fn range_attributes_are_compile_time_only() {
    assert_eq!(configure(42).percentage, 42);
    assert_eq!(configure(42).maximum, 100);
    assert_eq!(MINIMUM, 0);
}
