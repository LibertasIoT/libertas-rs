use libertas_macros::{LibertasExport, libertas_access_host, libertas_export};

#[libertas_export]
#[libertas_access_host("api.example.com,telemetry.example.net")]
pub fn synchronize() {}

#[derive(LibertasExport)]
pub struct InvalidPlacementCompilesForStudioValidation {
    #[libertas_access_host("api.example.com")]
    pub value: u32,
}

#[test]
fn access_host_metadata_preserves_functions_and_fields() {
    synchronize();
    let value = InvalidPlacementCompilesForStudioValidation { value: 1 };
    assert_eq!(value.value, 1);
}
