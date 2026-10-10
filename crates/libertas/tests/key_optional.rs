use libertas::data::{
    KeyCodec, KeyDecode, KeyDecodeError, KeyEncodeError, KeyFieldCodec, KeyFieldDecode,
};
use libertas_macros::{
    LibertasKeyCodec, LibertasKeyDecode, LibertasKeyFieldCodec, LibertasKeyFieldDecode,
};

fn bytes<T: KeyFieldCodec>(value: &T) -> Vec<u8> {
    let mut output = vec![0xff; T::ENCODED_SIZE];
    value.encode_field(&mut output).unwrap();
    output
}

#[test]
fn optional_golden_bytes_preserve_absence_and_zero() {
    assert_eq!(<Option<u16>>::ENCODED_SIZE, 3);
    for (value, expected) in [
        (None, [0, 0, 0]),
        (Some(0), [1, 0, 0]),
        (Some(256), [1, 1, 0]),
        (Some(u16::MAX), [1, 255, 255]),
    ] {
        assert_eq!(bytes(&value), expected);
        assert_eq!(Option::<u16>::decode_field(&expected).unwrap(), value);
    }
    assert_eq!(bytes(&None::<bool>), [0, 0]);
    assert_eq!(bytes(&Some(false)), [1, 0]);
    assert_eq!(bytes(&Some(true)), [1, 1]);
    assert_eq!(bytes(&None::<[u8; 0]>), [0]);
    assert_eq!(bytes(&Some([] as [u8; 0])), [1]);
    assert_eq!(Option::<[u8; 0]>::decode_field(&[0]).unwrap(), None);
    assert_eq!(Option::<[u8; 0]>::decode_field(&[1]).unwrap(), Some([]));
}

#[test]
fn optional_order_is_none_first_then_the_underlying_numeric_order() {
    let values = core::iter::once(None)
        .chain((i8::MIN..=i8::MAX).map(Some))
        .collect::<Vec<_>>();
    let encoded = values.iter().map(bytes).collect::<Vec<_>>();
    assert!(encoded.windows(2).all(|pair| pair[0] < pair[1]));
    for (value, encoded) in values.iter().zip(&encoded) {
        assert_eq!(&Option::<i8>::decode_field(encoded).unwrap(), value);
    }
    let mut values = [
        f64::from_bits(0xfff8000000000042),
        f64::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        1.0,
        f64::INFINITY,
        f64::from_bits(0x7ff0000000000042),
        f64::from_bits(0x7ff8000000000042),
    ];
    values.sort_by(f64::total_cmp);
    let mut encoded = vec![bytes(&None::<f64>)];
    for value in values {
        let field = bytes(&Some(value));
        assert_eq!(
            Option::<f64>::decode_field(&field)
                .unwrap()
                .unwrap()
                .to_bits(),
            value.to_bits()
        );
        encoded.push(field);
    }
    assert!(encoded.windows(2).all(|pair| pair[0] < pair[1]));
}

#[derive(Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
enum Choice {
    Empty,
    Flag(bool),
    Wide(u64),
}

#[derive(Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Record<T> {
    before: u8,
    value: Option<T>,
    after: i16,
}

#[derive(Debug, PartialEq, LibertasKeyCodec, LibertasKeyDecode)]
enum Key {
    Empty,
    Record(Record<Choice>),
}

#[test]
fn optional_struct_and_union_payloads_keep_fixed_offsets_and_u16_union_tags() {
    assert_eq!(Choice::ENCODED_SIZE, 10);
    assert_eq!(Record::<Choice>::ENCODED_SIZE, 14);
    for value in [
        None,
        Some(Choice::Empty),
        Some(Choice::Flag(true)),
        Some(Choice::Wide(u64::MAX)),
    ] {
        let key = Key::Record(Record {
            before: 42,
            value,
            after: -1,
        });
        let encoded = key.to_key_bytes().unwrap();
        assert_eq!(encoded.len(), 16);
        assert_eq!(&encoded[..3], &[0, 1, 42]);
        assert_eq!(&encoded[14..], &[0x7f, 0xff]);
        assert_eq!(Key::decode_key(&encoded).unwrap(), key);
    }
    assert_eq!(
        bytes(&Some(Choice::Flag(true))),
        [1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(Key::Empty.to_key_bytes().unwrap(), [0, 0]);
}

#[derive(Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Bounded<T, const N: usize> {
    #[libertas_size(max = N)]
    value: T,
}

type MaybeText = Option<String>;
type Text = Bounded<MaybeText, 4>;
type Binary = Bounded<Option<Vec<u8>>, 4>;

#[test]
fn bounded_optional_aliases_distinguish_none_empty_nuls_and_utf8() {
    assert_eq!(Text::ENCODED_SIZE, 9);
    assert_eq!(Binary::ENCODED_SIZE, 9);
    let mut previous = Vec::new();
    for value in [
        None,
        Some(""),
        Some("\0"),
        Some("\0\0"),
        Some("a"),
        Some("a\0"),
        Some("é"),
        Some("🦀"),
    ] {
        let text = Text {
            value: value.map(str::to_owned),
        };
        let encoded = bytes(&text);
        assert_eq!(encoded.len(), 9);
        assert!(previous < encoded);
        assert_eq!(Text::decode_field(&encoded).unwrap(), text);
        previous = encoded;
    }
    assert_eq!(bytes(&Text { value: None }), [0; 9]);
    assert_eq!(
        bytes(&Text {
            value: Some("a\0".into())
        }),
        [1, b'a', 0, 0, 0, 0, 0, 0, 2]
    );
    let binary = Binary {
        value: Some(vec![0xff, 0]),
    };
    assert_eq!(bytes(&binary), [1, 255, 0, 0, 0, 0, 0, 0, 2]);
    assert_eq!(Binary::decode_field(&bytes(&binary)).unwrap(), binary);

    type Empty = Bounded<MaybeText, 0>;
    assert_eq!(Empty::ENCODED_SIZE, 5);
    assert_eq!(bytes(&Empty { value: None }), [0; 5]);
    assert_eq!(
        bytes(&Empty {
            value: Some(String::new())
        }),
        [1, 0, 0, 0, 0]
    );
}

#[derive(Debug, PartialEq, LibertasKeyCodec, LibertasKeyDecode)]
enum BoundedKey {
    Text(#[libertas_size(max = 4)] MaybeText),
    Binary {
        #[libertas_size(max = 4)]
        value: Option<Vec<u8>>,
    },
}

#[test]
fn bounded_optional_root_variants_check_capacity_and_exact_buffers() {
    for key in [
        BoundedKey::Text(None),
        BoundedKey::Text(Some("éé".into())),
        BoundedKey::Binary {
            value: Some(vec![1, 2, 3, 4]),
        },
    ] {
        let encoded = key.to_key_bytes().unwrap();
        assert_eq!(encoded.len(), 11);
        assert_eq!(BoundedKey::decode_key(&encoded).unwrap(), key);
        for length in (0..encoded.len()).chain([encoded.len() + 1]) {
            let mut input = encoded.clone();
            input.resize(length, 0);
            assert!(matches!(
                BoundedKey::decode_key(&input),
                Err(KeyDecodeError::InvalidBufferLength { .. })
            ));
            assert!(matches!(
                key.encode_key(&mut input),
                Err(KeyEncodeError::InvalidBufferLength { .. })
            ));
        }
    }
    assert_eq!(
        BoundedKey::Text(Some("ééé".into())).to_key_bytes(),
        Err(KeyEncodeError::CapacityExceeded {
            capacity: 4,
            actual: 6
        })
    );
    assert_eq!(
        BoundedKey::Binary {
            value: Some(vec![0; 5])
        }
        .to_key_bytes(),
        Err(KeyEncodeError::CapacityExceeded {
            capacity: 4,
            actual: 5
        })
    );
    for length in [0, 1, 2, 4] {
        assert_eq!(
            None::<u16>.encode_field(&mut vec![0; length]),
            Err(KeyEncodeError::InvalidBufferLength {
                expected: 3,
                actual: length
            })
        );
        assert_eq!(
            Option::<u16>::decode_field(&vec![0; length]),
            Err(KeyDecodeError::InvalidBufferLength {
                expected: 3,
                actual: length
            })
        );
    }
}

#[test]
fn optional_decoders_reject_invalid_presence_padding_and_present_payloads() {
    for value in 2..=255 {
        assert_eq!(
            Option::<u16>::decode_field(&[value, 0, 0]),
            Err(KeyDecodeError::InvalidPresence { value })
        );
        let mut input = [0; 9];
        input[0] = value;
        assert_eq!(
            Text::decode_field(&input),
            Err(KeyDecodeError::InvalidPresence { value })
        );
    }
    for offset in 1..9 {
        let mut input = [0; 9];
        input[offset] = 1;
        assert_eq!(
            Text::decode_field(&input),
            Err(KeyDecodeError::InvalidPadding)
        );
        assert_eq!(
            Option::<u64>::decode_field(&input),
            Err(KeyDecodeError::InvalidPadding)
        );
    }
    assert_eq!(
        Option::<bool>::decode_field(&[1, 2]),
        Err(KeyDecodeError::InvalidBoolean { value: 2 })
    );
    let mut input = [1, 0xff, 0, 0, 0, 0, 0, 0, 1];
    assert_eq!(Text::decode_field(&input), Err(KeyDecodeError::InvalidUtf8));
    input[5..].copy_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(
        Text::decode_field(&input),
        Err(KeyDecodeError::InvalidLength {
            capacity: 4,
            actual: u32::MAX
        })
    );
    let mut input = bytes(&Some(Choice::Empty));
    input[2] = 3;
    assert_eq!(
        Option::<Choice>::decode_field(&input),
        Err(KeyDecodeError::UnknownDiscriminator { discriminator: 3 })
    );
}

#[test]
fn none_never_calls_the_payload_codec_and_encoding_does_not_require_decoding() {
    struct EncodeOnly;
    impl KeyFieldCodec for EncodeOnly {
        const ENCODED_SIZE: usize = 2;
        fn write_field(&self, _: &mut libertas::data::KeyWriter<'_>) -> Result<(), KeyEncodeError> {
            panic!("None must not call the payload encoder");
        }
    }
    impl KeyFieldDecode for EncodeOnly {
        fn decode_field(_: &[u8]) -> Result<Self, KeyDecodeError> {
            panic!("None must not call the payload decoder");
        }
    }
    assert_eq!(bytes(&None::<EncodeOnly>), [0, 0, 0]);
    assert!(
        Option::<EncodeOnly>::decode_field(&[0, 0, 0])
            .unwrap()
            .is_none()
    );

    struct NoDecoder(u16);
    impl KeyFieldCodec for NoDecoder {
        const ENCODED_SIZE: usize = 2;
        fn write_field(
            &self,
            output: &mut libertas::data::KeyWriter<'_>,
        ) -> Result<(), KeyEncodeError> {
            self.0.write_field(output)
        }
    }
    #[derive(LibertasKeyCodec)]
    enum Encoder {
        Value(Option<NoDecoder>),
    }
    assert_eq!(
        Encoder::Value(Some(NoDecoder(256))).to_key_bytes().unwrap(),
        [0, 0, 1, 1, 0]
    );
}

#[test]
fn arbitrary_optional_inputs_either_fail_or_reencode_canonically() {
    let mut state = 0x5432abcdu32;
    for length in 0..20 {
        for _ in 0..256 {
            let mut input = vec![0; length];
            for value in &mut input {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                *value = state as u8;
            }
            for tag in [0, 1, 2, 255] {
                if let Some(first) = input.first_mut() {
                    *first = tag;
                }
                if let Ok(value) = Option::<Choice>::decode_field(&input) {
                    assert_eq!(bytes(&value), input);
                }
                if let Ok(value) = Text::decode_field(&input) {
                    assert_eq!(bytes(&value), input);
                }
            }
        }
    }
}
