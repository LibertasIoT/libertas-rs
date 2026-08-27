use libertas_macros::{LibertasExport, libertas_export};

#[derive(LibertasExport)]
struct ClientEndpoints {
    #[libertas_default("acme-weather::serve_weather")]
    recommended: u32,
    #[libertas_fixed("acme-weather::serve_weather")]
    required: u32,
}

#[libertas_export]
fn configure(
    #[libertas_default("acme-weather::serve_weather")] recommended: u32,
    #[libertas_fixed("acme-weather::serve_weather")] required: u32,
) -> u32 {
    recommended + required
}

#[test]
fn endpoint_task_process_metadata_compiles_without_runtime_effect() {
    let endpoints = ClientEndpoints {
        recommended: 3,
        required: 4,
    };
    assert_eq!(configure(endpoints.recommended, endpoints.required), 7);
}
