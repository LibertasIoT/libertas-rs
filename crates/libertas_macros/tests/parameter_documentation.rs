use libertas_macros::{libertas_data_error_status, libertas_export};

/// Example function
/// Accepts a documented configuration value.
#[libertas_data_error_status]
#[libertas_export]
fn documented(
    /// Daily limit
    /// The selected maximum number of minutes.
    minutes: u32,
) -> u32 {
    minutes
}

#[test]
fn documented_parameters_compile_and_preserve_values() {
    assert_eq!(documented(20), 20);
}
