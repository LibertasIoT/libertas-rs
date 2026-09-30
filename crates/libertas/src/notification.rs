use crate::*;
use alloc::string::String;
/// Libertas Rust SDK - Notification API
///
/// Provides functions for sending notifications with severity levels and arguments.
use alloc::vec::Vec;
use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode};

/// Borrowed canonical FormattedText bytes encoded as Avro Bytes, not an array of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LibertasFormattedTextArgument<'a>(&'a [u8]);

impl<'a> From<&'a [u8]> for LibertasFormattedTextArgument<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(value)
    }
}

impl AvroEncode for LibertasFormattedTextArgument<'_> {
    fn avro_encode(&self, buffer: &mut Vec<u8>) {
        (self.0.len() as i64).avro_encode(buffer);
        buffer.extend_from_slice(self.0);
    }
}

/// Notification severity levels.
/// - `Debug`: Debugging info.
/// - `Info`: General info.
/// - `AlertLow`: Low-priority alerts.
/// - `AlertGuarded`: Guarded alerts.
/// - `AlertElevated`: Elevated alerts.
/// - `AlertHigh`: High alerts.
/// - `AlertSevere`: Severe alerts.
#[repr(u8)]
#[derive(LibertasAvroEncode, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NotificationImportance {
    Debug,
    Info,
    AlertLow,
    AlertGuarded,
    AlertElevated,
    AlertHigh,
    AlertSevere,
}

/// A typed argument for Libertas Message Format (LMF) templates.
///
/// The variant is encoded with the value so clients can give the encoded
/// semantic type priority over a conflicting LMF1 template type.
#[derive(LibertasAvroEncode, Debug, Clone, Copy)]
pub enum LibertasMessageArgument<'a> {
    LiteralText(&'a str),
    Object(LibertasId),
    Boolean(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f32),
    Double(f64),
    UnitSigned {
        unit_type: &'a str,
        value: i64,
    },
    UnitUnsigned {
        unit_type: &'a str,
        value: u64,
    },
    UnitFloat {
        unit_type: &'a str,
        value: f32,
    },
    UnitDouble {
        unit_type: &'a str,
        value: f64,
    },
    ResourceText(&'a str),
    Plural(i64),
    /// A complete canonical FormattedText tuple, rendered in this message's locale.
    FormattedText(LibertasFormattedTextArgument<'a>),
    /// Exact fixed-point currency amount. `minor_units` is scaled by 10^`scale`.
    Currency {
        currency_code: &'a str,
        scale: i64,
        minor_units: i64,
    },
    /// Gregorian calendar date in `yyyyMMdd` form, without a timezone.
    DateOnly(i64),
    /// Unix epoch seconds, displayed in the renderer's local timezone.
    DateTime(u64),
    /// Clock seconds from local midnight; 86400 denotes the end of the day.
    TimeOnly(u32),
}

impl<'a> LibertasMessageArgument<'a> {
    /// Uses a canonical tuple returned by [`libertas_formatted_text`] as one argument.
    pub fn formatted_text(value: &'a [u8]) -> Self {
        Self::FormattedText(value.into())
    }
}

/// An owned, decoded Libertas Message Argument that can be compared with `LibertasMessageArgument`. This type decodes Avro-encoded arguments and uses owned values (for example, `String` instead of `&str`). Each variant corresponds to the same variant in `LibertasMessageArgument`; `PartialEq` supports encoding tests and wire verification.
///
/// The message is rendered with Libertas Message Format 1 (LMF1), the custom Libertas extension of ICU MessageFormat 1. The `LibertasMessageArgument` enum allows for a wide range of argument types,
/// including literal text, resource text, objects, boolean values, numerical values, and unit values with specific types. By using this enum, developers can create messages that are rich in content and can convey
/// a wide range of information to users or other parts of the system when sending notifications through the Libertas system.
///
/// Note that the `LibertasMessageArgumentDecode` enum is designed to be used in conjunction with the Avro decoding process,
/// where you would decode a message's arguments from Avro format into instances of `LibertasMessageArgumentDecode`,
/// and then compare those decoded arguments with the original `LibertasMessageArgument` values to ensure they match as expected.
///
/// This can help verify that the encoding and decoding processes are working correctly and that the arguments are being transmitted
/// accurately through the Libertas system.
///
/// For example, if you have a message template that includes certain arguments, you can encode those arguments using
/// `LibertasMessageArgument`, send the message through the Libertas system, and then decode the arguments on the receiving end
/// into `LibertasMessageArgumentDecode` instances. You can then compare the decoded arguments with the original `LibertasMessageArgument`
/// values to confirm that they were transmitted correctly and that the message content is accurate based on the provided arguments.
///
/// It is also used as argument in a name template to provide predictible yet flexible name for database name and data field, making
/// Libertas Apps fully self-describing.
///
/// The `LibertasMessageArgumentDecode` enum includes the following variants:
/// - `LiteralText`: A variant that represents a literal string argument, which is a string that is included in the message as-is, without being treated as a resource string. This can be used to include specific text in the message that does not need to be localized or looked up from resources.
/// - `Object`: A variant that represents a Libertas system object, such as a device, user, or other entity within the system. When this variant is used, the argument is expected to be a 32-bit unsigned integer that serves as an identifier for the object. The application task must have access permission to the object in order to include it as an argument in the message. This allows messages to reference specific objects within the system, providing context and information about those objects in the message content.
/// - `Boolean`: A variant that represents a boolean value (true or false) that can be included as an argument in a message sent through the Libertas system. This allows messages to convey binary information or flags that can be used to indicate conditions, toggle features, or provide simple true/false values in the context of the message being sent.
/// - `Signed`: A variant that represents a signed integer value that can be included as an argument in a message sent through the Libertas system. This allows messages to convey numerical information such as counts, measurements, or other integer-based data that can be positive or negative in the context of the message being sent.
/// - `Unsigned`: A variant that represents an unsigned integer value that can be included as an argument in a message sent through the Libertas system. This allows messages to convey numerical information that is non-negative, such as counts, sizes, or other unsigned integer-based data in the context of the message being sent.
/// - `Float`: A variant that represents a floating-point value that can be included as an argument in a message sent through the Libertas system. This allows messages to convey numerical information that requires decimal precision, such as measurements, percentages, or other float-based data in the context of the message being sent.
/// - `Double`: A variant that represents a double-precision floating-point value that can be included as an argument in a message sent through the Libertas system. This allows messages to convey numerical information that requires higher precision than a standard float, such as scientific measurements, financial data, or other double-based data in the context of the message being sent.
/// - `UnitSigned`: A variant that represents a unit value with a specific type that can be included as an argument in a message sent through the Libertas system. This allows messages to convey measurements or other data that has a specific unit of measurement, such as temperature readings with units (e.g., "Celsius" or "Fahrenheit"), distance measurements with units (e.g., "meters" or "miles"), or any other relevant value that is associated with a specific unit type in the context of the message being sent.
/// - `UnitUnsigned`: A variant that represents a unit value with a specific type that can be included as an argument in a message sent through the Libertas system. This allows messages to convey measurements or other data that has a specific unit of measurement, such as file sizes with units (e.g., "KB" or "MB"), speed measurements with units (e.g., "km/h" or "mph"), or any other relevant value that is associated with a specific unit type in the context of the message being sent.
/// - `UnitFloat`: A variant that represents a unit value with a specific type that can be included as an argument in a message sent through the Libertas system. This allows messages to convey measurements or other data that has a specific unit of measurement, such as precise temperature readings with units (e.g., "Celsius" or "Fahrenheit"), financial amounts with units (e.g., "USD" or "EUR"), or any other relevant value that is associated with a specific unit type in the context of the message being sent.
/// - `UnitDouble`: A variant that represents a unit value with a specific type that can be included as an argument in a message sent through the Libertas system. This allows messages to convey measurements or other data that has a specific unit of measurement, such as precise temperature readings with units (e.g., "Celsius" or "Fahrenheit"), financial amounts with units (e.g., "USD" or "EUR"), or any other relevant value that is associated with a specific unit type in the context of the message being sent.
/// - `ResourceText`: A variant that represents a resource string argument, which is a string that is included in the message as a reference to a resource string rather than as a literal string. This allows messages to include text that can be localized or looked up from resources based on the user's language or other context, providing more flexibility and internationalization support for the message content.
/// - `Plural`: A plural value that can be included as an argument in a message sent through the Libertas system. This variant of the `LibertasMessageArgument` enum allows you to include a numerical value that represents a count for pluralization purposes in the context of the message being sent. The `Plural` variant is typically used in conjunction with resource string templates that have plural forms, where the value provided in this argument will determine which plural form of the resource string to use when generating the final message. For example, if you have a resource string that has different forms for singular and plural (e.g., "You have {0} new message" vs. "You have {0} new messages"), you would use the `Plural` argument to provide the count of new messages, and the system would automatically certain word of the resource string based on that count.
#[derive(LibertasAvroEncode, LibertasAvroDecode, PartialEq, Debug, Clone)]
pub enum LibertasMessageArgumentDecode {
    LiteralText(String),
    Object(LibertasId),
    Boolean(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f32),
    Double(f64),
    UnitSigned {
        unit_type: String,
        value: i64,
    },
    UnitUnsigned {
        unit_type: String,
        value: u64,
    },
    UnitFloat {
        unit_type: String,
        value: f32,
    },
    UnitDouble {
        unit_type: String,
        value: f64,
    },
    ResourceText(String),
    Plural(i64),
    FormattedText(Vec<u8>),
    Currency {
        currency_code: String,
        scale: i64,
        minor_units: i64,
    },
    /// Gregorian calendar date in `yyyyMMdd` form, without a timezone.
    DateOnly(i64),
    /// Unix epoch seconds, displayed in the renderer's local timezone.
    DateTime(u64),
    /// Clock seconds from local midnight; 86400 denotes the end of the day.
    TimeOnly(u32),
}

impl LibertasMessageArgumentDecode {
    /// Borrows an owned decoded argument for APIs that accept message arguments.
    /// This preserves the variant and value without allocating or re-encoding.
    pub fn as_message_argument(&self) -> LibertasMessageArgument<'_> {
        use LibertasMessageArgument as A;
        match self {
            Self::LiteralText(text) => A::LiteralText(text),
            Self::Object(id) => A::Object(*id),
            Self::Boolean(value) => A::Boolean(*value),
            Self::Signed(value) => A::Signed(*value),
            Self::Unsigned(value) => A::Unsigned(*value),
            Self::Float(value) => A::Float(*value),
            Self::Double(value) => A::Double(*value),
            Self::UnitSigned { unit_type, value } => A::UnitSigned {
                unit_type,
                value: *value,
            },
            Self::UnitUnsigned { unit_type, value } => A::UnitUnsigned {
                unit_type,
                value: *value,
            },
            Self::UnitFloat { unit_type, value } => A::UnitFloat {
                unit_type,
                value: *value,
            },
            Self::UnitDouble { unit_type, value } => A::UnitDouble {
                unit_type,
                value: *value,
            },
            Self::ResourceText(key) => A::ResourceText(key),
            Self::Plural(value) => A::Plural(*value),
            Self::FormattedText(value) => A::formatted_text(value),
            Self::DateOnly(value) => A::DateOnly(*value),
            Self::DateTime(value) => A::DateTime(*value),
            Self::TimeOnly(value) => A::TimeOnly(*value),
            Self::Currency {
                currency_code,
                scale,
                minor_units,
            } => A::Currency {
                currency_code,
                scale: *scale,
                minor_units: *minor_units,
            },
        }
    }
}

impl PartialEq<LibertasMessageArgument<'_>> for LibertasMessageArgumentDecode {
    fn eq(&self, other: &LibertasMessageArgument<'_>) -> bool {
        match (self, other) {
            (
                LibertasMessageArgumentDecode::LiteralText(s1),
                LibertasMessageArgument::LiteralText(s2),
            ) => s1 == s2,
            (LibertasMessageArgumentDecode::Object(o1), LibertasMessageArgument::Object(o2)) => {
                o1 == o2
            }
            (LibertasMessageArgumentDecode::Boolean(b1), LibertasMessageArgument::Boolean(b2)) => {
                b1 == b2
            }
            (LibertasMessageArgumentDecode::Signed(i1), LibertasMessageArgument::Signed(i2)) => {
                i1 == i2
            }
            (
                LibertasMessageArgumentDecode::Unsigned(u1),
                LibertasMessageArgument::Unsigned(u2),
            ) => u1 == u2,
            (LibertasMessageArgumentDecode::Float(f1), LibertasMessageArgument::Float(f2)) => {
                f1 == f2
            }
            (LibertasMessageArgumentDecode::Double(d1), LibertasMessageArgument::Double(d2)) => {
                d1 == d2
            }
            (
                LibertasMessageArgumentDecode::UnitSigned {
                    unit_type: ut1,
                    value: v1,
                },
                LibertasMessageArgument::UnitSigned {
                    unit_type: ut2,
                    value: v2,
                },
            ) => ut1 == ut2 && v1 == v2,
            (
                LibertasMessageArgumentDecode::UnitUnsigned {
                    unit_type: ut1,
                    value: v1,
                },
                LibertasMessageArgument::UnitUnsigned {
                    unit_type: ut2,
                    value: v2,
                },
            ) => ut1 == ut2 && v1 == v2,
            (
                LibertasMessageArgumentDecode::UnitFloat {
                    unit_type: ut1,
                    value: v1,
                },
                LibertasMessageArgument::UnitFloat {
                    unit_type: ut2,
                    value: v2,
                },
            ) => ut1 == ut2 && v1 == v2,
            (
                LibertasMessageArgumentDecode::UnitDouble {
                    unit_type: ut1,
                    value: v1,
                },
                LibertasMessageArgument::UnitDouble {
                    unit_type: ut2,
                    value: v2,
                },
            ) => ut1 == ut2 && v1 == v2,
            (
                LibertasMessageArgumentDecode::ResourceText(s1),
                LibertasMessageArgument::ResourceText(s2),
            ) => s1 == s2,
            (LibertasMessageArgumentDecode::Plural(p1), LibertasMessageArgument::Plural(p2)) => {
                p1 == p2
            }
            (
                LibertasMessageArgumentDecode::FormattedText(value),
                LibertasMessageArgument::FormattedText(other),
            ) => value.as_slice() == other.0,
            (
                LibertasMessageArgumentDecode::Currency {
                    currency_code: code1,
                    scale: scale1,
                    minor_units: value1,
                },
                LibertasMessageArgument::Currency {
                    currency_code: code2,
                    scale: scale2,
                    minor_units: value2,
                },
            ) => code1 == code2 && scale1 == scale2 && value1 == value2,
            (Self::DateOnly(value), LibertasMessageArgument::DateOnly(other)) => value == other,
            (Self::DateTime(value), LibertasMessageArgument::DateTime(other)) => value == other,
            (Self::TimeOnly(value), LibertasMessageArgument::TimeOnly(other)) => value == other,
            _ => false,
        }
    }
}

#[derive(LibertasAvroEncode)]
struct FormattedTextRef<'a> {
    resource_name: &'a str,
    arguments: &'a [LibertasMessageArgument<'a>],
}

/// Encodes localized formatted text for a FormattedText runtime byte array.
///
/// The returned bytes are an Avro record containing the string-resource name
/// followed by the same typed Libertas Message Argument array used by Libertas
/// notifications. Runtime clients localize and render the value; applications
/// must not Base64-wrap it when the schema host is a byte array.
pub fn libertas_formatted_text(
    resource_name: &str,
    arguments: &[LibertasMessageArgument<'_>],
) -> Vec<u8> {
    let value = FormattedTextRef {
        resource_name,
        arguments,
    };
    let mut encoded = Vec::new();
    value.avro_encode(&mut encoded);
    encoded
}

#[derive(LibertasAvroEncode)]
struct LibertasNotification<'a> {
    level: NotificationImportance,
    source: LibertasId,
    resource_name: &'a str,
    arguments: &'a [LibertasMessageArgument<'a>],
}

#[repr(C)]
struct LibertasNotificationRaw {
    recipients: *const LibertasUser,
    recipients_len: usize,
    data: *const u8,
    data_len: usize,
}

/// Sends a templated notification to recipients.
///
/// # Arguments
/// * `recipients` - List of recipient IDs.
/// * `level` - Message severity.
/// * `source` - Optional source ID.
/// * `resource_name` - Template resource name.
/// * `args` - Template arguments.
///
/// # Panics
/// On size limits or permission violations.
pub fn libertas_notification_send(
    recipients: &[LibertasUser],
    level: NotificationImportance,
    source: Option<LibertasId>,
    resource_name: &str,
    args: &[LibertasMessageArgument],
) {
    let sourceid = if let Some(source) = source {
        source
    } else {
        libertas_get_task_id()
    };
    let n = LibertasNotification {
        level,
        source: sourceid,
        resource_name,
        arguments: args,
    };
    let mut data = Vec::<u8>::new();
    n.avro_encode(&mut data);
    let raw_message = LibertasNotificationRaw {
        recipients: recipients.as_ptr(),
        recipients_len: recipients.len(),
        data: data.as_ptr(),
        data_len: data.len(),
    };
    __libertas_device_send_raw(
        PROTOCOL_LIBERTAS,
        DEVICE_SYSTEM,
        OP_SYSTEM_MESSAGE,
        0,
        0,
        &raw const raw_message as *const u8,
        core::mem::size_of::<LibertasNotificationRaw>(),
    )
}

/// Sends a literal text notification.
///
/// # Arguments
/// * `recipients` - List of recipient IDs.
/// * `level` - Message severity.
/// * `text` - Message content.
///
/// # Panics
/// On size limits or permission violations.
pub fn libertas_notification_send_literal(
    recipients: &[LibertasUser],
    level: NotificationImportance,
    text: &str,
) {
    let arg = LibertasMessageArgument::LiteralText(text);
    libertas_notification_send(recipients, level, None, "SYS_LITERAL", &[arg]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoded_arguments_borrow_without_changing_type_or_value() {
        let arguments = [
            LibertasMessageArgumentDecode::LiteralText(String::from("text")),
            LibertasMessageArgumentDecode::Object(42),
            LibertasMessageArgumentDecode::Boolean(true),
            LibertasMessageArgumentDecode::Signed(-7),
            LibertasMessageArgumentDecode::Unsigned(7),
            LibertasMessageArgumentDecode::Float(1.25),
            LibertasMessageArgumentDecode::Double(2.5),
            LibertasMessageArgumentDecode::UnitSigned {
                unit_type: String::from("kg"),
                value: -3,
            },
            LibertasMessageArgumentDecode::UnitUnsigned {
                unit_type: String::from("kg"),
                value: 3,
            },
            LibertasMessageArgumentDecode::UnitFloat {
                unit_type: String::from("kg"),
                value: 1.5,
            },
            LibertasMessageArgumentDecode::UnitDouble {
                unit_type: String::from("kg"),
                value: 2.5,
            },
            LibertasMessageArgumentDecode::ResourceText(String::from("READY")),
            LibertasMessageArgumentDecode::Plural(4),
            LibertasMessageArgumentDecode::DateOnly(20240229),
            LibertasMessageArgumentDecode::DateTime(1_704_067_200),
            LibertasMessageArgumentDecode::TimeOnly(86400),
            LibertasMessageArgumentDecode::FormattedText(libertas_formatted_text("INNER", &[])),
            LibertasMessageArgumentDecode::Currency {
                currency_code: String::from("USD"),
                scale: 2,
                minor_units: -5,
            },
        ];
        for argument in &arguments {
            assert_eq!(argument, &argument.as_message_argument());
        }
    }

    #[test]
    fn temporal_arguments_append_wire_tags_and_retain_values() {
        let values: [(i64, LibertasMessageArgument<'_>, i64); 3] = [
            (15, LibertasMessageArgument::DateOnly(20240229), 20240229),
            (
                16,
                LibertasMessageArgument::DateTime(253402214399),
                253402214399,
            ),
            (17, LibertasMessageArgument::TimeOnly(86400), 86400),
        ];
        for (tag, argument, value) in values {
            let encoded = libertas_formatted_text("WHEN", &[argument]);
            let mut expected = Vec::new();
            "WHEN".avro_encode(&mut expected);
            1i64.avro_encode(&mut expected);
            tag.avro_encode(&mut expected);
            value.avro_encode(&mut expected);
            0i64.avro_encode(&mut expected);
            assert_eq!(encoded, expected);
            let mut offset = 0;
            let decoded = crate::DataName::avro_decode(&encoded, &mut offset).unwrap();
            assert_eq!(offset, encoded.len());
            assert_eq!(decoded.arguments[0], argument);
            assert_eq!(
                decoded.arguments[0],
                decoded.arguments[0].as_message_argument()
            );
        }
    }

    #[test]
    fn formatted_text_encoder_matches_the_canonical_tuple() {
        assert_eq!(libertas_formatted_text("READY", &[]), b"\x0aREADY\x00",);
        assert_eq!(
            libertas_formatted_text("DEVICE", &[LibertasMessageArgument::Object(42)],),
            b"\x0cDEVICE\x02\x02\x54\x00",
        );
        let inner = libertas_formatted_text("INNER", &[LibertasMessageArgument::Plural(2)]);
        let outer =
            libertas_formatted_text("OUTER", &[LibertasMessageArgument::formatted_text(&inner)]);
        assert_eq!(outer, b"\x0aOUTER\x02\x1a\x14\x0aINNER\x02\x18\x04\x00\x00",);
        let mut offset = 0;
        let decoded = crate::DataName::avro_decode(&outer, &mut offset)
            .unwrap_or_else(|error| panic!("{error}: {outer:?}"));
        assert_eq!(offset, outer.len());
        assert_eq!(decoded.resource_name, "OUTER");
        assert_eq!(
            decoded.arguments,
            Vec::from([LibertasMessageArgumentDecode::FormattedText(inner)])
        );

        let exact = libertas_formatted_text(
            "AMOUNT",
            &[LibertasMessageArgument::Currency {
                currency_code: "USD",
                scale: 2,
                minor_units: -5,
            }],
        );
        assert_eq!(exact, b"\x0cAMOUNT\x02\x1c\x06USD\x04\x09\x00");
        let mut offset = 0;
        let decoded = crate::DataName::avro_decode(&exact, &mut offset).unwrap();
        assert_eq!(offset, exact.len());
        assert_eq!(
            decoded.arguments,
            Vec::from([LibertasMessageArgumentDecode::Currency {
                currency_code: String::from("USD"),
                scale: 2,
                minor_units: -5,
            }])
        );
    }
}
