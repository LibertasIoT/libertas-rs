//! Shared message definitions for Libertas Hub.
//!
//! This crate contains protocol definitions only. It has no runtime, transport,
//! serialization, allocation, or platform dependencies.

#![no_std]
#![forbid(unsafe_code)]

/// Operation carried by a Libertas endpoint message.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EndpointOperation {
    /// Request to subscribe to an endpoint.
    SubscriptionRequest = 3,
    /// Data report from an endpoint.
    Data = 5,
    /// Request that expects a response.
    Request = 8,
    /// Response to a request or subscription request.
    Response = 9,
    /// Authoritative notification that the peer process is down.
    PeerDown = 20,
    /// Notification that the peer cannot be reached and its status is unknown.
    PeerTimeout = 21,
}

/// Endpoint message status carried in the first payload byte.
///
/// [`Self::Success`] is followed by exactly one Avro datum.
/// [`Self::InvalidMessage`] has no Avro body and may be sent in either
/// direction. Peer-status notifications have no payload and therefore do not
/// carry this status byte.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EndpointMessageStatus {
    /// The message is valid.
    Success = 0,
    /// The peer could not accept or decode the message.
    InvalidMessage = 1,
}

/// Endpoint subscription request opcode.
pub const OP_ENDPOINT_SUB_REQ: u8 = EndpointOperation::SubscriptionRequest as u8;
/// Endpoint data message opcode.
pub const OP_ENDPOINT_DATA: u8 = EndpointOperation::Data as u8;
/// Endpoint request opcode.
pub const OP_ENDPOINT_REQ: u8 = EndpointOperation::Request as u8;
/// Endpoint response opcode.
pub const OP_ENDPOINT_RSP: u8 = EndpointOperation::Response as u8;
/// Authoritative peer-down notification opcode.
pub const OP_ENDPOINT_PEER_DOWN: u8 = EndpointOperation::PeerDown as u8;
/// Peer network-timeout notification opcode.
pub const OP_ENDPOINT_PEER_TIMEOUT: u8 = EndpointOperation::PeerTimeout as u8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_wire_values_are_stable() {
        assert_eq!(EndpointMessageStatus::Success as u8, 0);
        assert_eq!(EndpointMessageStatus::InvalidMessage as u8, 1);
        assert_eq!(OP_ENDPOINT_SUB_REQ, 3);
        assert_eq!(OP_ENDPOINT_DATA, 5);
        assert_eq!(OP_ENDPOINT_REQ, 8);
        assert_eq!(OP_ENDPOINT_RSP, 9);
        assert_eq!(OP_ENDPOINT_PEER_DOWN, 20);
        assert_eq!(OP_ENDPOINT_PEER_TIMEOUT, 21);
    }
}
