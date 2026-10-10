use libertas::data::{KeyCodec, KeyEncodeError, KeyFieldCodec};
use libertas_macros::{LibertasKeyCodec, LibertasKeyFieldCodec};

fn field<T: KeyFieldCodec>(value: &T) -> Vec<u8> {
    let mut bytes = vec![0xff; T::ENCODED_SIZE];
    value.encode_field(&mut bytes).unwrap();
    bytes
}

#[test]
fn primitive_bytes_are_canonical_and_independent_of_memory_layout() {
    assert_eq!(field(&false), [0]);
    assert_eq!(field(&true), [1]);
    assert_eq!(field(&0xa5u8), [0xa5]);
    assert_eq!(field(&0x1234u16), [0x12, 0x34]);
    assert_eq!(field(&0x12345678u32), [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(
        field(&0x123456789abcdef0u64),
        [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]
    );
    assert_eq!(
        field(&0x0123456789abcdef_fedcba9876543210u128),
        [
            1, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32,
            0x10
        ]
    );
    assert_eq!(field(&[0, 255, 17]), [0, 255, 17]);
    assert_eq!(field(&[] as &[u8; 0]), []);

    macro_rules! signed_boundaries {
        ($($ty:ty),*) => {$(
            let width = <$ty>::ENCODED_SIZE;
            assert_eq!(field(&<$ty>::MIN), vec![0; width]);
            assert_eq!(field(&<$ty>::MAX), vec![255; width]);
            let mut negative_one = vec![255; width];
            negative_one[0] = 0x7f;
            assert_eq!(field(&(-1 as $ty)), negative_one);
            let mut zero = vec![0; width];
            zero[0] = 0x80;
            assert_eq!(field(&(0 as $ty)), zero);
        )*};
    }
    signed_boundaries!(i8, i16, i32, i64, i128);
}

#[test]
fn integer_byte_order_matches_numeric_order() {
    let signed = (i8::MIN..=i8::MAX)
        .map(|value| field(&value))
        .collect::<Vec<_>>();
    assert!(signed.windows(2).all(|pair| pair[0] < pair[1]));
    macro_rules! ordered {
        ($($ty:ty),*) => {$(
            let mut values = vec![<$ty>::MIN, <$ty>::MAX, 0, 1, 127, 128, 255, 256];
            let mut state = 0x79eeb017fc1195aau128;
            for _ in 0..1024 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                values.push(state as $ty);
            }
            values.sort();
            values.dedup();
            let bytes = values.iter().map(field).collect::<Vec<_>>();
            assert!(bytes.windows(2).all(|pair| pair[0] < pair[1]), stringify!($ty));
        )*};
    }
    ordered!(u16, u32, u64, u128, i16, i32, i64, i128);
}

#[derive(Debug, LibertasKeyFieldCodec)]
enum Subject {
    Missing,
    Device(u64),
    Document { id: [u8; 16] },
}

#[derive(LibertasKeyFieldCodec)]
struct Reading {
    subject: Subject,
    timestamp: i32,
}

#[derive(LibertasKeyCodec)]
enum Key {
    Settings,
    Device { id: u64 },
    Reading(Reading),
    Last,
}

#[test]
fn root_lengths_are_per_variant_and_nested_unions_use_the_largest_payload() {
    assert_eq!(Subject::ENCODED_SIZE, 18);
    assert_eq!(Reading::ENCODED_SIZE, 22);
    assert_eq!(Key::Settings.to_key_bytes().unwrap(), [0, 0]);
    assert_eq!(Key::Last.to_key_bytes().unwrap(), [0, 3]);
    assert_eq!(
        Key::Device { id: 42 }.to_key_bytes().unwrap(),
        [0, 1, 0, 0, 0, 0, 0, 0, 0, 42]
    );

    let key = Key::Reading(Reading {
        subject: Subject::Device(42),
        timestamp: -1,
    });
    assert_eq!(key.encoded_len(), 24);
    assert_eq!(
        key.to_key_bytes().unwrap(),
        [
            0, 2, // root tag
            0, 1, // field tag
            0, 0, 0, 0, 0, 0, 0, 42, // selected payload
            0, 0, 0, 0, 0, 0, 0, 0, // field padding
            0x7f, 0xff, 0xff, 0xff, // next field stays at the fixed offset
        ]
    );
    assert_eq!(field(&Subject::Missing), [0; 18]);
    let mut document = vec![0, 2];
    document.extend_from_slice(&[0xa5; 16]);
    assert_eq!(field(&Subject::Document { id: [0xa5; 16] }), document);
}

#[test]
fn appending_a_larger_root_variant_preserves_existing_bytes() {
    #[derive(LibertasKeyCodec)]
    enum Before {
        Setting,
        Device(u64),
    }
    #[derive(LibertasKeyCodec)]
    enum After {
        Setting,
        Device(u64),
        Large([u8; 128]),
    }
    assert_eq!(
        Before::Setting.to_key_bytes(),
        After::Setting.to_key_bytes()
    );
    assert_eq!(
        Before::Device(42).to_key_bytes(),
        After::Device(42).to_key_bytes()
    );
    assert_eq!(After::Large([0; 128]).encoded_len(), 130);
}

#[derive(LibertasKeyFieldCodec)]
enum Nested {
    Empty,
    Subject(Subject),
}

#[test]
fn nested_padding_is_recursive_and_overwrites_existing_bytes() {
    assert_eq!(Nested::ENCODED_SIZE, 20);
    assert_eq!(field(&Nested::Empty), [0; 20]);
    let mut expected = vec![0; 20];
    expected[1] = 1;
    expected[3] = 1;
    expected[11] = 42;
    assert_eq!(field(&Nested::Subject(Subject::Device(42))), expected);
}

#[test]
fn compound_key_order_follows_tags_then_fields() {
    let mut keys = vec![Key::Settings.to_key_bytes().unwrap()];
    for id in [0, 255, 256, u64::MAX] {
        keys.push(Key::Device { id }.to_key_bytes().unwrap());
    }
    for id in [0, 255, 256, u64::MAX] {
        for timestamp in [i32::MIN, -1, 0, 1, i32::MAX] {
            keys.push(
                Key::Reading(Reading {
                    subject: Subject::Device(id),
                    timestamp,
                })
                .to_key_bytes()
                .unwrap(),
            );
        }
    }
    keys.push(Key::Last.to_key_bytes().unwrap());
    assert!(keys.windows(2).all(|pair| pair[0] < pair[1]));
}

type Text = String;
type Bytes = Vec<u8>;
const CAPACITY: usize = 4;

#[derive(LibertasKeyFieldCodec)]
struct Name {
    #[libertas_size(max = CAPACITY)]
    value: Text,
}

#[derive(LibertasKeyFieldCodec)]
struct Binary {
    #[libertas_size(max = 2 + 2)]
    value: Bytes,
}

#[derive(LibertasKeyCodec)]
enum BoundedKey {
    Named {
        #[libertas_size(max = 4)]
        value: String,
    },
    Bytes(#[libertas_size(max = 4)] Vec<u8>),
}

#[test]
fn bounded_fields_preserve_length_nuls_unicode_and_declared_width() {
    assert_eq!(Name::ENCODED_SIZE, 8);
    assert_eq!(Binary::ENCODED_SIZE, 8);
    assert_eq!(
        field(&Name { value: "a".into() }),
        [b'a', 0, 0, 0, 0, 0, 0, 1]
    );
    assert_eq!(
        field(&Name {
            value: "a\0".into()
        }),
        [b'a', 0, 0, 0, 0, 0, 0, 2]
    );
    assert_eq!(
        field(&Name { value: "é".into() }),
        [0xc3, 0xa9, 0, 0, 0, 0, 0, 2]
    );
    assert_eq!(
        field(&Binary {
            value: vec![0, 255, 0, 17]
        }),
        [0, 255, 0, 17, 0, 0, 0, 4]
    );
    assert_eq!(field(&Binary { value: vec![] }), [0; 8]);
    assert_eq!(
        BoundedKey::Named { value: "é".into() }
            .to_key_bytes()
            .unwrap(),
        [0, 0, 0xc3, 0xa9, 0, 0, 0, 0, 0, 2]
    );
    assert_eq!(
        BoundedKey::Bytes(vec![0, 255]).to_key_bytes().unwrap(),
        [0, 1, 0, 255, 0, 0, 0, 0, 0, 2]
    );
}

#[test]
fn bounded_byte_order_is_content_order_even_with_zero_suffixes() {
    let mut values = vec![vec![]];
    let mut level = vec![vec![]];
    for _ in 0..4 {
        level = level
            .iter()
            .flat_map(|prefix| {
                [0, 1, 127, 255].map(|byte| {
                    let mut value = prefix.clone();
                    value.push(byte);
                    value
                })
            })
            .collect();
        values.extend(level.iter().cloned());
    }
    values.sort();
    let keys = values
        .into_iter()
        .map(|value| field(&Binary { value }))
        .collect::<Vec<_>>();
    assert!(keys.windows(2).all(|pair| pair[0] < pair[1]));
    let mut strings = [
        "", "\0", "\0\0", "a", "a\0", "a\0\0", "a\0b", "ab", "é", "中", "🦀",
    ];
    strings.sort();
    let keys = strings
        .into_iter()
        .map(|value| {
            field(&Name {
                value: value.into(),
            })
        })
        .collect::<Vec<_>>();
    assert!(keys.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn rejects_capacity_overflow_without_truncation() {
    let mut output = [0xa5; 8];
    assert_eq!(
        Name {
            value: "ééé".into()
        }
        .encode_field(&mut output),
        Err(KeyEncodeError::CapacityExceeded {
            capacity: 4,
            actual: 6
        })
    );
    assert_eq!(output, [0xa5; 8]);
    assert_eq!(
        BoundedKey::Bytes(vec![0; 5]).to_key_bytes(),
        Err(KeyEncodeError::CapacityExceeded {
            capacity: 4,
            actual: 5
        })
    );
}

#[test]
fn rejects_short_and_oversized_buffers_before_writing() {
    for size in [0, 1, 3] {
        let mut output = vec![0xa5; size];
        assert_eq!(
            42u16.encode_field(&mut output),
            Err(KeyEncodeError::InvalidBufferLength {
                expected: 2,
                actual: size
            })
        );
        assert_eq!(output, vec![0xa5; size]);
    }
    let key = Key::Reading(Reading {
        subject: Subject::Missing,
        timestamp: 0,
    });
    for size in [0, 2, 23, 25] {
        let mut output = vec![0xa5; size];
        assert_eq!(
            key.encode_key(&mut output),
            Err(KeyEncodeError::InvalidBufferLength {
                expected: 24,
                actual: size
            })
        );
        assert_eq!(output, vec![0xa5; size]);
    }
    assert_eq!(
        Subject::Missing.encode_field(&mut [0; 19]),
        Err(KeyEncodeError::InvalidBufferLength {
            expected: 18,
            actual: 19
        })
    );
}

#[derive(LibertasKeyFieldCodec)]
struct Generic<T: Clone, const N: usize>
where
    T: PartialEq,
{
    first: T,
    raw: [u8; N],
    #[libertas_size(max = N)]
    name: String,
}

#[derive(LibertasKeyCodec)]
enum GenericRoot<T: Clone>
where
    T: PartialEq,
{
    Item(T),
}

#[derive(LibertasKeyFieldCodec)]
struct Empty {}

#[test]
fn generic_types_constants_aliases_and_empty_payloads_compose() {
    let value = Generic::<u16, 2> {
        first: 256,
        raw: [1, 2],
        name: "a".into(),
    };
    assert_eq!(Generic::<u16, 2>::ENCODED_SIZE, 10);
    assert_eq!(field(&value), [1, 0, 1, 2, b'a', 0, 0, 0, 0, 1]);
    assert_eq!(
        GenericRoot::Item(256u16).to_key_bytes().unwrap(),
        [0, 0, 1, 0]
    );
    assert_eq!(field(&Empty {}), []);
    assert_eq!(<libertas::LibertasDateTime>::ENCODED_SIZE, 8);
    assert_eq!(<libertas::LibertasTimeOnly>::ENCODED_SIZE, 4);
}

#[derive(LibertasKeyFieldCodec)]
struct ZeroCapacity {
    #[libertas_size(max = 0)]
    value: Vec<u8>,
}

#[test]
fn zero_capacity_has_a_length_suffix_and_accepts_only_empty_values() {
    assert_eq!(field(&ZeroCapacity { value: vec![] }), [0; 4]);
    assert_eq!(
        ZeroCapacity { value: vec![0] }.encode_field(&mut [0; 4]),
        Err(KeyEncodeError::CapacityExceeded {
            capacity: 0,
            actual: 1
        })
    );
}
