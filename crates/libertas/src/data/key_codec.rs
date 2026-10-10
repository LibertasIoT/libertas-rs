//! Canonical, ascending binary key codecs.

use crate::InlineByteBuffer;
use alloc::{string::String, vec::Vec};
use core::{fmt, mem::MaybeUninit, slice};

/// Sequential writer into a key's possibly uninitialized destination.
///
/// Codec implementations use this writer so that only initialized bytes can
/// become visible. Writes are bounded; each field must fill its declared width.
pub struct KeyWriter<'a> {
    output: &'a mut [MaybeUninit<u8>],
    written: usize,
}

impl KeyWriter<'_> {
    /// Bytes still required to complete this key or field.
    pub fn remaining(&self) -> usize {
        self.output.len() - self.written
    }

    /// Appends initialized bytes without reading or zeroing the destination.
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), KeyEncodeError> {
        let destination = self.next(bytes.len())?;
        for (slot, byte) in destination.iter_mut().zip(bytes) {
            slot.write(*byte);
        }
        self.written += bytes.len();
        Ok(())
    }

    /// Appends `count` copies of a byte, including canonical padding.
    pub fn fill(&mut self, byte: u8, count: usize) -> Result<(), KeyEncodeError> {
        self.next(count)?.fill(MaybeUninit::new(byte));
        self.written += count;
        Ok(())
    }

    /// Encodes exactly `width` bytes with a separately bounded field writer.
    pub fn field(
        &mut self,
        width: usize,
        encode: impl FnOnce(&mut KeyWriter<'_>) -> Result<(), KeyEncodeError>,
    ) -> Result<(), KeyEncodeError> {
        write_exact(self.next(width)?, width, encode)?;
        self.written += width;
        Ok(())
    }

    /// Checks the complete width expected by a codec before writing.
    pub fn check_len(&self, expected: usize) -> Result<(), KeyEncodeError> {
        check_encode_len(self.remaining(), expected)
    }

    fn next(&mut self, count: usize) -> Result<&mut [MaybeUninit<u8>], KeyEncodeError> {
        let remaining = self.remaining();
        if count > remaining {
            return Err(KeyEncodeError::InvalidBufferLength {
                expected: count,
                actual: remaining,
            });
        }
        Ok(&mut self.output[self.written..self.written + count])
    }
}

fn check_encode_len(actual: usize, expected: usize) -> Result<(), KeyEncodeError> {
    if actual != expected {
        return Err(KeyEncodeError::InvalidBufferLength { expected, actual });
    }
    Ok(())
}

fn write_exact(
    output: &mut [MaybeUninit<u8>],
    expected: usize,
    encode: impl FnOnce(&mut KeyWriter<'_>) -> Result<(), KeyEncodeError>,
) -> Result<&mut [u8], KeyEncodeError> {
    check_encode_len(output.len(), expected)?;
    let mut writer = KeyWriter { output, written: 0 };
    encode(&mut writer)?;
    check_encode_len(writer.written, expected)?;
    // SAFETY: KeyWriter exposes only sequential initialized writes. Field
    // writers advance their parent only after every byte has been written.
    // Completion is checked here, independently of a custom codec's result.
    Ok(unsafe { slice::from_raw_parts_mut(writer.output.as_mut_ptr().cast(), expected) })
}

fn as_uninit(output: &mut [u8]) -> &mut [MaybeUninit<u8>] {
    // SAFETY: MaybeUninit<u8> has the same layout as u8. This private adapter
    // only feeds KeyWriter, which never replaces initialized bytes with uninit.
    unsafe { slice::from_raw_parts_mut(output.as_mut_ptr().cast(), output.len()) }
}

/// An error encoding a complete key or a fixed-width key field.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyEncodeError {
    /// The destination must have exactly the declared encoded length.
    InvalidBufferLength {
        /// Required number of bytes.
        expected: usize,
        /// Supplied number of bytes.
        actual: usize,
    },
    /// A present byte payload is shorter than its declared minimum.
    BelowMinimum {
        /// Minimum present payload length in bytes.
        minimum: usize,
        /// Actual present payload length in bytes.
        actual: usize,
    },
    /// A bounded String or byte sequence exceeds its declared byte capacity.
    CapacityExceeded {
        /// Maximum payload length in bytes.
        capacity: usize,
        /// Actual payload length in bytes.
        actual: usize,
    },
}

impl fmt::Display for KeyEncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BelowMinimum { minimum, actual } => {
                write!(f, "key field has {actual} bytes; minimum is {minimum}")
            }
            Self::InvalidBufferLength { expected, actual } => {
                write!(f, "key buffer has {actual} bytes; expected {expected}")
            }
            Self::CapacityExceeded { capacity, actual } => {
                write!(f, "key field has {actual} bytes; capacity is {capacity}")
            }
        }
    }
}

impl core::error::Error for KeyEncodeError {}

/// A malformed or unknown binary key representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyDecodeError {
    /// The encoded length does not match the selected layout.
    InvalidBufferLength {
        /// Required number of bytes (two when a root discriminator is missing).
        expected: usize,
        /// Supplied number of bytes.
        actual: usize,
    },
    /// No variant in this union has the encoded discriminator.
    UnknownDiscriminator {
        /// Unrecognized two-byte tag.
        discriminator: u16,
    },
    /// A Boolean byte is neither zero nor one.
    InvalidBoolean {
        /// Invalid encoded byte.
        value: u8,
    },
    /// An optional field's presence byte is neither zero nor one.
    InvalidPresence {
        /// Invalid encoded presence byte.
        value: u8,
    },
    /// A present byte payload is shorter than its declared minimum.
    BelowMinimum {
        /// Minimum present payload length in bytes.
        minimum: usize,
        /// Actual present payload length in bytes.
        actual: usize,
    },
    /// A bounded String or byte sequence claims more bytes than its capacity.
    InvalidLength {
        /// Declared payload capacity.
        capacity: usize,
        /// Encoded payload length.
        actual: u32,
    },
    /// A bounded field, absent optional field, or union contains nonzero padding.
    InvalidPadding,
    /// A String payload is not valid UTF-8.
    InvalidUtf8,
}

impl fmt::Display for KeyDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BelowMinimum { minimum, actual } => {
                write!(f, "key field has {actual} bytes; minimum is {minimum}")
            }
            Self::InvalidBufferLength { expected, actual } => {
                write!(f, "key input has {actual} bytes; expected {expected}")
            }
            Self::UnknownDiscriminator { discriminator } => {
                write!(f, "unknown key discriminator {discriminator}")
            }
            Self::InvalidBoolean { value } => write!(f, "invalid key Boolean byte {value}"),
            Self::InvalidPresence { value } => write!(f, "invalid key presence byte {value}"),
            Self::InvalidLength { capacity, actual } => {
                write!(f, "key length suffix is {actual}; capacity is {capacity}")
            }
            Self::InvalidPadding => f.write_str("nonzero key padding"),
            Self::InvalidUtf8 => f.write_str("invalid UTF-8 in key String"),
        }
    }
}

impl core::error::Error for KeyDecodeError {}

/// Encodes a complete key selected by a root union.
///
/// Derive with [`libertas_macros::LibertasKeyCodec`] on an enum. Its zero-based
/// variant index is the two-byte big-endian discriminator, followed by that
/// variant's fields. No `repr` or explicit discriminants are needed. Different
/// variants can have different encoded lengths; no root padding is added.
///
/// All fields have fixed-width encodings. Bytewise unsigned lexicographic
/// comparison defines key ordering and equality. Published variants and their
/// field layouts must remain unchanged; append new variants for new key types.
/// Decoding is deliberately independent of this encoding trait.
///
/// ```
/// use libertas::data::KeyCodec;
/// use libertas_macros::LibertasKeyCodec;
///
/// #[derive(LibertasKeyCodec)]
/// enum Key {
///     Settings,
///     Reading { device: u16, timestamp: i32 },
/// }
/// assert_eq!(Key::Settings.to_key_bytes()?, [0, 0]);
/// let reading = Key::Reading { device: 42, timestamp: -1 };
/// assert_eq!(reading.to_key_bytes()?, [0, 1, 0, 42, 0x7f, 0xff, 0xff, 0xff]);
/// # Ok::<(), libertas::data::KeyEncodeError>(())
/// ```
///
/// Only an enum can be a key root:
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyCodec)]
/// struct Key { value: u64 }
/// ```
pub trait KeyCodec {
    /// Exact encoded length of the selected variant, including its tag.
    fn encoded_len(&self) -> usize;

    /// Writes the complete encoding sequentially. Derives implement this method.
    fn write_key(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError>;

    /// Writes a complete key into an exactly sized destination.
    ///
    /// The destination may have been partially written on error. Callers must
    /// discard it in that case. Successful writes initialize every byte.
    fn encode_key(&self, output: &mut [u8]) -> Result<(), KeyEncodeError> {
        write_exact(as_uninit(output), self.encoded_len(), |writer| {
            self.write_key(writer)
        })?;
        Ok(())
    }

    /// Encodes directly into exactly sized uninitialized storage.
    ///
    /// Returns the initialized bytes only on success. On error, some bytes may
    /// have been written, but the destination must still be treated as uninitialized.
    fn encode_key_uninit<'a>(
        &self,
        output: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut [u8], KeyEncodeError> {
        write_exact(output, self.encoded_len(), |writer| self.write_key(writer))
    }

    /// Appends directly to inline or heap spare capacity without preliminary
    /// zeroing. On error the buffer's initialized length and prefix are unchanged.
    fn encode_key_to_buffer(&self, buffer: &mut InlineByteBuffer) -> Result<(), KeyEncodeError> {
        let width = self.encoded_len();
        let previous = buffer.len();
        buffer.reserve(width);
        write_exact(&mut buffer.spare_capacity_mut()[..width], width, |writer| {
            self.write_key(writer)
        })?;
        // SAFETY: write_exact verified initialization of the entire new suffix.
        unsafe { buffer.set_len(previous + width) };
        Ok(())
    }

    /// Allocates and encodes a complete key.
    fn to_key_bytes(&self) -> Result<Vec<u8>, KeyEncodeError> {
        let width = self.encoded_len();
        let mut output = Vec::with_capacity(width);
        write_exact(&mut output.spare_capacity_mut()[..width], width, |writer| {
            self.write_key(writer)
        })?;
        // SAFETY: write_exact verified initialization of all width bytes.
        unsafe { output.set_len(width) };
        Ok(output)
    }
}

/// Decodes a complete root key, consuming exactly one entire encoding.
///
/// Derive [`libertas_macros::LibertasKeyDecode`] alongside
/// [`libertas_macros::LibertasKeyCodec`]. Every component must support decoding.
/// Unknown tags, truncated or trailing bytes, and noncanonical fields return
/// errors. A decoder never interprets an unknown variant as an absent record.
/// Encoding-only types do not need to implement this trait.
///
/// ```
/// use libertas::data::{KeyCodec, KeyDecode};
/// use libertas_macros::{LibertasKeyCodec, LibertasKeyDecode};
/// #[derive(Debug, PartialEq, LibertasKeyCodec, LibertasKeyDecode)]
/// enum Key {
///     Settings,
///     Reading { timestamp: i64, value: f64 },
/// }
/// let original = Key::Reading { timestamp: -1, value: -0.0 };
/// let bytes = original.to_key_bytes().unwrap();
/// assert_eq!(Key::decode_key(&bytes).unwrap(), original);
/// ```
///
/// Components of a decoding root must opt into field decoding too:
///
/// ```compile_fail
/// use libertas_macros::{LibertasKeyCodec, LibertasKeyDecode, LibertasKeyFieldCodec};
/// #[derive(LibertasKeyFieldCodec)]
/// struct EncodingOnly { value: u64 }
/// #[derive(LibertasKeyCodec, LibertasKeyDecode)]
/// enum Key { Value(EncodingOnly) }
/// ```
pub trait KeyDecode: KeyCodec + Sized {
    /// Decodes an exact, complete key representation.
    fn decode_key(input: &[u8]) -> Result<Self, KeyDecodeError>;
}

/// Encodes a component with one fixed binary width for all of its values.
///
/// Integers use fixed-width big-endian bytes; signed integers first flip their
/// sign bit. Booleans use `0` and `1`. Byte arrays are copied unchanged.
/// Floats use the total order of [`f32::total_cmp`] / [`f64::total_cmp`],
/// preserving every IEEE bit. Negative zero and positive zero are distinct
/// keys, as are different NaN signs and payloads. Key equality compares encoded
/// bytes, not the float's ordinary `PartialEq`.
/// Derive [`libertas_macros::LibertasKeyFieldCodec`] for named structs and
/// enums. Structs concatenate fields in declaration order. A field enum uses
/// a two-byte tag and a zero-padded payload slot sized to its largest variant.
/// Enum tags are zero-based declaration indexes encoded as big-endian `u16`;
/// no `repr` annotation or explicit discriminants are needed. Existing variants
/// must never be reordered or removed. A published field enum is frozen.
///
/// `Option<T>` adds one presence byte to `T`'s fixed width. `None` is zero
/// followed by an all-zero payload slot; `Some` is one followed by `T`'s
/// encoding. `None` sorts before every `Some`, which retains `T`'s ordering.
/// Directly nested `Option<Option<T>>` types are rejected, including through
/// aliases and generic substitutions. Declared enum tags remain `u16`.
///
/// A String or `Vec<u8>` field requires `#[libertas_size(max = N)]`. It occupies
/// `N + 4` bytes: content, zero padding, then a big-endian `u32` content length.
/// String capacity is measured in UTF-8 bytes; encoding does not normalize or
/// truncate values. The length suffix preserves distinctions involving NULs.
/// Capacities must be compile-time constants fitting `u32`, and complete
/// encoded widths must fit `usize` on the compilation target. Arrays other
/// than `[u8; N]` are not supported in this version.
/// `#[libertas_size(min = N, max = N)]` instead requires exactly `N` bytes
/// and omits the length suffix. A smaller explicit minimum constrains present
/// content without changing the `max + 4` width. These bounds belong on the
/// field; attributes on a Rust type alias cannot specialize its codec.
/// The same annotations support optional String/Vec fields: one `Option`
/// adds one presence byte, even when absent. None bypasses the minimum.
/// Aliases of a single optional layer use the same rule.
///
/// Key layouts are acyclic. `Box<T>` is not a key field representation:
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct Invalid { value: Box<u16> }
/// ```
///
/// ```
/// use libertas::data::{KeyFieldCodec, KeyFieldDecode};
/// let mut bytes = [0xff; <Option<u16>>::ENCODED_SIZE];
/// None::<u16>.encode_field(&mut bytes).unwrap();
/// assert_eq!(bytes, [0, 0, 0]);
/// Some(256u16).encode_field(&mut bytes).unwrap();
/// assert_eq!(bytes, [1, 1, 0]);
/// assert_eq!(Option::<u16>::decode_field(&bytes).unwrap(), Some(256));
/// ```
///
/// Nested optional layouts fail during constant evaluation, even through aliases:
///
/// ```compile_fail
/// use libertas::data::KeyFieldCodec;
/// type MaybeId = Option<u16>;
/// const _: usize = <Option<MaybeId>>::ENCODED_SIZE;
/// ```
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct Invalid { value: Option<Option<u16>> }
/// ```
///
/// ```compile_fail
/// type MaybeId = Option<u16>;
/// #[derive(libertas_macros::LibertasKeyCodec)]
/// enum Invalid { Value(Option<MaybeId>) }
/// ```
///
/// The same restriction applies to bounded bytes and generic payloads:
///
/// ```compile_fail
/// type MaybeText = Option<String>;
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct Invalid { #[libertas_size(max = 4)] value: Option<MaybeText> }
/// ```
///
/// ```compile_fail
/// use libertas::data::KeyFieldDecode;
/// fn decode<T: KeyFieldDecode>() {
///     let _ = Option::<T>::decode_field(&[]);
/// }
/// decode::<Option<u16>>();
/// ```
///
/// Absence does not make a nested optional type a valid layout:
///
/// ```compile_fail
/// use libertas::data::KeyFieldCodec;
/// None::<Option<u16>>.encode_field(&mut [0; 5]).unwrap();
/// ```
///
/// ```
/// use libertas::data::KeyFieldCodec;
/// use libertas_macros::LibertasKeyFieldCodec;
///
/// #[derive(LibertasKeyFieldCodec)]
/// struct Name {
///     #[libertas_size(max = 4)]
///     name: String,
/// }
/// let mut bytes = [0xff; Name::ENCODED_SIZE];
/// Name { name: "a\0".into() }.encode_field(&mut bytes)?;
/// assert_eq!(bytes, [b'a', 0, 0, 0, 0, 0, 0, 2]);
/// # Ok::<(), libertas::data::KeyEncodeError>(())
/// ```
///
/// Unbounded fields and machine-dependent integer widths are unsupported:
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct Unbounded { name: String }
/// ```
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct MachineSized { value: usize }
/// ```
///
/// A root codec alone cannot be embedded as a field:
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyCodec)]
/// enum Root { Only }
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct Nested { root: Root }
/// ```
///
/// Byte-capacity annotations apply to String/Vec fields and their Option wrappers:
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct Invalid {
///     #[libertas_size(max = 4)]
///     value: u32,
/// }
/// ```
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct UnboundedOptional { value: Option<String> }
/// ```
///
/// ```compile_fail
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct InvalidOptional {
///     #[libertas_size(max = 4)]
///     value: Option<u32>,
/// }
/// ```
///
/// Invalid widths fail during constant evaluation:
///
/// ```compile_fail
/// use libertas::data::KeyFieldCodec;
/// #[derive(libertas_macros::LibertasKeyFieldCodec)]
/// struct Invalid {
///     #[libertas_size(max = usize::MAX)]
///     value: String,
/// }
/// const _: usize = Invalid::ENCODED_SIZE;
/// ```
///
/// The root encoder also requires capacities to be constant:
///
/// ```compile_fail
/// fn capacity() -> usize { 4 }
/// #[derive(libertas_macros::LibertasKeyCodec)]
/// enum Invalid {
///     Value { #[libertas_size(max = capacity())] value: String },
/// }
/// ```
pub trait KeyFieldCodec {
    /// Whether this codec is itself an optional layer, rather than a payload
    /// that may contain optional fields. Used to reject nested Option layouts.
    #[doc(hidden)]
    const IS_OPTIONAL: bool = false;

    /// Exact encoded width, independent of the value.
    const ENCODED_SIZE: usize;

    /// Writes the complete field sequentially. Derives implement this method.
    fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError>;

    /// Encodes into a destination of exactly [`Self::ENCODED_SIZE`] bytes.
    ///
    /// The destination may have been partially written on error. Successful
    /// writes initialize every byte, including canonical zero padding.
    fn encode_field(&self, output: &mut [u8]) -> Result<(), KeyEncodeError> {
        write_exact(as_uninit(output), Self::ENCODED_SIZE, |writer| {
            self.write_field(writer)
        })?;
        Ok(())
    }

    /// Encodes directly into uninitialized storage and returns initialized bytes
    /// only after the complete fixed-width field has been written.
    fn encode_field_uninit<'a>(
        &self,
        output: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut [u8], KeyEncodeError> {
        write_exact(output, Self::ENCODED_SIZE, |writer| {
            self.write_field(writer)
        })
    }
}

/// Decodes one fixed-width component using its [`KeyFieldCodec`] layout.
///
/// Derive [`libertas_macros::LibertasKeyFieldDecode`] alongside
/// [`libertas_macros::LibertasKeyFieldCodec`] for structs or field enums.
/// The decoder rejects wrong lengths, unknown tags, nonzero padding, invalid
/// Boolean/presence bytes, invalid length suffixes, and invalid String UTF-8. String
/// and byte-vector results allocate only their actual content. Floats preserve
/// all bits, including NaN payloads and signed zero, without normalization.
pub trait KeyFieldDecode: KeyFieldCodec + Sized {
    /// Decodes exactly [`Self::ENCODED_SIZE`] bytes.
    fn decode_field(input: &[u8]) -> Result<Self, KeyDecodeError>;
}

macro_rules! unsigned {
    ($($ty:ty),* $(,)?) => {$(
        impl KeyFieldCodec for $ty {
            const ENCODED_SIZE: usize = core::mem::size_of::<Self>();

            fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
                output.check_len(Self::ENCODED_SIZE)?;
                output.write(&self.to_be_bytes())?;
                Ok(())
            }
        }
        impl KeyFieldDecode for $ty {
            fn decode_field(input: &[u8]) -> Result<Self, KeyDecodeError> {
                Ok(Self::from_be_bytes(__private::read_array(input)?))
            }
        }
    )*};
}

macro_rules! signed {
    ($($ty:ty => $word:ty),* $(,)?) => {$(
        impl KeyFieldCodec for $ty {
            const ENCODED_SIZE: usize = core::mem::size_of::<Self>();

            fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
                output.check_len(Self::ENCODED_SIZE)?;
                let ordered = (*self as $word) ^ (1 << (<$word>::BITS - 1));
                output.write(&ordered.to_be_bytes())?;
                Ok(())
            }
        }
        impl KeyFieldDecode for $ty {
            fn decode_field(input: &[u8]) -> Result<Self, KeyDecodeError> {
                let ordered = <$word>::from_be_bytes(__private::read_array(input)?);
                Ok((ordered ^ (1 << (<$word>::BITS - 1))) as Self)
            }
        }
    )*};
}

unsigned!(u8, u16, u32, u64, u128);
signed!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128);

macro_rules! floats {
    ($($ty:ty => $word:ty),* $(,)?) => {$(
        impl KeyFieldCodec for $ty {
            const ENCODED_SIZE: usize = core::mem::size_of::<Self>();

            fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
                output.check_len(Self::ENCODED_SIZE)?;
                let bits = self.to_bits();
                let sign = 1 << (<$word>::BITS - 1);
                let ordered = if bits & sign != 0 { !bits } else { bits ^ sign };
                output.write(&ordered.to_be_bytes())?;
                Ok(())
            }
        }
        impl KeyFieldDecode for $ty {
            fn decode_field(input: &[u8]) -> Result<Self, KeyDecodeError> {
                let ordered = <$word>::from_be_bytes(__private::read_array(input)?);
                let sign = 1 << (<$word>::BITS - 1);
                let bits = if ordered & sign != 0 { ordered ^ sign } else { !ordered };
                Ok(Self::from_bits(bits))
            }
        }
    )*};
}

floats!(f32 => u32, f64 => u64);

impl KeyFieldCodec for bool {
    const ENCODED_SIZE: usize = 1;

    fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
        output.check_len(Self::ENCODED_SIZE)?;
        output.write(&[u8::from(*self)])?;
        Ok(())
    }
}

impl KeyFieldDecode for bool {
    fn decode_field(input: &[u8]) -> Result<Self, KeyDecodeError> {
        match __private::read_array::<1>(input)?[0] {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(KeyDecodeError::InvalidBoolean { value }),
        }
    }
}

impl<const N: usize> KeyFieldCodec for [u8; N] {
    const ENCODED_SIZE: usize = N;

    fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
        output.check_len(N)?;
        output.write(self)?;
        Ok(())
    }
}

impl<const N: usize> KeyFieldDecode for [u8; N] {
    fn decode_field(input: &[u8]) -> Result<Self, KeyDecodeError> {
        __private::read_array(input)
    }
}

impl<T: KeyFieldCodec> KeyFieldCodec for Option<T> {
    const IS_OPTIONAL: bool = true;
    const ENCODED_SIZE: usize = {
        assert!(
            !T::IS_OPTIONAL,
            "nested Option key fields are not supported"
        );
        __private::add_width(1, T::ENCODED_SIZE)
    };

    fn write_field(&self, output: &mut KeyWriter<'_>) -> Result<(), KeyEncodeError> {
        const {
            let _ = Self::ENCODED_SIZE;
        }
        __private::encode_optional(self, output, T::ENCODED_SIZE, T::write_field)
    }
}

impl<T: KeyFieldDecode> KeyFieldDecode for Option<T> {
    fn decode_field(input: &[u8]) -> Result<Self, KeyDecodeError> {
        const {
            let _ = Self::ENCODED_SIZE;
        }
        __private::decode_optional(input, T::ENCODED_SIZE, T::decode_field)
    }
}

/// Implementation support for generated codecs, not a standalone authoring API.
#[doc(hidden)]
pub mod __private {
    use super::*;

    pub fn check_decode_len(input: &[u8], expected: usize) -> Result<(), KeyDecodeError> {
        if input.len() != expected {
            return Err(KeyDecodeError::InvalidBufferLength {
                expected,
                actual: input.len(),
            });
        }
        Ok(())
    }

    pub fn read_array<const N: usize>(input: &[u8]) -> Result<[u8; N], KeyDecodeError> {
        input
            .try_into()
            .map_err(|_| KeyDecodeError::InvalidBufferLength {
                expected: N,
                actual: input.len(),
            })
    }

    pub fn take<'a>(input: &mut &'a [u8], width: usize) -> Result<&'a [u8], KeyDecodeError> {
        if input.len() < width {
            return Err(KeyDecodeError::InvalidBufferLength {
                expected: width,
                actual: input.len(),
            });
        }
        let (field, rest) = input.split_at(width);
        *input = rest;
        Ok(field)
    }

    pub fn check_padding(input: &[u8]) -> Result<(), KeyDecodeError> {
        if input.iter().any(|byte| *byte != 0) {
            return Err(KeyDecodeError::InvalidPadding);
        }
        Ok(())
    }

    #[derive(Clone, Copy)]
    pub struct ByteBounds {
        minimum: usize,
        capacity: usize,
        exact: bool,
    }

    impl ByteBounds {
        pub const fn new(minimum: Option<usize>, capacity: usize) -> Self {
            assert!(capacity <= u32::MAX as usize, "key capacity exceeds u32");
            let (minimum, exact) = match minimum {
                Some(minimum) => (minimum, minimum == capacity),
                None => (0, false),
            };
            assert!(minimum <= capacity, "key minimum exceeds maximum");
            Self {
                minimum,
                capacity,
                exact,
            }
        }
        pub const fn width(self) -> usize {
            if self.exact {
                self.capacity
            } else {
                add_width(self.capacity, 4)
            }
        }
    }

    pub const fn add_width(left: usize, right: usize) -> usize {
        match left.checked_add(right) {
            Some(width) => width,
            None => panic!("key encoded width overflows usize"),
        }
    }

    pub fn encode_optional<T>(
        value: &Option<T>,
        output: &mut KeyWriter<'_>,
        payload_width: usize,
        encode: impl FnOnce(&T, &mut KeyWriter<'_>) -> Result<(), KeyEncodeError>,
    ) -> Result<(), KeyEncodeError> {
        output.check_len(add_width(1, payload_width))?;
        match value {
            None => output.fill(0, output.remaining())?,
            Some(value) => {
                output.write(&[1])?;
                output.field(payload_width, |writer| encode(value, writer))?;
            }
        }
        Ok(())
    }

    pub fn decode_optional<T>(
        input: &[u8],
        payload_width: usize,
        decode: impl FnOnce(&[u8]) -> Result<T, KeyDecodeError>,
    ) -> Result<Option<T>, KeyDecodeError> {
        check_decode_len(input, add_width(1, payload_width))?;
        match input[0] {
            0 => {
                check_padding(&input[1..])?;
                Ok(None)
            }
            1 => decode(&input[1..]).map(Some),
            value => Err(KeyDecodeError::InvalidPresence { value }),
        }
    }

    mod sealed {
        pub trait Sealed {}
        impl Sealed for super::String {}
        impl Sealed for super::Vec<u8> {}
        impl<T: Sealed> Sealed for Option<T> {}
    }

    // Trait dispatch supports native aliases without guessing types by name in
    // the proc macro. Supports the two byte-sequence types and one Option layer.
    pub trait BoundedBytes: sealed::Sealed + Sized {
        const OPTIONAL_DEPTH: usize;
        fn encode_bounded(
            &self,
            bounds: ByteBounds,
            output: &mut KeyWriter<'_>,
        ) -> Result<(), KeyEncodeError>;
        fn decode_bounded(input: &[u8], bounds: ByteBounds) -> Result<Self, KeyDecodeError>;
    }

    pub const fn bounded_field_width<T: BoundedBytes>(bounds: ByteBounds) -> usize {
        add_width(T::OPTIONAL_DEPTH, bounds.width())
    }

    impl BoundedBytes for String {
        const OPTIONAL_DEPTH: usize = 0;

        fn encode_bounded(
            &self,
            bounds: ByteBounds,
            output: &mut KeyWriter<'_>,
        ) -> Result<(), KeyEncodeError> {
            encode_bytes(self.as_bytes(), bounds, output)
        }
        fn decode_bounded(input: &[u8], bounds: ByteBounds) -> Result<Self, KeyDecodeError> {
            core::str::from_utf8(decode_bytes(input, bounds)?)
                .map(String::from)
                .map_err(|_| KeyDecodeError::InvalidUtf8)
        }
    }

    impl BoundedBytes for Vec<u8> {
        const OPTIONAL_DEPTH: usize = 0;

        fn encode_bounded(
            &self,
            bounds: ByteBounds,
            output: &mut KeyWriter<'_>,
        ) -> Result<(), KeyEncodeError> {
            encode_bytes(self, bounds, output)
        }
        fn decode_bounded(input: &[u8], bounds: ByteBounds) -> Result<Self, KeyDecodeError> {
            Ok(decode_bytes(input, bounds)?.to_vec())
        }
    }

    impl<T: BoundedBytes> BoundedBytes for Option<T> {
        const OPTIONAL_DEPTH: usize = {
            assert!(
                T::OPTIONAL_DEPTH == 0,
                "nested Option key fields are not supported"
            );
            1
        };

        fn encode_bounded(
            &self,
            bounds: ByteBounds,
            output: &mut KeyWriter<'_>,
        ) -> Result<(), KeyEncodeError> {
            const {
                let _ = Self::OPTIONAL_DEPTH;
            }
            encode_optional(
                self,
                output,
                bounded_field_width::<T>(bounds),
                |value, output| T::encode_bounded(value, bounds, output),
            )
        }
        fn decode_bounded(input: &[u8], bounds: ByteBounds) -> Result<Self, KeyDecodeError> {
            const {
                let _ = Self::OPTIONAL_DEPTH;
            }
            decode_optional(input, bounded_field_width::<T>(bounds), |input| {
                T::decode_bounded(input, bounds)
            })
        }
    }

    pub fn encode_bounded<T: BoundedBytes>(
        value: &T,
        bounds: ByteBounds,
        output: &mut KeyWriter<'_>,
    ) -> Result<(), KeyEncodeError> {
        T::encode_bounded(value, bounds, output)
    }

    fn encode_bytes(
        bytes: &[u8],
        bounds: ByteBounds,
        output: &mut KeyWriter<'_>,
    ) -> Result<(), KeyEncodeError> {
        output.check_len(bounds.width())?;
        if bytes.len() > bounds.capacity {
            return Err(KeyEncodeError::CapacityExceeded {
                capacity: bounds.capacity,
                actual: bytes.len(),
            });
        }
        if bytes.len() < bounds.minimum {
            return Err(KeyEncodeError::BelowMinimum {
                minimum: bounds.minimum,
                actual: bytes.len(),
            });
        }
        output.write(bytes)?;
        if bounds.exact {
            return Ok(());
        }
        output.fill(0, bounds.capacity - bytes.len())?;
        output.write(&(bytes.len() as u32).to_be_bytes())?;
        Ok(())
    }

    pub fn decode_bounded<T: BoundedBytes>(
        input: &[u8],
        bounds: ByteBounds,
    ) -> Result<T, KeyDecodeError> {
        T::decode_bounded(input, bounds)
    }

    fn decode_bytes(input: &[u8], bounds: ByteBounds) -> Result<&[u8], KeyDecodeError> {
        check_decode_len(input, bounds.width())?;
        if bounds.exact {
            return Ok(input);
        }
        let length = u32::from_be_bytes(read_array(&input[bounds.capacity..])?);
        if length as usize > bounds.capacity {
            return Err(KeyDecodeError::InvalidLength {
                capacity: bounds.capacity,
                actual: length,
            });
        }
        if (length as usize) < bounds.minimum {
            return Err(KeyDecodeError::BelowMinimum {
                minimum: bounds.minimum,
                actual: length as usize,
            });
        }
        check_padding(&input[length as usize..bounds.capacity])?;
        Ok(&input[..length as usize])
    }
}
