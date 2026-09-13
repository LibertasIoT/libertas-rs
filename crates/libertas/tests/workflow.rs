extern crate alloc;

use libertas::{AvroDecode, AvroEncode};
use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode, LibertasExport};

#[derive(LibertasExport, LibertasAvroEncode, LibertasAvroDecode, Debug, PartialEq)]
enum ProfileProtocol {
    #[libertas_request]
    #[libertas_next_response("Edit,Start,Finished")]
    Reopen { cookie: u64 },
    #[libertas_request]
    #[libertas_prev_request("Reopen")]
    Edit { cookie: u64, name: String },
    #[libertas_response]
    #[libertas_workflow_request("Edit")]
    Start { cookie: u64 },
    #[libertas_response]
    Finished,
}

#[test]
fn workflow_annotations_preserve_payload_encoding() {
    let value = ProfileProtocol::Edit { cookie: 42, name: "Profile".into() };
    let mut bytes = Vec::new();
    value.avro_encode(&mut bytes);
    let mut offset = 0;
    assert_eq!(ProfileProtocol::avro_decode(&bytes, &mut offset).unwrap(), value);
    assert_eq!(offset, bytes.len());
}
