//! Helpers for the LMB1 value inside a schema field annotated with `MimeTypes`.
//! Plain Avro Bytes remain opaque and do not use this envelope.

use alloc::vec::Vec;

const MAGIC: &[u8; 4] = b"LMB1";

/// A borrowed, defined MIME-aware Bytes value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MimeBytes<'a> {
    pub mime_type: &'a str,
    pub content: &'a [u8],
}

/// True for the exact lowercase ASCII MIME grammar used by `MimeTypes` values.
pub fn is_exact_mime_type(value: &str) -> bool {
    fn component(text: &str) -> bool {
        let bytes = text.as_bytes();
        (1..=127).contains(&bytes.len()) &&
            matches!(bytes[0], b'a'..=b'z' | b'0'..=b'9') &&
            bytes[1..].iter().all(|byte| matches!(byte,
                b'a'..=b'z' | b'0'..=b'9' | b'!' | b'#' | b'$' | b'&' |
                b'^' | b'_' | b'.' | b'+' | b'-'))
    }
    let mut parts = value.split('/');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(major), Some(minor), None) => component(major) && component(minor),
        _ => false,
    }
}

/// Match an exact value type against a canonical comma-delimited filter.
pub fn mime_type_matches_filter(mime_type: &str, filter: &str) -> bool {
    if !is_exact_mime_type(mime_type) { return false; }
    let major = mime_type.split('/').next().unwrap_or("");
    filter.split(',').any(|entry| entry == mime_type || entry == "*/*" ||
        entry.strip_suffix("/*") == Some(major))
}

/// Encode a defined media value. An empty byte string is instead the valid
/// media-undefined wire value and can be sent as ordinary empty Bytes.
pub fn encode_mime_bytes(mime_type: &str, content: &[u8]) -> Result<Vec<u8>, &'static str> {
    if !is_exact_mime_type(mime_type) || mime_type.len() > u8::MAX as usize {
        return Err("invalid MIME type");
    }
    let mut output = Vec::with_capacity(5 + mime_type.len() + content.len());
    output.extend_from_slice(MAGIC);
    output.push(mime_type.len() as u8);
    output.extend_from_slice(mime_type.as_bytes());
    output.extend_from_slice(content);
    Ok(output)
}

/// Decode an LMB1 value. `Ok(None)` means present empty Bytes (media undefined).
/// This helper borrows the original wire slice and does not copy content.
pub fn decode_mime_bytes(bytes: &[u8]) -> Result<Option<MimeBytes<'_>>, &'static str> {
    if bytes.is_empty() { return Ok(None); }
    if bytes.len() < 5 || &bytes[..4] != MAGIC { return Err("expected LMB1 MIME Bytes"); }
    let length = bytes[4] as usize;
    if length < 3 || bytes.len() < 5 + length { return Err("truncated MIME type"); }
    let mime_type = core::str::from_utf8(&bytes[5..5 + length]).map_err(|_| "invalid MIME type")?;
    if !is_exact_mime_type(mime_type) { return Err("invalid MIME type"); }
    Ok(Some(MimeBytes { mime_type, content: &bytes[5 + length..] }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_and_undefined() {
        assert_eq!(decode_mime_bytes(&[]).unwrap(), None);
        let value = encode_mime_bytes("image/png", &[1, 2, 3]).unwrap();
        assert_eq!(&value[..5], b"LMB1\x09");
        assert_eq!(decode_mime_bytes(&value).unwrap(), Some(MimeBytes {
            mime_type: "image/png", content: &[1, 2, 3],
        }));
        assert!(decode_mime_bytes(b"raw bytes").is_err());
        assert!(decode_mime_bytes(b"LMB1\x09image").is_err());
        assert!(mime_type_matches_filter("image/png", "image/*"));
        assert!(!mime_type_matches_filter("image/png", "application/*"));
    }
}
