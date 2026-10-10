use libertas::{
    AvroDecode, AvroEncode, LibertasDevice, LibertasEndpoint, LibertasId, LibertasLanDevice,
    LibertasPeer, LibertasTaskId, LibertasUser, LibertasVirtualDevice,
    data::{KeyCodec, KeyDecode},
};
use libertas_macros::{LibertasKeyCodec, LibertasKeyDecode};

#[derive(Debug, PartialEq, LibertasKeyCodec, LibertasKeyDecode)]
enum IdKey {
    Ids {
        user: LibertasUser,
        device: LibertasDevice,
        virtual_device: LibertasVirtualDevice,
        endpoint: LibertasEndpoint,
        lan_device: LibertasLanDevice,
        task: LibertasTaskId,
        peer: LibertasPeer,
    },
}

#[test]
fn system_ids_use_signed_eight_byte_keys_and_numeric_order() {
    let cases: [(LibertasId, [u8; 8]); 5] = [
        (i64::MIN, [0; 8]),
        (-2, [127, 255, 255, 255, 255, 255, 255, 254]),
        (0, [128, 0, 0, 0, 0, 0, 0, 0]),
        (4_294_967_312, [128, 0, 0, 1, 0, 0, 0, 16]),
        (i64::MAX, [255; 8]),
    ];
    let mut previous = None;
    for (id, payload) in cases {
        let key = IdKey::Ids {
            user: id,
            device: id,
            virtual_device: id,
            endpoint: id,
            lan_device: id,
            task: id,
            peer: id,
        };
        let bytes = key.to_key_bytes().unwrap();
        assert_eq!(bytes.len(), 2 + 7 * 8);
        assert_eq!(&bytes[..2], [0, 0]);
        for field in bytes[2..].chunks_exact(8) {
            assert_eq!(field, payload);
        }
        assert_eq!(IdKey::decode_key(&bytes).unwrap(), key);
        if let Some(before) = previous {
            assert!(before < bytes);
        }
        previous = Some(bytes);
    }
}

#[test]
fn signed_ids_preserve_positive_avro_bytes_and_roundtrip_dynamic_ids() {
    for id in [i64::MIN, -2, -1, 0, 4_294_967_312, i64::MAX] {
        let value: LibertasId = id;
        let mut bytes = Vec::new();
        value.avro_encode(&mut bytes);
        let mut offset = 0;
        assert_eq!(LibertasId::avro_decode(&bytes, &mut offset), Ok(value));
        assert_eq!(offset, bytes.len());
        if value >= 0 {
            let mut old_bytes = Vec::new();
            (value as u64).avro_encode(&mut old_bytes);
            assert_eq!(bytes, old_bytes);
        }
    }
    assert_eq!(libertas::LIBERTAS_BROADCAST_DEST, -1);
}
