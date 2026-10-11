#![no_std]

extern crate alloc;

use alloc::string::String;
use libertas::data::{KeyCodec, KeyDecode};
use libertas_macros::{
    LibertasKeyCodec, LibertasKeyDecode, LibertasKeyFieldCodec, LibertasKeyFieldDecode,
};

#[derive(LibertasKeyFieldCodec, LibertasKeyFieldDecode)]
struct Account {
    #[libertas_size(max = 4)]
    #[libertas_pattern(r"[0-9]{4}")]
    number: String,
}

#[derive(LibertasKeyCodec, LibertasKeyDecode)]
enum Key {
    Account(Account),
    Direct {
        #[libertas_size(max = 4)]
        #[libertas_pattern(r"[0-9]{4}")]
        number: String,
    },
}

#[test]
fn pattern_helpers_preserve_native_key_bytes_and_decoding_without_std() {
    let key = Key::Account(Account {
        number: String::from("0009"),
    });
    let bytes = key.to_key_bytes().unwrap();
    assert_eq!(bytes, [0, 0, b'0', b'0', b'0', b'9', 0, 0, 0, 4]);
    assert_eq!(
        Key::decode_key(&bytes).unwrap().to_key_bytes().unwrap(),
        bytes
    );
    let direct = Key::Direct {
        number: String::from("abcd"),
    };
    // Metadata does not turn a key codec into the intrinsic String validator.
    assert_eq!(
        direct.to_key_bytes().unwrap(),
        [0, 1, b'a', b'b', b'c', b'd', 0, 0, 0, 4]
    );
}
