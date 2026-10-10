//! Version 1 of the shared Avro storage command protocol.
//! Keys are ordered codec bytes inside Avro Bytes; values are Avro union bytes.
use crate::{AvroDecode, AvroEncode};
use alloc::vec::Vec;
use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode, VariantIndex};

pub const VERSION: u32 = 1;
pub const MAX_KEY_BYTES: usize = 4096;
pub const MAX_VALUE_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_REQUEST_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_ITEMS: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub enum Target {
    Keys(Vec<Vec<u8>>),
    Prefix(Vec<u8>),
}
impl Target {
    pub fn validate(&self) -> bool {
        match self {
            Self::Keys(keys) => {
                !keys.is_empty()
                    && keys.len() <= MAX_ITEMS
                    && keys.iter().all(|k| (2..=MAX_KEY_BYTES).contains(&k.len()))
                    && keys.windows(2).all(|p| p[0] < p[1])
            }
            Self::Prefix(prefix) => prefix.len() <= MAX_KEY_BYTES,
        }
    }
    pub fn contains(&self, key: &[u8]) -> bool {
        match self {
            Self::Keys(keys) => keys.binary_search_by(|k| k.as_slice().cmp(key)).is_ok(),
            Self::Prefix(prefix) => key.starts_with(prefix),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub enum Mode {
    ReadOnly,
    ReadWrite,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub enum Direction {
    Ascending,
    Descending,
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub enum Bound {
    Unbounded,
    Included(Vec<u8>),
    Excluded(Vec<u8>),
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub struct Range {
    pub start: Bound,
    pub end: Bound,
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub struct Scan {
    pub range: Range,
    pub direction: Direction,
    pub limit: u32,
    pub continuation: Option<Vec<u8>>,
    pub keys_only: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode, VariantIndex)]
pub enum Mutation {
    Put { key: Vec<u8>, value: Vec<u8> },
    Delete { key: Vec<u8> },
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode, VariantIndex)]
pub enum Operation {
    Get(Vec<u8>),
    GetMany(Vec<Vec<u8>>),
    Exists(Vec<u8>),
    Scan(Scan),
    Put { key: Vec<u8>, value: Vec<u8> },
    Delete(Vec<u8>),
    WriteBatch(Vec<Mutation>),
}
impl Operation {
    pub fn writes(&self) -> bool {
        matches!(
            self,
            Self::Put { .. } | Self::Delete(_) | Self::WriteBatch(_)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode, VariantIndex)]
pub enum Command {
    Begin {
        target: Target,
        mode: Mode,
    },
    Execute {
        target: Target,
        operation: Operation,
    },
    ExecuteInTransaction {
        transaction: u64,
        operation: Operation,
    },
    Commit(u64),
    Rollback(u64),
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub struct Request {
    pub version: u32,
    pub id: u64,
    pub checked: bool,
    pub command: Command,
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub struct Entry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub enum Result {
    Begun(u64),
    Value(Option<Vec<u8>>),
    Values(Vec<Option<Vec<u8>>>),
    Exists(bool),
    Entries {
        items: Vec<Entry>,
        continuation: Option<Vec<u8>>,
    },
    Keys {
        items: Vec<Vec<u8>>,
        continuation: Option<Vec<u8>>,
    },
    Done,
    Failure(u32),
}
#[derive(Debug, Clone, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
pub struct Response {
    pub id: u64,
    pub result: Result,
}

pub fn encode<T: AvroEncode>(value: &T) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.avro_encode(&mut bytes);
    bytes
}
pub fn decode<T: AvroDecode>(bytes: &[u8]) -> core::result::Result<T, &'static str> {
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err("Storage message exceeds limit");
    }
    let mut offset = 0;
    let value = T::avro_decode(bytes, &mut offset)?;
    if offset != bytes.len() {
        return Err("Trailing storage message bytes");
    }
    Ok(value)
}

/// Validate frame lengths and collection counts before allocating decoded data.
/// This is the host entry point for App-supplied storage messages.
pub fn decode_request(bytes: &[u8]) -> core::result::Result<Request, &'static str> {
    struct Frame<'a> {
        bytes: &'a [u8],
        offset: usize,
    }
    impl Frame<'_> {
        fn number(&mut self) -> core::result::Result<i64, &'static str> {
            i64::avro_decode(self.bytes, &mut self.offset)
        }
        fn tag(&mut self, variants: i64) -> core::result::Result<i64, &'static str> {
            let n = self.number()?;
            if n < 0 || n >= variants {
                return Err("Invalid storage union tag");
            }
            Ok(n)
        }
        fn boolean(&mut self) -> core::result::Result<(), &'static str> {
            bool::avro_decode(self.bytes, &mut self.offset).map(|_| ())
        }
        fn bytes(&mut self, max: usize) -> core::result::Result<(), &'static str> {
            let len = usize::try_from(self.number()?).map_err(|_| "Invalid byte length")?;
            if len > max {
                return Err("Storage byte length exceeds limit");
            }
            self.offset = self
                .offset
                .checked_add(len)
                .filter(|n| *n <= self.bytes.len())
                .ok_or("Truncated storage bytes")?;
            Ok(())
        }
        fn array(
            &mut self,
            mut item: impl FnMut(&mut Self) -> core::result::Result<(), &'static str>,
        ) -> core::result::Result<(), &'static str> {
            let mut total = 0usize;
            loop {
                let n = self.number()?;
                if n == 0 {
                    return Ok(());
                }
                let count = usize::try_from(n.checked_abs().ok_or("Invalid storage array count")?)
                    .map_err(|_| "Invalid storage array count")?;
                total = total
                    .checked_add(count)
                    .filter(|n| *n <= MAX_ITEMS)
                    .ok_or("Storage item limit")?;
                let end = if n < 0 {
                    let size =
                        usize::try_from(self.number()?).map_err(|_| "Invalid block length")?;
                    Some(
                        self.offset
                            .checked_add(size)
                            .filter(|n| *n <= self.bytes.len())
                            .ok_or("Truncated array block")?,
                    )
                } else {
                    None
                };
                for _ in 0..count {
                    item(self)?;
                }
                if end.is_some_and(|end| end != self.offset) {
                    return Err("Invalid storage array block");
                }
            }
        }
        fn target(&mut self) -> core::result::Result<(), &'static str> {
            if self.tag(2)? == 0 {
                self.array(|s| s.bytes(MAX_KEY_BYTES))
            } else {
                self.bytes(MAX_KEY_BYTES)
            }
        }
        fn operation(&mut self) -> core::result::Result<(), &'static str> {
            match self.tag(7)? {
                0 | 2 | 5 => self.bytes(MAX_KEY_BYTES),
                1 => self.array(|s| s.bytes(MAX_KEY_BYTES)),
                3 => {
                    for _ in 0..2 {
                        if self.tag(3)? != 0 {
                            self.bytes(MAX_KEY_BYTES)?;
                        }
                    }
                    self.tag(2)?;
                    let limit = self.number()?;
                    if limit <= 0 || limit > MAX_ITEMS as i64 {
                        return Err("Scan item limit");
                    }
                    if self.tag(2)? == 1 {
                        self.bytes(MAX_REQUEST_BYTES)?;
                    }
                    self.boolean()
                }
                4 => {
                    self.bytes(MAX_KEY_BYTES)?;
                    self.bytes(MAX_VALUE_BYTES)
                }
                6 => self.array(|s| {
                    let tag = s.tag(2)?;
                    s.bytes(MAX_KEY_BYTES)?;
                    if tag == 0 {
                        s.bytes(MAX_VALUE_BYTES)?;
                    }
                    Ok(())
                }),
                _ => unreachable!(),
            }
        }
    }
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err("Storage frame limit");
    }
    let mut frame = Frame { bytes, offset: 0 };
    frame.number()?;
    frame.number()?;
    frame.boolean()?;
    match frame.tag(5)? {
        0 => {
            frame.target()?;
            frame.tag(2)?;
        }
        1 => {
            frame.target()?;
            frame.operation()?;
        }
        2 => {
            frame.number()?;
            frame.operation()?;
        }
        3 | 4 => {
            frame.number()?;
        }
        _ => unreachable!(),
    }
    if frame.offset != bytes.len() {
        return Err("Trailing storage frame bytes");
    }
    decode(bytes)
}
