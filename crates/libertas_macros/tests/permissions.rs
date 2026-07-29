use libertas_macros::libertas_permissions;

#[allow(dead_code)]
const APP_PERMISSIONS: &[&str] = &["devices.read", "devices.control"];

#[libertas_permissions(APP_PERMISSIONS)]
fn configure_devices() -> bool {
    true
}

#[test]
fn permissions_attribute_preserves_the_function() {
    assert!(configure_devices());
}
