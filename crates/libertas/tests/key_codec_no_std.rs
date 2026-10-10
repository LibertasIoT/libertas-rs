#![no_std]

extern crate alloc;

use alloc::{string::String, vec};
use libertas::data::{KeyCodec, KeyDecode, KeyFieldCodec};
use libertas_macros::{
    LibertasKeyCodec, LibertasKeyDecode, LibertasKeyFieldCodec, LibertasKeyFieldDecode,
};

#[derive(LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Component {
    #[libertas_size(max = 2)]
    text: String,
}

#[derive(LibertasKeyCodec, LibertasKeyDecode)]
enum Key {
    Named(Component),
}

#[test]
fn generated_codecs_work_without_the_std_prelude() {
    assert_eq!(Component::ENCODED_SIZE, 6);
    let key = Key::Named(Component {
        text: String::from("a"),
    });
    assert_eq!(key.to_key_bytes().unwrap(), vec![0, 0, b'a', 0, 0, 0, 0, 1]);
    let decoded = Key::decode_key(&key.to_key_bytes().unwrap()).unwrap();
    assert_eq!(decoded.to_key_bytes().unwrap(), key.to_key_bytes().unwrap());
}

#[allow(non_camel_case_types, unused_imports, dead_code)]
mod shadowed_names {
    use core::option::Option::Some as Ok;
    type u8 = bool;
    type u16 = bool;
    type usize = bool;

    #[derive(libertas_macros::LibertasKeyFieldCodec, libertas_macros::LibertasKeyFieldDecode)]
    pub struct Field {
        pub value: ::core::primitive::u16,
    }

    #[derive(libertas_macros::LibertasKeyCodec, libertas_macros::LibertasKeyDecode)]
    pub enum Key {
        Item(Field),
    }
}

#[test]
fn generated_names_do_not_depend_on_caller_imports() {
    let value = shadowed_names::Key::Item(shadowed_names::Field { value: 256 });
    assert_eq!(value.to_key_bytes().unwrap(), [0, 0, 1, 0]);
    let decoded = shadowed_names::Key::decode_key(&[0, 0, 1, 0]).unwrap();
    assert_eq!(decoded.to_key_bytes().unwrap(), [0, 0, 1, 0]);
}

#[test]
fn optional_fields_and_bounded_aliases_work_without_std() {
    type MaybeText = core::option::Option<String>;
    #[derive(LibertasKeyCodec, LibertasKeyDecode)]
    enum Optional {
        Value {
            number: Option<u16>,
            #[libertas_size(max = 2)]
            text: MaybeText,
        },
    }
    let value = Optional::Value {
        number: Some(256),
        text: None,
    };
    let encoded = value.to_key_bytes().unwrap();
    assert_eq!(encoded, [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        Optional::decode_key(&encoded)
            .unwrap()
            .to_key_bytes()
            .unwrap(),
        encoded
    );
}

#[test]
fn inline_key_and_avro_encoding_work_without_std() {
    use libertas::{AvroEncode, InlineByteBuffer};
    #[derive(libertas_macros::LibertasAvroEncode)]
    struct Value {
        number: i64,
        text: String,
    }
    let mut buffer = InlineByteBuffer::new();
    Value {
        number: -1,
        text: String::from("a"),
    }
    .avro_encode(&mut buffer);
    assert_eq!(buffer.as_slice(), [1, 2, b'a']);
    buffer.clear();
    Key::Named(Component {
        text: String::from("a"),
    })
    .encode_key_to_buffer(&mut buffer)
    .unwrap();
    assert_eq!(buffer.as_slice(), [0, 0, b'a', 0, 0, 0, 0, 1]);
}
