use libertas_macros::{libertas_export, libertas_permissions, LibertasExport};

#[allow(dead_code)]
const APP_PERMISSIONS: &[&str] = &["devices.read", "devices.control"];

#[libertas_permissions(APP_PERMISSIONS)]
fn configure_devices() -> bool {
    true
}

#[derive(LibertasExport)]
struct WeatherServer {
    #[libertas_endpoint_server]
    #[libertas_permissions(APP_PERMISSIONS)]
    endpoint: u32,
}

#[libertas_export]
fn use_weather(#[libertas_permissions(APP_PERMISSIONS)] endpoint: u32) -> u32 {
    endpoint
}

#[test]
fn permissions_attribute_preserves_the_function() {
    assert!(configure_devices());
    let server = WeatherServer { endpoint: 7 };
    assert_eq!(use_weather(server.endpoint), 7);
}
