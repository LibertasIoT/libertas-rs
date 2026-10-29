use alloc::string::{String, ToString};
use core::fmt::{self, Write};

/// A Request-rooted ErrorFieldPath or RevealPath locator.
/// Key steps identify members of keyed Lists; indices identify their current position.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaPath(String);

/// A String or integer List key. Floating-point keys are not supported.
pub enum SchemaPathKey {
    String(String),
    Signed(i64),
    Unsigned(u64),
}
impl From<&str> for SchemaPathKey {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
impl From<String> for SchemaPathKey {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
macro_rules! integer_keys {
    ($variant:ident, $target:ty, $($ty:ty),+) => { $(
        impl From<$ty> for SchemaPathKey {
            fn from(value: $ty) -> Self { Self::$variant(value as $target) }
        }
    )+ };
}
integer_keys!(Signed, i64, i8, i16, i32, i64);
integer_keys!(Unsigned, u64, u8, u16, u32, u64);

impl SchemaPath {
    pub fn root() -> Self {
        Self::default()
    }

    /// Append a native schema field name.
    pub fn field(mut self, name: &str) -> Self {
        let mut chars = name.chars();
        assert!(
            chars
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
                && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$'),
            "invalid schema field name"
        );
        self.separator();
        self.0.push_str(name);
        self
    }

    pub fn index(mut self, index: usize) -> Self {
        self.separator();
        let _ = write!(self.0, "{index}");
        self
    }

    pub fn key(mut self, key: impl Into<SchemaPathKey>) -> Self {
        self.separator();
        self.0.push('=');
        match key.into() {
            SchemaPathKey::String(value) => {
                self.0.push('"');
                for c in value.chars() {
                    match c {
                        '"' => self.0.push_str("\\\""),
                        '\\' => self.0.push_str("\\\\"),
                        '\n' => self.0.push_str("\\n"),
                        '\r' => self.0.push_str("\\r"),
                        '\t' => self.0.push_str("\\t"),
                        c if c < '\u{20}' => {
                            let _ = write!(self.0, "\\u{:04x}", c as u32);
                        }
                        c => self.0.push(c),
                    }
                }
                self.0.push('"');
            }
            SchemaPathKey::Signed(value) => {
                let _ = write!(self.0, "{value}");
            }
            SchemaPathKey::Unsigned(value) => {
                let _ = write!(self.0, "{value}");
            }
        }
        self
    }

    fn separator(&mut self) {
        if !self.0.is_empty() {
            self.0.push('/');
        }
    }
}
impl fmt::Display for SchemaPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
impl From<SchemaPath> for String {
    fn from(value: SchemaPath) -> Self {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_position_and_integer_key_are_distinct() {
        assert_eq!(SchemaPath::root().to_string(), "[]");
        assert_eq!(
            SchemaPath::root()
                .field("rows")
                .index(42)
                .field("answer")
                .to_string(),
            "[rows/42/answer]"
        );
        assert_eq!(
            SchemaPath::root()
                .field("rows")
                .key(42i32)
                .field("answer")
                .to_string(),
            "[rows/=42/answer]"
        );
        assert_eq!(
            SchemaPath::root().field("rows").key(i64::MIN).to_string(),
            "[rows/=-9223372036854775808]"
        );
        assert_eq!(
            SchemaPath::root().key(u64::MAX).to_string(),
            "[=18446744073709551615]"
        );
    }

    #[test]
    fn string_keys_use_json_escaping() {
        assert_eq!(
            SchemaPath::root()
                .field("rows")
                .key("a\"/b]\\雪\n\t\u{0}")
                .field("answer")
                .to_string(),
            "[rows/=\"a\\\"/b]\\\\雪\\n\\t\\u0000\"/answer]"
        );
        assert_eq!(SchemaPath::root().key("").to_string(), "[=\"\"]");
    }
}
