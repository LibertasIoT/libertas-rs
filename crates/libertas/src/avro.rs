use alloc::vec::Vec;
use alloc::string::String;
use alloc::boxed::Box;

fn take_bytes<'a>(
    buffer: &'a [u8],
    offset: &mut usize,
    len: usize,
    eof_error: &'static str,
) -> Result<&'a [u8], &'static str> {
    let end = offset.checked_add(len).ok_or(eof_error)?;
    let bytes = buffer.get(*offset..end).ok_or(eof_error)?;
    *offset = end;
    Ok(bytes)
}

/// A trait for encoding Rust data types into Avro format, which is a compact binary serialization format. Types that implement the `AvroEncode` trait can be encoded into a byte buffer in Avro format using the `avro_encode` method. This trait is essential for converting Rust data structures into a format that can be stored or transmitted efficiently, especially when working with indexed data in the indexed database. The `avro_encode` method takes a mutable reference to a `Vec<u8>` buffer, where the encoded bytes will be appended. Implementing this trait for your custom data types allows you to easily serialize them into Avro format for storage or communication purposes.
pub trait AvroEncode {
    /// Encode the implementing type into Avro format and append the encoded bytes to the provided buffer. The `avro_encode` method is responsible for converting the data of the implementing type into a compact binary representation according to the Avro specification. The encoded bytes are appended to the `buffer`, which is a mutable reference to a `Vec<u8>`. This allows you to build up a byte buffer containing the Avro-encoded data, which can then be stored in the indexed database or transmitted over a network. When implementing this method for your custom types, you should ensure that the encoding follows the Avro format correctly so that it can be decoded later using the corresponding `AvroDecode` trait.
    fn avro_encode(&self, buffer: &mut Vec<u8>);
}

/// A trait for decoding Rust data types from Avro format, which is a compact binary serialization format. Types that implement the `AvroDecode` trait can be decoded from a byte buffer in Avro format using the `avro_decode` method. This trait is essential for converting Avro-encoded byte data back into usable Rust data structures, especially when working with indexed data read from the indexed database. The `avro_decode` method takes a byte slice (`&[u8]`) containing the Avro-encoded data and a mutable reference to an offset (`&mut usize`) that tracks the current position in the buffer. The method returns a `Result<Self, &'static str>`, where `Self` is the type being decoded, and the error string provides information about any decoding issues that may arise, such as unexpected end of buffer or invalid data formats. When implementing this method for your custom types, you should ensure that the decoding logic correctly interprets the Avro format and updates the offset accordingly as bytes are read from the buffer.
pub trait AvroDecode: Sized {
    /// Decode an instance of the implementing type from Avro format using the provided byte buffer and offset. The `avro_decode` method is responsible for reading bytes from the `buffer` starting at the position indicated by `offset`, interpreting those bytes according to the Avro specification, and constructing an instance of the implementing type. The method returns a `Result<Self, &'static str>`, where `Self` is the type being decoded, and the error string provides information about any issues that may arise during decoding, such as reaching the end of the buffer unexpectedly or encountering invalid data formats. When implementing this method for your custom types, you should ensure that the decoding logic correctly follows the Avro format and updates the offset as bytes are consumed from the buffer to allow for sequential decoding of multiple values. 
    ///
    /// The input may come from an untrusted peer. Implementations must return
    /// `Err` for invalid input and must not panic because endpoint listeners
    /// convert decode errors into the platform `InvalidMessage` status.
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str>;
}

/// Marker traits to indicate that a type is not encoded as raw bytes in Avro format. These traits can be used to prevent certain types from being treated as raw byte arrays when encoding or decoding data in Avro format. For example, types like `String` or `Vec<T>` should not be treated as raw bytes, as they have specific encoding rules in Avro. By implementing these marker traits for such types, you can ensure that they are encoded and decoded correctly according to the Avro specification, rather than being treated as opaque byte arrays. The `NotBytesEncode` trait indicates that a type should not be encoded as raw bytes, while the `NotBytesDecode` trait indicates that a type should not be decoded from raw bytes. Implementing these traits for your custom types can help ensure that they are handled correctly during Avro encoding and decoding processes.
pub trait NotBytesEncode {}
/// Marker trait to indicate that a type is not decoded from raw bytes in Avro format. This trait can be used to prevent certain types from being treated as raw byte arrays when decoding data in Avro format. For example, types like `String` or `Vec<T>` should not be treated as raw bytes, as they have specific decoding rules in Avro. By implementing this marker trait for such types, you can ensure that they are decoded correctly according to the Avro specification, rather than being treated as opaque byte arrays. Implementing this trait for your custom types can help ensure that they are handled correctly during the Avro decoding process.   
pub trait NotBytesDecode {}

impl NotBytesEncode for bool {}
impl NotBytesDecode for bool {}
impl NotBytesEncode for i32 {}
impl NotBytesDecode for i32 {}
impl NotBytesEncode for i64 {}
impl NotBytesDecode for i64 {}
impl NotBytesEncode for f32 {}
impl NotBytesDecode for f32 {}
impl NotBytesEncode for f64 {}
impl NotBytesDecode for f64 {}
impl NotBytesEncode for String {}
impl NotBytesDecode for String {}
impl NotBytesEncode for &str {}
impl<T> NotBytesEncode for &[T] {}
impl<T> NotBytesEncode for Vec<T> {}
impl<T> NotBytesDecode for Vec<T> {}
impl<T> NotBytesEncode for Option<T> {}
impl<T> NotBytesDecode for Option<T> {}
impl<T> NotBytesEncode for Box<T> {}
impl<T> NotBytesDecode for Box<T> {}

impl AvroEncode for bool {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(if *self { 1 } else { 0 });
    }
}


impl AvroDecode for bool {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        if *offset >= buffer.len() { return Err("EOF while reading bool"); }
        let v = buffer[*offset];
        *offset += 1;
        match v {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Invalid bool byte"),
        }
    }
}


impl AvroEncode for i32 {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        let n = (*self << 1) ^ (*self >> 31);
        let mut n = n as u32;
        loop {
            let mut byte = (n & 0x7F) as u8;
            n >>= 7;
            if n != 0 {
                byte |= 0x80;
            }
            buffer.push(byte);
            if n == 0 { break; }
        }
    }
}


impl AvroDecode for i32 {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let mut result: u32 = 0;
        let mut shift = 0;
        loop {
            if *offset >= buffer.len() { return Err("EOF while reading i32"); }
            let byte = buffer[*offset];
            *offset += 1;
            if shift == 28 && byte & 0xf0 != 0 {
                return Err("Invalid terminal byte for i32");
            }
            result |= ((byte & 0x7F) as u32) << shift;
            if byte & 0x80 == 0 { break; }
            shift += 7;
            if shift >= 32 { return Err("Too many bytes for i32"); }
        }
        let decoded = ((result >> 1) as i32) ^ -( (result & 1) as i32 );
        Ok(decoded)
    }
}


impl AvroEncode for i64 {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        let n = (*self << 1) ^ (*self >> 63);
        let mut n = n as u64;
        loop {
            let mut byte = (n & 0x7F) as u8;
            n >>= 7;
            if n != 0 {
                byte |= 0x80;
            }
            buffer.push(byte);
            if n == 0 { break; }
        }
    }
}


impl AvroDecode for i64 {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let mut result: u64 = 0;
        let mut shift = 0;
        loop {
            if *offset >= buffer.len() { return Err("EOF while reading i64"); }
            let byte = buffer[*offset];
            *offset += 1;
            if shift == 63 && byte & 0xfe != 0 {
                return Err("Invalid terminal byte for i64");
            }
            result |= ((byte & 0x7F) as u64) << shift;
            if byte & 0x80 == 0 { break; }
            shift += 7;
            if shift >= 64 { return Err("Too many bytes for i64"); }
        }
        let decoded = ((result >> 1) as i64) ^ -( (result & 1) as i64 );
        Ok(decoded)
    }
}


impl AvroEncode for f32 {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_le_bytes());
    }
}


impl AvroDecode for f32 {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(take_bytes(buffer, offset, 4, "EOF while reading f32")?);
        Ok(f32::from_le_bytes(bytes))
    }
}


impl AvroEncode for f64 {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_le_bytes());
    }
}


impl AvroDecode for f64 {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(take_bytes(buffer, offset, 8, "EOF while reading f64")?);
        Ok(f64::from_le_bytes(bytes))
    }
}

impl AvroEncode for String {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        let len = self.len() as i64;
        len.avro_encode(buffer);
        buffer.extend_from_slice(self.as_bytes());
    }
}


impl AvroDecode for String {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let len = i64::avro_decode(buffer, offset)?;
        if len < 0 { return Err("Negative length for string"); }
        let len = usize::try_from(len).map_err(|_| "String length is too large")?;
        let bytes = take_bytes(buffer, offset, len, "EOF while reading string")?;
        String::from_utf8(bytes.to_vec()).map_err(|_| "Invalid UTF-8 in string")
    }
}

impl AvroEncode for &str {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        let len = bytes.len() as i64;
        len.avro_encode(buffer);
        buffer.extend_from_slice(bytes);
    }
}

impl<T> AvroEncode for &[T] 
where 
    T: AvroEncode 
{
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        let len = self.len() as i64;

        if len == 0 {
            // Avro represents an empty array as a single 0 byte (0 count block)
            buffer.push(0);
            return;
        }
        len.avro_encode(buffer);

        // 2. Encode each element in the block
        for item in *self {
            item.avro_encode(buffer);
        }

        // 3. Write a 0 to indicate the end of blocks
        buffer.push(0);
    }
}

impl<A: AvroEncode> AvroEncode for (A,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
    }
}


impl<A: AvroDecode> AvroDecode for (A,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((A::avro_decode(buffer, offset)?,))
    }
}


impl<A: AvroEncode, B: AvroEncode> AvroEncode for (A, B) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode> AvroDecode for (A, B) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode> AvroEncode for (A, B, C) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode> AvroDecode for (A, B, C) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode> AvroEncode for (A, B, C, D) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode> AvroDecode for (A, B, C, D) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode> AvroEncode for (A, B, C, D, E) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode> AvroDecode for (A, B, C, D, E) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode> AvroEncode for (A, B, C, D, E, F) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode> AvroDecode for (A, B, C, D, E, F) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode> AvroEncode for (A, B, C, D, E, F, G) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode> AvroDecode for (A, B, C, D, E, F, G) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, 
        G: AvroEncode, H: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, 
        G: AvroDecode, H: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, 
        G: AvroEncode, H: AvroEncode, I: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, 
        G: AvroDecode, H: AvroDecode, I: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, 
        G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, 
        G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, 
        G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, 
        G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, 
        G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode> 
        AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, 
        G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode> 
        AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, 
        G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode,
        M: AvroEncode> 
        AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
    }
}


impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, 
        G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, 
        M: AvroDecode> 
        AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A> NotBytesEncode for (A,) {}
impl<A> NotBytesDecode for (A,) {}
impl<A, B> NotBytesEncode for (A, B) {}
impl<A, B> NotBytesDecode for (A, B) {}
impl<A, B, C> NotBytesEncode for (A, B, C) {}
impl<A, B, C> NotBytesDecode for (A, B, C) {}
impl<A, B, C, D> NotBytesEncode for (A, B, C, D) {}
impl<A, B, C, D> NotBytesDecode for (A, B, C, D) {}
impl<A, B, C, D, E> NotBytesEncode for (A, B, C, D, E) {}
impl<A, B, C, D, E> NotBytesDecode for (A, B, C, D, E) {}
impl<A, B, C, D, E, F> NotBytesEncode for (A, B, C, D, E, F) {}
impl<A, B, C, D, E, F> NotBytesDecode for (A, B, C, D, E, F) {}
impl<A, B, C, D, E, F, G> NotBytesEncode for (A, B, C, D, E, F, G) {}
impl<A, B, C, D, E, F, G> NotBytesDecode for (A, B, C, D, E, F, G) {}
impl<A, B, C, D, E, F, G, H> NotBytesEncode for (A, B, C, D, E, F, G, H) {}
impl<A, B, C, D, E, F, G, H> NotBytesDecode for (A, B, C, D, E, F, G, H) {}
impl<A, B, C, D, E, F, G, H, I> NotBytesEncode for (A, B, C, D, E, F, G, H, I) {}
impl<A, B, C, D, E, F, G, H, I> NotBytesDecode for (A, B, C, D, E, F, G, H, I) {}
impl<A, B, C, D, E, F, G, H, I, J> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J) {}
impl<A, B, C, D, E, F, G, H, I, J> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J) {}
impl<A, B, C, D, E, F, G, H, I, J, K> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K) {}
impl<A, B, C, D, E, F, G, H, I, J, K> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M) {}

impl AvroEncode for u8 { fn avro_encode(&self, buffer: &mut Vec<u8>) { (*self as i32).avro_encode(buffer); } }
impl AvroDecode for u8 { fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> { u8::try_from(i32::avro_decode(buffer, offset)?).map_err(|_| "Decoded value does not fit u8") } }

impl AvroEncode for i8 { fn avro_encode(&self, buffer: &mut Vec<u8>) { (*self as i32).avro_encode(buffer); } }
impl AvroDecode for i8 { fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> { i8::try_from(i32::avro_decode(buffer, offset)?).map_err(|_| "Decoded value does not fit i8") } }

impl AvroEncode for i16 { fn avro_encode(&self, buffer: &mut Vec<u8>) { (*self as i32).avro_encode(buffer); } }
impl AvroDecode for i16 { fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> { i16::try_from(i32::avro_decode(buffer, offset)?).map_err(|_| "Decoded value does not fit i16") } }
impl NotBytesEncode for i16 {}
impl NotBytesDecode for i16 {}

impl AvroEncode for u16 { fn avro_encode(&self, buffer: &mut Vec<u8>) { (*self as i32).avro_encode(buffer); } }
impl AvroDecode for u16 { fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> { u16::try_from(i32::avro_decode(buffer, offset)?).map_err(|_| "Decoded value does not fit u16") } }
impl NotBytesEncode for u16 {}
impl NotBytesDecode for u16 {}

impl AvroEncode for u32 { fn avro_encode(&self, buffer: &mut Vec<u8>) { (*self as i64).avro_encode(buffer); } }
impl AvroDecode for u32 { fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> { u32::try_from(i64::avro_decode(buffer, offset)?).map_err(|_| "Decoded value does not fit u32") } }
impl NotBytesEncode for u32 {}
impl NotBytesDecode for u32 {}

impl AvroEncode for u64 { fn avro_encode(&self, buffer: &mut Vec<u8>) { (*self as i64).avro_encode(buffer); } }
impl AvroDecode for u64 { fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> { u64::try_from(i64::avro_decode(buffer, offset)?).map_err(|_| "Decoded value does not fit u64") } }
impl NotBytesEncode for u64 {}
impl NotBytesDecode for u64 {}

impl AvroEncode for Vec<i8> {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        let len = self.len() as i64;
        len.avro_encode(buffer);
        for &b in self {
            buffer.push(b as u8);
        }
    }
}


impl AvroDecode for Vec<i8> {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let len = i64::avro_decode(buffer, offset)?;
        if len < 0 { return Err("Negative length for bytes"); }
        let len = usize::try_from(len).map_err(|_| "Bytes length is too large")?;
        let bytes = take_bytes(buffer, offset, len, "EOF while reading bytes")?;
        Ok(bytes.iter().map(|&b| b as i8).collect())
    }
}


impl AvroEncode for Vec<u8> {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        let len = self.len() as i64;
        len.avro_encode(buffer);
        buffer.extend_from_slice(self);
    }
}


impl AvroDecode for Vec<u8> {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let len = i64::avro_decode(buffer, offset)?;
        if len < 0 { return Err("Negative length for bytes"); }
        let len = usize::try_from(len).map_err(|_| "Bytes length is too large")?;
        let bytes = take_bytes(buffer, offset, len, "EOF while reading bytes")?;
        Ok(bytes.to_vec())
    }
}


impl<T: AvroEncode + NotBytesEncode> AvroEncode for Vec<T> {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        if !self.is_empty() {
            let len = self.len() as i64;
            len.avro_encode(buffer);
            for item in self {
                item.avro_encode(buffer);
            }
        }
        0i64.avro_encode(buffer);
    }
}


impl<T: AvroDecode + NotBytesDecode> AvroDecode for Vec<T> {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let mut result = Vec::new();
        loop {
            let block_count = i64::avro_decode(buffer, offset)?;
            if block_count == 0 {
                break;
            }
            let (count, block_end) = if block_count < 0 {
                let count = block_count.checked_neg()
                    .ok_or("Invalid Avro array block count")?;
                let byte_count = i64::avro_decode(buffer, offset)?;
                if byte_count < 0 {
                    return Err("Negative Avro array block size");
                }
                let byte_count = usize::try_from(byte_count)
                    .map_err(|_| "Avro array block is too large")?;
                let block_end = offset.checked_add(byte_count)
                    .filter(|end| *end <= buffer.len())
                    .ok_or("EOF while reading Avro array block")?;
                (count, Some(block_end))
            } else {
                (block_count, None)
            };
            let count = usize::try_from(count)
                .map_err(|_| "Avro array item count is too large")?;
            result.try_reserve(count)
                .map_err(|_| "Avro array item count is too large")?;
            for _ in 0..count {
                let item_offset = *offset;
                let item = T::avro_decode(buffer, offset)?;
                if *offset <= item_offset {
                    return Err("Avro array item consumed no bytes");
                }
                if block_end.is_some_and(|end| *offset > end) {
                    return Err("Avro array item exceeds its block");
                }
                result.push(item);
            }
            if block_end.is_some_and(|end| *offset != end) {
                return Err("Avro array block size mismatch");
            }
        }
        Ok(result)
    }
}


impl<T: AvroEncode> AvroEncode for Option<T> {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        match self {
            None => {
                0i32.avro_encode(buffer);
            }
            Some(v) => {
                1i32.avro_encode(buffer);
                v.avro_encode(buffer);
            }
        }
    }
}


impl<T: AvroDecode> AvroDecode for Option<T> {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        let index = i32::avro_decode(buffer, offset)?;
        match index {
            0 => Ok(None),
            1 => Ok(Some(T::avro_decode(buffer, offset)?)),
            _ => Err("Invalid Option union index"),
        }
    }
}

impl<T: AvroEncode> AvroEncode for Box<T> {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.as_ref().avro_encode(buffer);
    }
}

impl<T: AvroDecode> AvroDecode for Box<T> {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok(Box::new(T::avro_decode(buffer, offset)?))
    }
}

impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn malformed_lengths_and_array_blocks_return_errors_without_panicking() {
        let mut offset = usize::MAX;
        assert!(f64::avro_decode(&[], &mut offset).is_err());

        let mut minimum_block = Vec::new();
        i64::MIN.avro_encode(&mut minimum_block);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut offset = 0;
            Vec::<bool>::avro_decode(&minimum_block, &mut offset)
        }));
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());

        let mut mismatched_block = Vec::new();
        (-1i64).avro_encode(&mut mismatched_block);
        2i64.avro_encode(&mut mismatched_block);
        true.avro_encode(&mut mismatched_block);
        0i64.avro_encode(&mut mismatched_block);
        let mut offset = 0;
        assert!(Vec::<bool>::avro_decode(&mismatched_block, &mut offset).is_err());

        let mut impossible_count = Vec::new();
        i64::MAX.avro_encode(&mut impossible_count);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut offset = 0;
            Vec::<bool>::avro_decode(&impossible_count, &mut offset)
        }));
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());

        let mut negative_unsigned = Vec::new();
        (-1i32).avro_encode(&mut negative_unsigned);
        let mut offset = 0;
        assert!(u8::avro_decode(&negative_unsigned, &mut offset).is_err());

        let mut overlong_i32 = [0x80; 6];
        overlong_i32[5] = 0;
        let mut offset = 0;
        assert!(i32::avro_decode(&overlong_i32, &mut offset).is_err());
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode, T: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
        self.19.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode, T: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
            T::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode, T: AvroEncode, U: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
        self.19.avro_encode(buffer);
        self.20.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode, T: AvroDecode, U: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
            T::avro_decode(buffer, offset)?,
            U::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode, T: AvroEncode, U: AvroEncode, V: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
        self.19.avro_encode(buffer);
        self.20.avro_encode(buffer);
        self.21.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode, T: AvroDecode, U: AvroDecode, V: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
            T::avro_decode(buffer, offset)?,
            U::avro_decode(buffer, offset)?,
            V::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode, T: AvroEncode, U: AvroEncode, V: AvroEncode, W: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
        self.19.avro_encode(buffer);
        self.20.avro_encode(buffer);
        self.21.avro_encode(buffer);
        self.22.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode, T: AvroDecode, U: AvroDecode, V: AvroDecode, W: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
            T::avro_decode(buffer, offset)?,
            U::avro_decode(buffer, offset)?,
            V::avro_decode(buffer, offset)?,
            W::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode, T: AvroEncode, U: AvroEncode, V: AvroEncode, W: AvroEncode, X: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
        self.19.avro_encode(buffer);
        self.20.avro_encode(buffer);
        self.21.avro_encode(buffer);
        self.22.avro_encode(buffer);
        self.23.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode, T: AvroDecode, U: AvroDecode, V: AvroDecode, W: AvroDecode, X: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
            T::avro_decode(buffer, offset)?,
            U::avro_decode(buffer, offset)?,
            V::avro_decode(buffer, offset)?,
            W::avro_decode(buffer, offset)?,
            X::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode, T: AvroEncode, U: AvroEncode, V: AvroEncode, W: AvroEncode, X: AvroEncode, Y: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
        self.19.avro_encode(buffer);
        self.20.avro_encode(buffer);
        self.21.avro_encode(buffer);
        self.22.avro_encode(buffer);
        self.23.avro_encode(buffer);
        self.24.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode, T: AvroDecode, U: AvroDecode, V: AvroDecode, W: AvroDecode, X: AvroDecode, Y: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
            T::avro_decode(buffer, offset)?,
            U::avro_decode(buffer, offset)?,
            V::avro_decode(buffer, offset)?,
            W::avro_decode(buffer, offset)?,
            X::avro_decode(buffer, offset)?,
            Y::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,) {}


impl<A: AvroEncode, B: AvroEncode, C: AvroEncode, D: AvroEncode, E: AvroEncode, F: AvroEncode, G: AvroEncode, H: AvroEncode, I: AvroEncode, J: AvroEncode, K: AvroEncode, L: AvroEncode, M: AvroEncode, N: AvroEncode, O: AvroEncode, P: AvroEncode, Q: AvroEncode, R: AvroEncode, S: AvroEncode, T: AvroEncode, U: AvroEncode, V: AvroEncode, W: AvroEncode, X: AvroEncode, Y: AvroEncode, Z: AvroEncode> AvroEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,) {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        self.0.avro_encode(buffer);
        self.1.avro_encode(buffer);
        self.2.avro_encode(buffer);
        self.3.avro_encode(buffer);
        self.4.avro_encode(buffer);
        self.5.avro_encode(buffer);
        self.6.avro_encode(buffer);
        self.7.avro_encode(buffer);
        self.8.avro_encode(buffer);
        self.9.avro_encode(buffer);
        self.10.avro_encode(buffer);
        self.11.avro_encode(buffer);
        self.12.avro_encode(buffer);
        self.13.avro_encode(buffer);
        self.14.avro_encode(buffer);
        self.15.avro_encode(buffer);
        self.16.avro_encode(buffer);
        self.17.avro_encode(buffer);
        self.18.avro_encode(buffer);
        self.19.avro_encode(buffer);
        self.20.avro_encode(buffer);
        self.21.avro_encode(buffer);
        self.22.avro_encode(buffer);
        self.23.avro_encode(buffer);
        self.24.avro_encode(buffer);
        self.25.avro_encode(buffer);
    }
}



impl<A: AvroDecode, B: AvroDecode, C: AvroDecode, D: AvroDecode, E: AvroDecode, F: AvroDecode, G: AvroDecode, H: AvroDecode, I: AvroDecode, J: AvroDecode, K: AvroDecode, L: AvroDecode, M: AvroDecode, N: AvroDecode, O: AvroDecode, P: AvroDecode, Q: AvroDecode, R: AvroDecode, S: AvroDecode, T: AvroDecode, U: AvroDecode, V: AvroDecode, W: AvroDecode, X: AvroDecode, Y: AvroDecode, Z: AvroDecode> AvroDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,) {
    fn avro_decode(buffer: &[u8], offset: &mut usize) -> Result<Self, &'static str> {
        Ok((
            A::avro_decode(buffer, offset)?,
            B::avro_decode(buffer, offset)?,
            C::avro_decode(buffer, offset)?,
            D::avro_decode(buffer, offset)?,
            E::avro_decode(buffer, offset)?,
            F::avro_decode(buffer, offset)?,
            G::avro_decode(buffer, offset)?,
            H::avro_decode(buffer, offset)?,
            I::avro_decode(buffer, offset)?,
            J::avro_decode(buffer, offset)?,
            K::avro_decode(buffer, offset)?,
            L::avro_decode(buffer, offset)?,
            M::avro_decode(buffer, offset)?,
            N::avro_decode(buffer, offset)?,
            O::avro_decode(buffer, offset)?,
            P::avro_decode(buffer, offset)?,
            Q::avro_decode(buffer, offset)?,
            R::avro_decode(buffer, offset)?,
            S::avro_decode(buffer, offset)?,
            T::avro_decode(buffer, offset)?,
            U::avro_decode(buffer, offset)?,
            V::avro_decode(buffer, offset)?,
            W::avro_decode(buffer, offset)?,
            X::avro_decode(buffer, offset)?,
            Y::avro_decode(buffer, offset)?,
            Z::avro_decode(buffer, offset)?,
        ))
    }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> NotBytesEncode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,) {}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> NotBytesDecode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,) {}
