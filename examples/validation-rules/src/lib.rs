//! Validation-rule CI sample
//! #[libertas_types_only]
//! #[libertas_string_resources(MESSAGES)]
//! Run Studio's installed schema compiler launcher with `test-rules fixtures/*.json`.
//! The fixtures assert client rule outcomes without deploying or submitting data.

extern crate alloc;

use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode, LibertasExport};

pub const MESSAGES: &[(&str, &str)] = &[("RANGE_ORDER", "Minimum must not exceed maximum.")];

/// Range settings
#[derive(Debug, PartialEq, LibertasExport, LibertasAvroEncode, LibertasAvroDecode)]
#[libertas_validation_rules(r#"[minimum] <= [maximum] : "RANGE_ORDER";"#)]
pub struct Settings {
    /// Minimum allowed value
    pub minimum: i64,
    /// Maximum allowed value
    pub maximum: i64,
}
