//! Shared message definitions for Libertas Hub.
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
    #[libertas_request]
    #[libertas_subscription_request]
    LocationReq,

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
        let encoded = HubProtocol::LocationReq.to_avro();
        assert_eq!(
            HubProtocol::from_avro(&encoded),
            Ok(HubProtocol::LocationReq)
        );
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
