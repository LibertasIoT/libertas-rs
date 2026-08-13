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
fn string_resources_compile_on_functions_and_named_types() {
    assert!(run());
    let value = ReusableOutput { value: String::new() };
    assert!(value.value.is_empty());
}
