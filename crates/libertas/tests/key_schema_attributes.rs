extern crate alloc;

use libertas::data::{
    KeyCodec, KeyDecode, KeyDecodeError, KeyEncodeError, KeyFieldCodec, KeyFieldDecode,
};
use libertas_macros::{
    LibertasAvroDecode, LibertasAvroEncode, LibertasKeyCodec, LibertasKeyDecode,
    LibertasKeyFieldCodec, LibertasKeyFieldDecode, libertas_data_schema,
};

#[derive(Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Exact<T, const N: usize> {
    #[libertas_size(min = N, max = N)]
    value: T,
}

#[derive(Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Bounded {
    #[libertas_size(min = 2, max = 4)]
    value: Option<String>,
}

#[derive(Debug, PartialEq, LibertasKeyCodec, LibertasKeyDecode)]
enum Keys {
    Singleton,
    Exact(Exact<Option<String>, 4>),
    Bounded(Bounded),
}

#[derive(LibertasAvroEncode, LibertasAvroDecode)]
enum Data {
    Record(u32),
}

#[libertas_data_schema(value = Data, key = Keys)]
fn app() -> Data {
    Data::Record(3)
}

fn bytes<T: KeyFieldCodec>(value: &T) -> Vec<u8> {
    let mut result = vec![255; T::ENCODED_SIZE];
    value.encode_field(&mut result).unwrap();
    result
}

#[test]
fn shared_sizes_select_exact_bytes_and_count_utf8_with_optional_presence() {
    assert_eq!(
        bytes(&Exact::<_, 4> {
            value: "🦀".to_string()
        }),
        "🦀".as_bytes()
    );
    assert_eq!(
        bytes(&Exact::<_, 4> {
            value: Some("🦀".to_string())
        }),
        [1, 240, 159, 166, 128]
    );
    assert_eq!(bytes(&Exact::<Option<String>, 4> { value: None }), [0; 5]);
    assert_eq!(
        bytes(&Exact::<_, 2> {
            value: vec![0, 255]
        }),
        [0, 255]
    );
    assert_eq!(Exact::<Vec<u8>, 0>::ENCODED_SIZE, 0);
    assert_eq!(
        Exact::<Vec<u8>, 0>::decode_field(&[]).unwrap().value,
        Vec::<u8>::new()
    );
    assert_eq!(Exact::<Option<String>, 0>::ENCODED_SIZE, 1);
    assert_eq!(
        Exact::<Option<String>, 0>::decode_field(&[1])
            .unwrap()
            .value,
        Some(String::new())
    );
    assert_eq!(
        Exact::<String, 2>::decode_field(&[255, 0]),
        Err(KeyDecodeError::InvalidUtf8)
    );
    let key = Keys::Exact(Exact {
        value: Some("🦀".into()),
    });
    let encoded = key.to_key_bytes().unwrap();
    assert_eq!(encoded, [0, 1, 1, 240, 159, 166, 128]);
    assert_eq!(Keys::decode_key(&encoded).unwrap(), key);
}

#[test]
fn bounds_apply_to_present_content_without_changing_absence_or_padding() {
    assert_eq!(Bounded::ENCODED_SIZE, 9);
    assert_eq!(bytes(&Bounded { value: None }), [0; 9]);
    assert_eq!(
        bytes(&Bounded {
            value: Some("é".into())
        }),
        [1, 195, 169, 0, 0, 0, 0, 0, 2]
    );
    assert_eq!(
        Bounded {
            value: Some("a".into())
        }
        .encode_field(&mut [0; 9]),
        Err(KeyEncodeError::BelowMinimum {
            minimum: 2,
            actual: 1
        })
    );
    assert_eq!(
        Bounded::decode_field(&[1, b'a', 0, 0, 0, 0, 0, 0, 1]),
        Err(KeyDecodeError::BelowMinimum {
            minimum: 2,
            actual: 1
        })
    );
    assert_eq!(
        Exact::<_, 2> { value: vec![1] }.encode_field(&mut [0; 2]),
        Err(KeyEncodeError::BelowMinimum {
            minimum: 2,
            actual: 1
        })
    );
    assert_eq!(
        Exact::<_, 2> {
            value: vec![1, 2, 3]
        }
        .encode_field(&mut [0; 2]),
        Err(KeyEncodeError::CapacityExceeded {
            capacity: 2,
            actual: 3
        })
    );
}

#[test]
fn paired_schema_attribute_preserves_function_and_checks_both_codecs() {
    let Data::Record(value) = app();
    assert_eq!(value, 3);
    assert_eq!(Keys::Singleton.to_key_bytes().unwrap(), [0, 0]);
    assert_eq!(Keys::Bounded(Bounded { value: None }).encoded_len(), 11);
}
