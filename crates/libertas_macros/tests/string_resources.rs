// Deliberately invalid schema placements: these fixtures verify only that
// the compiler helper passes through items unchanged. Studio rejects both
// placements. Valid Apps declare their catalog in leading package docs with
// //! #[libertas_string_resources(APP_STRINGS)]
// Compiler acceptance must never be treated as schema validation.

use libertas_macros::libertas_string_resources;

#[allow(dead_code)]
const OUTPUT_STRINGS: &[(&str, &str)] = &[("READY", "Ready")];

#[libertas_string_resources(OUTPUT_STRINGS)]
fn run() -> bool {
    true
}

#[allow(dead_code)]
#[libertas_string_resources(OUTPUT_STRINGS)]
struct ReusableOutput {
    value: String,
}

#[test]
fn invalid_resource_placements_pass_through_for_studio_validation() {
    assert!(run());
    let value = ReusableOutput { value: String::new() };
    assert!(value.value.is_empty());
}
