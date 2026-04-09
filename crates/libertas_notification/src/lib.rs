#![no_std]
extern crate alloc;

use alloc::vec::Vec;

use libertas::*;
use libertas_macros::LibertasAvroEncode;

/// Represents the severity level of a message in the Libertas system
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

/// Represents an argument that can be included in a message sent through the Libertas system
#[derive(LibertasAvroEncode)]
pub enum NotificationArgument<'a> {
    /// A literal string, not resource string
    LiteralText(&'a str),
    /// A Liberta system object such a a device or a user, etc. App task must have access permission to the object.
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
    /// It is the string resource name, not literal string!
    ResourceText (&'a str), 
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
/// Some host platform may limit the size of the message. App may panic on such platforms. Libertas Hub doesn't limit the message size thus will never panic.
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
pub fn libertas_send_notification(recipients: &[u32], level: NotificationImportance, source: Option<u32>, resource_name: &str, args: &[NotificationArgument]) {
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
pub fn libertas_send_notification_literal(recipients: &[u32], level: NotificationImportance, text: &str) {
    let arg = NotificationArgument::LiteralText(text);
    libertas_send_notification(recipients, level, None, "SYS_LITERAL", &[arg]);
}
