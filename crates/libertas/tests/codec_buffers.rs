extern crate alloc;

use core::mem::MaybeUninit;
use libertas::data::{KeyCodec, KeyDecode, KeyEncodeError, KeyFieldCodec, KeyWriter};
use libertas::{AvroDecode, AvroEncode, ByteBuffer, InlineByteBuffer, STACK_BUF_SIZE};
use libertas_macros::{
    LibertasAvroDecode, LibertasAvroEncode, LibertasKeyCodec, LibertasKeyDecode,
    LibertasKeyFieldCodec, LibertasKeyFieldDecode,
};
#[path = "support/allocations.rs"]
mod allocation_counter;
use allocation_counter::allocations;

#[derive(Debug, PartialEq, LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
enum Component {
    Empty,
    Number(u64),
}
#[derive(Debug, PartialEq, LibertasKeyCodec, LibertasKeyDecode)]
// The large inline field exercises spill; Box is not a supported key field.
#[allow(clippy::large_enum_variant)]
enum Key {
    Item {
        component: Component,
        optional: Option<u16>,
        #[libertas_size(max = 4)]
        name: String,
    },
    Large([u8; STACK_BUF_SIZE]),
}

#[test]
fn uninitialized_keys_and_fields_have_canonical_padding_and_no_allocations() {
    let key = Key::Item {
        component: Component::Empty,
        optional: None,
        name: "a".into(),
    };
    let expected = [
        0, 0, // root tag
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // nested tag and padding
        0, 0, 0, // None and padding
        b'a', 0, 0, 0, 0, 0, 0, 1, // name, padding and length
    ];
    let mut storage = [MaybeUninit::uninit(); 23];
    let (result, count) = allocations(|| key.encode_key_uninit(&mut storage));
    assert_eq!(count, 0);
    assert_eq!(result.unwrap(), expected);
    assert_eq!(Key::decode_key(&expected).unwrap(), key);

    let mut buffer = InlineByteBuffer::new();
    buffer.push(0xa5);
    let pointer = buffer.as_slice().as_ptr();
    let (result, count) = allocations(|| key.encode_key_to_buffer(&mut buffer));
    result.unwrap();
    assert_eq!(count, 0);
    assert_eq!(buffer.as_slice().as_ptr(), pointer);
    assert_eq!(buffer.as_slice()[0], 0xa5);
    assert_eq!(&buffer.as_slice()[1..], expected);

    let mut number = [MaybeUninit::uninit(); 3];
    assert_eq!(
        Some(256u16).encode_field_uninit(&mut number).unwrap(),
        [1, 1, 0]
    );
    let mut empty = [];
    assert_eq!([0u8; 0].encode_field_uninit(&mut empty).unwrap(), []);
}

#[test]
fn inline_key_encoding_spills_only_when_required_and_preserves_prefix() {
    let key = Key::Large([0x5a; STACK_BUF_SIZE]);
    let mut buffer = InlineByteBuffer::new();
    buffer.push(0xa5);
    key.encode_key_to_buffer(&mut buffer).unwrap();
    assert!(buffer.capacity() > STACK_BUF_SIZE);
    assert_eq!(&buffer.as_slice()[..3], [0xa5, 0, 1]);
    assert_eq!(&buffer.as_slice()[3..], [0x5a; STACK_BUF_SIZE]);
    let pointer = buffer.as_slice().as_ptr();
    buffer.clear();
    let (result, count) = allocations(|| key.encode_key_to_buffer(&mut buffer));
    result.unwrap();
    assert_eq!(count, 0);
    assert_eq!(buffer.as_slice().as_ptr(), pointer);
    assert_eq!(Key::decode_key(buffer.as_slice()).unwrap(), key);
}

#[test]
fn partial_key_failure_keeps_initialized_prefix_and_can_be_retried() {
    let mut buffer = InlineByteBuffer::new();
    buffer.extend_from_slice(&[0xa5, 0x5a]);
    let mut key = Key::Item {
        component: Component::Number(42),
        optional: Some(1),
        name: "too long".into(),
    };
    assert_eq!(
        key.encode_key_to_buffer(&mut buffer),
        Err(KeyEncodeError::CapacityExceeded {
            capacity: 4,
            actual: 8
        })
    );
    assert_eq!(buffer.as_slice(), [0xa5, 0x5a]);
    if let Key::Item { name, .. } = &mut key {
        name.clear();
    }
    key.encode_key_to_buffer(&mut buffer).unwrap();
    assert_eq!(Key::decode_key(&buffer.as_slice()[2..]).unwrap(), key);
}

#[test]
fn wrong_destination_length_leaves_initialized_sentinels_untouched() {
    #[derive(LibertasKeyCodec)]
    enum Fixed {
        Number(u16),
    }
    for size in [0, 3, 5] {
        let mut bytes = [MaybeUninit::new(0xa5); 5];
        assert_eq!(
            Fixed::Number(42).encode_key_uninit(&mut bytes[..size]),
            Err(KeyEncodeError::InvalidBufferLength {
                expected: 4,
                actual: size
            })
        );
        // SAFETY: Every byte began initialized and a length mismatch performs no writes.
        assert!(
            bytes
                .iter()
                .all(|byte| unsafe { byte.assume_init() } == 0xa5)
        );
    }
}

struct Incomplete;
impl KeyFieldCodec for Incomplete {
    const ENCODED_SIZE: usize = 2;
    fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
        output.write(&[1])
    }
}
impl KeyCodec for Incomplete {
    fn encoded_len(&self) -> usize {
        2
    }
    fn write_key(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
        self.write_field(output)
    }
}

#[test]
fn incomplete_custom_codecs_cannot_expose_uninitialized_bytes() {
    let mut storage = [MaybeUninit::uninit(); 2];
    let expected = KeyEncodeError::InvalidBufferLength {
        expected: 2,
        actual: 1,
    };
    assert_eq!(Incomplete.encode_key_uninit(&mut storage), Err(expected));
    assert_eq!(Incomplete.encode_field_uninit(&mut storage), Err(expected));
    assert_eq!(Incomplete.to_key_bytes(), Err(expected));
    let mut buffer = InlineByteBuffer::new();
    buffer.push(0xa5);
    assert_eq!(Incomplete.encode_key_to_buffer(&mut buffer), Err(expected));
    assert_eq!(buffer.as_slice(), [0xa5]);
    #[derive(LibertasKeyCodec)]
    enum Nested {
        Item {
            bad: Option<Incomplete>,
            following: u8,
        },
    }
    assert_eq!(
        Nested::Item {
            bad: Some(Incomplete),
            following: 3
        }
        .to_key_bytes(),
        Err(expected)
    );
}

#[test]
fn oversize_custom_writes_are_errors_and_panics_do_not_publish_partial_bytes() {
    struct Oversize;
    impl KeyCodec for Oversize {
        fn encoded_len(&self) -> usize {
            1
        }
        fn write_key(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
            output.write(&[1, 2])
        }
    }
    assert!(Oversize.to_key_bytes().is_err());
    struct Panics;
    impl KeyCodec for Panics {
        fn encoded_len(&self) -> usize {
            2
        }
        fn write_key(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
            output.write(&[1])?;
            panic!("codec panic");
        }
    }
    let mut buffer = InlineByteBuffer::new();
    buffer.push(0xa5);
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Panics.encode_key_to_buffer(&mut buffer)
        }))
        .is_err()
    );
    assert_eq!(buffer.as_slice(), [0xa5]);
}

#[derive(Debug, PartialEq, LibertasAvroEncode, LibertasAvroDecode)]
struct Payload {
    text: String,
    items: Vec<i64>,
    bytes: Vec<u8>,
    optional: Option<Box<i64>>,
    floats: (f32, f64),
}
#[derive(Debug, PartialEq, LibertasAvroEncode, LibertasAvroDecode)]
enum Value {
    Empty,
    Data(Payload),
    Named { flag: bool, count: i32 },
}

#[test]
fn derived_avro_and_primitives_encode_inline_without_intermediate_allocations() {
    let values = [
        Value::Empty,
        Value::Named {
            flag: true,
            count: -42,
        },
        Value::Data(Payload {
            text: "hello".into(),
            items: vec![0, -1, i64::MIN, i64::MAX],
            bytes: vec![0, 255, 1],
            optional: Some(Box::new(42)),
            floats: (-0.0, 1.5),
        }),
    ];
    for value in values {
        let expected = value.to_avro();
        let mut buffer = InlineByteBuffer::new();
        buffer.push(0xa5);
        let ((), count) = allocations(|| value.avro_encode(&mut buffer));
        assert_eq!(count, 0);
        assert_eq!(buffer.as_slice()[0], 0xa5);
        assert_eq!(&buffer.as_slice()[1..], expected);
        let mut offset = 1;
        assert_eq!(
            Value::avro_decode(buffer.as_slice(), &mut offset).unwrap(),
            value
        );
        assert_eq!(offset, buffer.len());
    }
    // ByteBuffer can also be passed as a trait object; Avro values remain generic.
    let mut buffer = InlineByteBuffer::new();
    let destination: &mut dyn ByteBuffer = &mut buffer;
    (-42i64, true, "a").avro_encode(destination);
    assert_eq!(buffer.as_slice(), [83, 1, 2, b'a']);
}

#[test]
fn avro_spill_preserves_bytes_and_existing_vec_callers() {
    let value = "x".repeat(STACK_BUF_SIZE + 1);
    let mut heap = vec![0xa5];
    value.avro_encode(&mut heap);
    let mut buffer = InlineByteBuffer::new();
    buffer.push(0xa5);
    value.avro_encode(&mut buffer);
    assert!(buffer.capacity() > STACK_BUF_SIZE);
    assert_eq!(buffer.as_slice(), heap);
}
