/// Libertas Rust SDK - Notification API
/// This module provides functions for sending notifications through the Libertas system. It defines the `NotificationImportance` enum to represent the severity level of messages, the `NotificationArgument` enum to 
/// represent various types of arguments that can be included in messages, and the `NotificationArgumentDecode` enum for decoding notification arguments. The main function in this module is `libertas_notification_send`, 
/// which allows you to send a system message to a list of recipients with a specified importance level, source, resource name, and arguments. Additionally, there is a convenience function `libertas_notification_send_literal` 
/// for sending simple literal text messages without using a resource template. By using this module, you can easily send notifications to users or groups within the Libertas system, allowing for effective communication and 
/// alerting based on various conditions and events in your application.
/// 

use alloc::vec::Vec;
use alloc::string::String;
use libertas_macros::{LibertasAvroEncode, LibertasAvroDecode};
use crate::*;

/// Represents the severity level of a message in the Libertas system. This enum is used to categorize messages based on their importance and urgency, allowing the system to prioritize and handle them accordingly. The different levels of importance are defined as follows:
/// - `Debug`: The least severe level, used for messages that are primarily intended for debugging purposes. These messages may contain detailed information about the internal state of the system, the flow of execution, and other fine-grained details that can help developers understand how the system is behaving. Debug messages are typically not shown to end-users and are intended for use during development and troubleshooting.
/// - `Info`: A level used for informational messages that indicate the normal operation of the system. These messages may contain general information about the system's operation or status, and they are typically not indicative of any issues or problems. Informational messages are intended to provide insights into the normal functioning of the system and may be shown to end-users or logged for reference, but they do not require immediate attention or action. 
/// - `AlertLow`: A level used for warning messages that indicate a potential issue or situation that could lead to problems if not addressed. These messages are typically used to alert developers or users about conditions that are not ideal but do not necessarily indicate an immediate failure or error. AlertLow messages may require attention and action to prevent further issues, but they do not indicate a critical problem at the moment.
/// - `AlertGuarded`: A level used for elevated alert messages that indicate a more significant issue or situation that requires attention. These messages are typically used to signal conditions that are more serious than warnings and may have a higher likelihood of leading to problems if not addressed. AlertGuarded messages often require prompt attention and action to mitigate potential issues and prevent them from escalating further.
/// - `AlertElevated`: A level used for high alert messages that indicate a critical issue or situation that requires immediate attention. These messages are typically used to signal conditions that are severe and have a high likelihood of causing significant problems or failures if not addressed promptly. AlertElevated messages often require urgent action to resolve the issue and prevent further damage or disruption to the system. 
/// - `AlertHigh`: A level used for severe alert messages that indicate an extremely critical issue or situation that requires immediate and decisive action. These messages are typically used to signal conditions that are dire and have an imminent risk of causing catastrophic failures or significant harm if not addressed immediately. AlertHigh messages often require emergency response and may involve multiple stakeholders working together to resolve the issue and prevent further damage or disruption to the system. 
/// - `AlertSevere`: The most severe level, used for extremely severe alert messages that indicate a situation of utmost criticality that demands immediate and extraordinary measures. These messages are typically reserved for conditions that are beyond dire and have an imminent and catastrophic risk of causing widespread failures, significant harm, or even loss of life if not addressed with the highest level of urgency and resources. AlertSevere messages often require a coordinated emergency response involving multiple teams, organizations, or even government agencies to manage the crisis and mitigate the impact on the system and its users.
/// The `NotificationImportance` enum allows the Libertas system to classify messages based on their severity and importance, enabling appropriate handling and prioritization of messages to ensure the stability, reliability, and safety of the system and its users.
#[repr(u8)]
#[derive(LibertasAvroEncode, PartialEq, Eq, Debug, Clone, Copy)]
pub enum NotificationImportance {
    Debug,
    Info,
    AlertLow,
    AlertGuarded,
    AlertElevated,
    AlertHigh,
    AlertSevere,
}

/// Represents an argument that can be included in a message sent through the Libertas system. 
/// This enum defines various types of arguments that can be used to provide additional information or context in messages, 
/// allowing for more dynamic and informative communication within the system. 
/// Each variant of the `NotificationArgument` enum corresponds to a different type of argument that can be included in a message, 
/// such as literal text, resource text, objects, boolean values, numerical values, and unit values with specific types. 
/// By using this enum, developers can create messages that are rich in content and can convey a wide range of information to users or 
/// other parts of the system when sending notifications through the Libertas system.
/// 
/// It is also used as argument in a name template to provide predictible yet flexible name for database name and data field, making 
/// Libertas Apps fully self-describing.
/// The `NotificationArgument` enum includes the following variants:
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
/// - `Plural`: A plural value that can be included as an argument in a message sent through the Libertas system. This variant of the `NotificationArgument` enum allows you to include a numerical value that represents a count for pluralization purposes in the context of the message being sent. The `Plural` variant is typically used in conjunction with resource string templates that have plural forms, where the value provided in this argument will determine which plural form of the resource string to use when generating the final message. For example, if you have a resource string that has different forms for singular and plural (e.g., "You have {0} new message" vs. "You have {0} new messages"), you would use the `Plural` argument to provide the count of new messages, and the system would automatically certain word of the resource string based on that count.
#[derive(LibertasAvroEncode)]
pub enum NotificationArgument<'a> {
    LiteralText(&'a str),
    Object(u32),
    Boolean(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f32),
    Double(f64),
    UnitSigned {unit_type: &'a str, value: i64},
    UnitUnsigned {unit_type: &'a str, value: u64},
    UnitFloat {unit_type: &'a str, value: f32},
    UnitDouble {unit_type: &'a str, value: f64},
    ResourceText (&'a str), 
    Plural(i64),
}

/// A struct representing a decoded notification argument that can be compared with `NotificationArgument`. This struct is used for decoding notification arguments that have been encoded in Avro format and allows for comparison with the original `NotificationArgument` values. Each variant of this enum corresponds to a variant in the `NotificationArgument` enum, but with owned types (e.g., `String` instead of `&str`) to facilitate decoding and comparison. The implementation of `PartialEq` allows you to compare a `NotificationArgumentDecode` instance with a `NotificationArgument` instance to check if they represent the same argument value, which can be useful for testing or for verifying that the decoded arguments match the expected values when processing notifications.
/// Note that the `NotificationArgumentDecode` enum is designed to be used in conjunction with the Avro decoding process, 
/// where you would decode a message's arguments from Avro format into instances of `NotificationArgumentDecode`, 
/// and then compare those decoded arguments with the original `NotificationArgument` values to ensure they match as expected. 
/// This can help verify that the encoding and decoding processes are working correctly and that the arguments are being transmitted 
/// accurately through the Libertas system. 
/// For example, if you have a message template that includes certain arguments, you can encode those arguments using 
/// `NotificationArgument`, send the message through the Libertas system, and then decode the arguments on the receiving end 
/// into `NotificationArgumentDecode` instances. You can then compare the decoded arguments with the original `NotificationArgument` 
/// values to confirm that they were transmitted correctly and that the message content is accurate based on the provided arguments.
/// 
/// It is also used as argument in a name template to provide predictible yet flexible name for database name and data field, making 
/// Libertas Apps fully self-describing.
/// 
/// The `NotificationArgumentDecode` enum includes the following variants:
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
/// - `Plural`: A plural value that can be included as an argument in a message sent through the Libertas system. This variant of the `NotificationArgument` enum allows you to include a numerical value that represents a count for pluralization purposes in the context of the message being sent. The `Plural` variant is typically used in conjunction with resource string templates that have plural forms, where the value provided in this argument will determine which plural form of the resource string to use when generating the final message. For example, if you have a resource string that has different forms for singular and plural (e.g., "You have {0} new message" vs. "You have {0} new messages"), you would use the `Plural` argument to provide the count of new messages, and the system would automatically certain word of the resource string based on that count.
#[derive(LibertasAvroDecode, PartialEq, Debug, Clone)]
pub enum NotificationArgumentDecode {
    LiteralText(String),
    Object(u32),
    Boolean(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f32),
    Double(f64),
    UnitSigned {unit_type: String, value: i64},
    UnitUnsigned {unit_type: String, value: u64},
    UnitFloat {unit_type: String, value: f32},
    UnitDouble {unit_type: String, value: f64},
    ResourceText (String), 
    Plural(i64),
}

impl PartialEq<NotificationArgument<'_>> for NotificationArgumentDecode {
    fn eq(&self, other: &NotificationArgument<'_>) -> bool {
        match (self, other) {
            (NotificationArgumentDecode::LiteralText(s1), NotificationArgument::LiteralText(s2)) => s1 == s2,
            (NotificationArgumentDecode::Object(o1), NotificationArgument::Object(o2)) => o1 == o2,
            (NotificationArgumentDecode::Boolean(b1), NotificationArgument::Boolean(b2)) => b1 == b2,
            (NotificationArgumentDecode::Signed(i1), NotificationArgument::Signed(i2)) => i1 == i2,
            (NotificationArgumentDecode::Unsigned(u1), NotificationArgument::Unsigned(u2)) => u1 == u2,
            (NotificationArgumentDecode::Float(f1), NotificationArgument::Float(f2)) => f1 == f2,
            (NotificationArgumentDecode::Double(d1), NotificationArgument::Double(d2)) => d1 == d2,
            (NotificationArgumentDecode::UnitSigned {unit_type: ut1, value: v1}, NotificationArgument::UnitSigned {unit_type: ut2, value: v2}) => ut1 == ut2 && v1 == v2,
            (NotificationArgumentDecode::UnitUnsigned {unit_type: ut1, value: v1}, NotificationArgument::UnitUnsigned {unit_type: ut2, value: v2}) => ut1 == ut2 && v1 == v2,
            (NotificationArgumentDecode::UnitFloat {unit_type: ut1, value: v1}, NotificationArgument::UnitFloat {unit_type: ut2, value: v2}) => ut1 == ut2 && v1 == v2,
            (NotificationArgumentDecode::UnitDouble {unit_type: ut1, value: v1}, NotificationArgument::UnitDouble {unit_type: ut2, value: v2}) => ut1 == ut2 && v1 == v2,
            (NotificationArgumentDecode::ResourceText(s1), NotificationArgument::ResourceText(s2)) => s1 == s2,
            (NotificationArgumentDecode::Plural(p1), NotificationArgument::Plural(p2)) => p1 == p2,
            _ => false
        }
    }
}

#[derive(LibertasAvroEncode)]
struct LibertasNotification<'a> {
    level: NotificationImportance,
    source: u32,
    resource_name: &'a str,
    arguments: &'a [NotificationArgument<'a>],
}

#[repr(C)]
struct LibertasNotificationRaw {
    recipients: *const u32,
    recipients_len: usize,
    data: *const u8,
    data_len: usize,
}

/// Sends a system message to a list of recipients. Each recipient is a system user or group.
/// 
/// The message will be delivered to recipients' smartphones or other supported devices.
/// 
/// # Panics
/// Some host platforms may limit the size of the message. App may panic on such platforms. Libertas Hub doesn't limit the message size thus will never panic.
/// 
/// By default the access permission is limited to the users (or groups) within the input data of the task function. If the user doesn't input a user in the 
/// task's function arguments and the code accesses that user, the task will be terminated for access violation.
///
/// # Arguments
/// * `recipients` - List of recipient IDs (user, group or client) to send the message to
/// * `level` - The severity level of the message (e.g., Debug, Info, Alert)
/// * `source` - Optional source object identifying the sender (e.g., device or app)
/// * `resource_name` - The resource string name for the message template
/// * `args` - Arguments to substitute into the message template
/// 
pub fn libertas_notification_send(recipients: &[u32], level: NotificationImportance, source: Option<u32>, resource_name: &str, args: &[NotificationArgument]) {
    let sourceid = if let Some(source) = source { source } else { libertas_get_task_id() };
    let n = LibertasNotification{
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
        data_len: data.len()
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM, OP_SYSTEM_MESSAGE, 0, 0, &raw const raw_message as *const u8, core::mem::size_of::<LibertasNotificationRaw>())
}

/// Sends a literal text message to a list of recipients.
///
/// This function sends a simple text message directly without using a resource template.
/// The message will be delivered to recipients' smartphones or other supported devices.
/// 
/// # Panics
/// App will panic if the resulting message size exceeds STACK_BUF_SIZE bytes.
/// App will panic on permission violation, including recipients.
///
/// # Arguments
/// * `recipients` - List of recipient IDs (user, group or client) to send the message to
/// * `level` - The severity level of the message (e.g., Debug, Info, Alert)
/// * `text` - The literal text content of the message to send
pub fn libertas_notification_send_literal(recipients: &[u32], level: NotificationImportance, text: &str) {
    let arg = NotificationArgument::LiteralText(text);
    libertas_notification_send(recipients, level, None, "SYS_LITERAL", &[arg]);
}
