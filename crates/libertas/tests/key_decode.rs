use libertas::data::{KeyCodec, KeyDecode, KeyDecodeError, KeyFieldCodec, KeyFieldDecode};
use libertas_macros::{
    LibertasKeyCodec, LibertasKeyDecode, LibertasKeyFieldCodec, LibertasKeyFieldDecode,
};

fn field<T: KeyFieldCodec>(value: &T) -> Vec<u8> {
    let mut bytes = vec![0xff; T::ENCODED_SIZE];
    value.encode_field(&mut bytes).unwrap();
    bytes
}

#[test]
fn primitive_decoders_recover_all_integer_widths_and_reject_wrong_lengths() {
    macro_rules! round_trip {
        ($($ty:ty),*) => {$(
            for value in [<$ty>::MIN, <$ty>::MAX, 0, 1, 127] {
                assert_eq!(<$ty>::decode_field(&field(&value)).unwrap(), value);
            }
            let width = <$ty>::ENCODED_SIZE;
            for size in [width - 1, width + 1] {
                assert_eq!(<$ty>::decode_field(&vec![0; size]), Err(KeyDecodeError::InvalidBufferLength { expected: width, actual: size }));
            }
        )*};
    }
    round_trip!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
    for value in i8::MIN..=i8::MAX {
        assert_eq!(i8::decode_field(&field(&value)).unwrap(), value);
    }
    for value in [false, true] {
        assert_eq!(bool::decode_field(&field(&value)).unwrap(), value);
    }
    for value in 2..=255 {
        assert_eq!(
            bool::decode_field(&[value]),
            Err(KeyDecodeError::InvalidBoolean { value })
        );
    }
    assert_eq!(
        <[u8; 4]>::decode_field(&[0, 255, 0, 1]).unwrap(),
        [0, 255, 0, 1]
    );
    assert_eq!(<[u8; 0]>::decode_field(&[]).unwrap(), []);
    assert!(<[u8; 0]>::decode_field(&[0]).is_err());
}

#[derive(Debug, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
enum Value {
    Empty,
    Flag(bool),
    Number(f64),
    Text {
        #[libertas_size(max = 8)]
        value: String,
    },
    Raw([u8; 16]),
}

#[derive(Debug, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Record {
    value: Value,
    sequence: i32,
}

#[derive(Debug, LibertasKeyCodec, LibertasKeyDecode)]
enum Key {
    Idle,
    Row { record: Record, timestamp: u64 },
    Binary(#[libertas_size(max = 8)] Vec<u8>),
}

fn keys() -> Vec<Key> {
    let mut keys = vec![
        Key::Idle,
        Key::Binary(vec![]),
        Key::Binary(vec![0, 255, 0]),
        Key::Binary(vec![255; 8]),
    ];
    for value in [
        Value::Empty,
        Value::Flag(false),
        Value::Flag(true),
        Value::Number(-0.0),
        Value::Number(f64::from_bits(0xfff8000000001234)),
        Value::Text {
            value: "é\0🦀".into(),
        },
        Value::Raw([0xa5; 16]),
    ] {
        keys.push(Key::Row {
            record: Record {
                value,
                sequence: -1,
            },
            timestamp: u64::MAX,
        });
    }
    keys
}

#[test]
fn derived_root_and_nested_field_decoders_round_trip_every_variant_shape() {
    for original in keys() {
        let bytes = original.to_key_bytes().unwrap();
        let decoded = Key::decode_key(&bytes).unwrap();
        assert_eq!(decoded.to_key_bytes().unwrap(), bytes);
    }
    let bytes = field(&Value::Text {
        value: "é\0🦀".into(),
    });
    match Value::decode_field(&bytes).unwrap() {
        Value::Text { value } => assert_eq!(value, "é\0🦀"),
        _ => panic!("wrong decoded variant"),
    }
}

#[test]
fn decoders_reject_all_truncations_and_trailing_bytes() {
    for value in keys() {
        let bytes = value.to_key_bytes().unwrap();
        for length in 0..bytes.len() {
            assert!(matches!(
                Key::decode_key(&bytes[..length]),
                Err(KeyDecodeError::InvalidBufferLength { .. })
            ));
        }
        let mut extra = bytes;
        extra.push(0);
        assert!(matches!(
            Key::decode_key(&extra),
            Err(KeyDecodeError::InvalidBufferLength { .. })
        ));
    }
    let bytes = field(&Value::Empty);
    for length in 0..bytes.len() {
        assert!(matches!(
            Value::decode_field(&bytes[..length]),
            Err(KeyDecodeError::InvalidBufferLength { .. })
        ));
    }
    let mut extra = bytes;
    extra.push(0);
    assert!(matches!(
        Value::decode_field(&extra),
        Err(KeyDecodeError::InvalidBufferLength { .. })
    ));
}

#[test]
fn decoders_reject_unknown_root_and_nested_tags() {
    assert!(matches!(
        Key::decode_key(&[0, 3]),
        Err(KeyDecodeError::UnknownDiscriminator { discriminator: 3 })
    ));
    let mut nested = field(&Value::Empty);
    nested[..2].copy_from_slice(&[0, 5]);
    assert!(matches!(
        Value::decode_field(&nested),
        Err(KeyDecodeError::UnknownDiscriminator { discriminator: 5 })
    ));
    let mut root = Key::Row {
        record: Record {
            value: Value::Empty,
            sequence: 0,
        },
        timestamp: 0,
    }
    .to_key_bytes()
    .unwrap();
    root[2..4].copy_from_slice(&[0, 5]);
    assert!(matches!(
        Key::decode_key(&root),
        Err(KeyDecodeError::UnknownDiscriminator { discriminator: 5 })
    ));
}

#[test]
fn decoders_reject_noncanonical_padding_at_each_layer() {
    let original = field(&Value::Flag(true));
    for offset in 3..original.len() {
        let mut invalid = original.clone();
        invalid[offset] = 1;
        assert!(matches!(
            Value::decode_field(&invalid),
            Err(KeyDecodeError::InvalidPadding)
        ));
    }
    let original = field(&Value::Text { value: "a".into() });
    // The bounded text has its own padding, then the union adds outer padding.
    for offset in (3..10).chain(14..18) {
        let mut invalid = original.clone();
        invalid[offset] = 1;
        assert!(matches!(
            Value::decode_field(&invalid),
            Err(KeyDecodeError::InvalidPadding)
        ));
    }
    let mut invalid = field(&Value::Flag(false));
    invalid[2] = 2;
    assert!(matches!(
        Value::decode_field(&invalid),
        Err(KeyDecodeError::InvalidBoolean { value: 2 })
    ));
}

#[test]
fn bounded_decoders_validate_length_suffixes_and_utf8() {
    let mut invalid = Key::Binary(vec![]).to_key_bytes().unwrap();
    for length in [9u32, u32::MAX] {
        invalid[10..14].copy_from_slice(&length.to_be_bytes());
        assert!(
            matches!(Key::decode_key(&invalid), Err(KeyDecodeError::InvalidLength { capacity: 8, actual }) if actual == length)
        );
    }
    let mut invalid = field(&Value::Text { value: "a".into() });
    invalid[2] = 0xff;
    assert!(matches!(
        Value::decode_field(&invalid),
        Err(KeyDecodeError::InvalidUtf8)
    ));
    // The same byte is valid in a bounded byte vector.
    let bytes = Key::Binary(vec![0xff]).to_key_bytes().unwrap();
    assert!(matches!(Key::decode_key(&bytes).unwrap(), Key::Binary(value) if value == [0xff]));
}

type Text = String;
#[derive(Clone, Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Generic<T: Clone, const N: usize>
where
    T: PartialEq,
{
    value: T,
    bytes: [u8; N],
    #[libertas_size(max = N)]
    text: Text,
}

#[derive(Debug, PartialEq, LibertasKeyCodec, LibertasKeyDecode)]
enum GenericRoot<T: Clone>
where
    T: PartialEq,
{
    Value(T),
}

#[derive(Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Empty {}

#[test]
fn generics_aliases_and_zero_width_fields_decode() {
    let key = GenericRoot::Value(Generic::<u16, 2> {
        value: 256,
        bytes: [0, 255],
        text: "é".into(),
    });
    let bytes = key.to_key_bytes().unwrap();
    assert_eq!(
        GenericRoot::<Generic<u16, 2>>::decode_key(&bytes).unwrap(),
        key
    );
    assert_eq!(Empty::decode_field(&[]).unwrap(), Empty {});
    assert!(Empty::decode_field(&[0]).is_err());
    let zero = Generic::<bool, 0> {
        value: true,
        bytes: [],
        text: String::new(),
    };
    assert_eq!(
        Generic::<bool, 0>::decode_field(&field(&zero)).unwrap(),
        zero
    );
}

#[test]
fn float_golden_bytes_cover_zeros_infinities_and_nan_signs() {
    assert_eq!(field(&f32::NEG_INFINITY), [0, 0x7f, 0xff, 0xff]);
    assert_eq!(field(&-1.0f32), [0x40, 0x7f, 0xff, 0xff]);
    assert_eq!(field(&-0.0f32), [0x7f, 0xff, 0xff, 0xff]);
    assert_eq!(field(&0.0f32), [0x80, 0, 0, 0]);
    assert_eq!(field(&1.0f32), [0xbf, 0x80, 0, 0]);
    assert_eq!(field(&f32::INFINITY), [0xff, 0x80, 0, 0]);
    assert_eq!(field(&f32::from_bits(0xffc00001)), [0, 0x3f, 0xff, 0xfe]);
    assert_eq!(field(&f32::from_bits(0x7fc00001)), [0xff, 0xc0, 0, 1]);
    assert_eq!(field(&-0.0f64), [0x7f, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(field(&0.0f64), [0x80, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(field(&1.0f64), [0xbf, 0xf0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn float_order_matches_total_cmp_and_decoding_preserves_every_sampled_bit() {
    macro_rules! check {
        ($float:ty, $word:ty, $special:expr) => {{
            let mut bits: Vec<$word> = $special.to_vec();
            let mut state = 0x1234abcd5876ff01u64;
            for _ in 0..20_000 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                bits.push(state as $word);
            }
            let mut values = bits
                .into_iter()
                .map(<$float>::from_bits)
                .collect::<Vec<_>>();
            values.sort_by(<$float>::total_cmp);
            let mut previous: Option<Vec<u8>> = None;
            for value in values {
                let encoded = field(&value);
                assert_eq!(
                    <$float>::decode_field(&encoded).unwrap().to_bits(),
                    value.to_bits()
                );
                if let Some(previous) = previous {
                    assert!(previous <= encoded);
                }
                previous = Some(encoded);
            }
        }};
    }
    check!(
        f32,
        u32,
        [
            0, 0x80000000, 1, 0x80000001, 0x7f800000, 0xff800000, 0x7f800001, 0xff800001,
            0x7fc00000, 0xffc00000, 0x7fffffff, 0xffffffff
        ]
    );
    check!(
        f64,
        u64,
        [
            0,
            0x8000000000000000,
            1,
            0x8000000000000001,
            0x7ff0000000000000,
            0xfff0000000000000,
            0x7ff0000000000001,
            0xfff0000000000001,
            0x7ff8000000000000,
            0xfff8000000000000,
            0x7fffffffffffffff,
            0xffffffffffffffff
        ]
    );
    assert!(f32::decode_field(&[0; 3]).is_err());
    assert!(f64::decode_field(&[0; 9]).is_err());
}

#[test]
fn arbitrary_input_never_panics_and_success_always_reencodes_identically() {
    let mut state = 0x87651234abcdef01u64;
    for length in 0..64 {
        for _ in 0..100 {
            let mut bytes = vec![0; length];
            for byte in &mut bytes {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                *byte = state as u8;
            }
            if length >= 2 {
                for tag in [0u16, 1, 2, 65535] {
                    bytes[..2].copy_from_slice(&tag.to_be_bytes());
                    if let Ok(key) = Key::decode_key(&bytes) {
                        assert_eq!(key.to_key_bytes().unwrap(), bytes);
                    }
                }
            } else {
                assert!(Key::decode_key(&bytes).is_err());
            }
            if let Ok(value) = Value::decode_field(&bytes) {
                assert_eq!(field(&value), bytes);
            }
        }
    }
}
