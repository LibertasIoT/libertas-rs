use libertas_macros::{
    libertas_export, libertas_range_max, libertas_range_min, LibertasExport,
};

#[libertas_range_min("{$.minimum} | null")]
#[libertas_range_max("{$.maximum} | null")]
type Percentage = i32;

#[derive(LibertasExport)]
struct Settings {
    #[libertas_range_min("0")]
    #[libertas_range_max("100")]
    percentage: Percentage,
}

#[libertas_export]
fn configure(
    #[libertas_range_min("0")] #[libertas_range_max("100")] percentage: Percentage,
) -> Settings {
    Settings { percentage }
}

#[test]
fn range_attributes_are_compile_time_only() {
    assert_eq!(configure(42).percentage, 42);
}
