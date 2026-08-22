//! Shared message definitions for Libertas Hub.
//! #[libertas_types_only]
//!
//! This crate contains definitions only. It does not implement Hub operations
//! or transport behavior.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode, LibertasExport};

/// Messages supported by the Libertas Hub protocol.
#[derive(Clone, Copy, Debug, LibertasAvroDecode, LibertasAvroEncode, LibertasExport, PartialEq)]
pub enum HubProtocol {
    /// Requests the Hub's current location.
    ///
    /// This message may be sent as either a request or a subscription request.
    /// `max_report_interval_seconds` is ignored for a one-shot request. For a
    /// subscription it must be greater than zero and limits the time between
    /// location reports.
    #[libertas_request]
    // Reading location does not require write access to the Hub endpoint.
    #[libertas_access_privilege("Read")]
    #[libertas_subscription_request]
    LocationReq {
        /// Maximum interval between subscription reports, in seconds.
        max_report_interval_seconds: u32,
    },

    /// Returns the Hub's current location.
    ///
    /// This message may be sent as either a response or subscription data.
    #[libertas_response]
    #[libertas_subscription_data]
    LocationRsp {
        /// Longitude in decimal degrees.
        longitude: f64,
        /// Latitude in decimal degrees.
        latitude: f64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_request_round_trips_through_avro() {
        let request = HubProtocol::LocationReq {
            max_report_interval_seconds: 300,
        };
        let encoded = request.to_avro();
        assert_eq!(encoded.as_slice(), &[0, 0xd8, 0x04]);
        assert_eq!(HubProtocol::from_avro(&encoded), Ok(request));
    }

    #[test]
    fn location_response_round_trips_through_avro() {
        let response = HubProtocol::LocationRsp {
            longitude: -73.985_664,
            latitude: 40.748_514,
        };
        let encoded = response.to_avro();
        assert_eq!(HubProtocol::from_avro(&encoded), Ok(response));
    }
}
