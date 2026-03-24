use crate::Vec;
use crate::RUNTIME_API;
use crate::DEVICE_SYSTEM;
use crate::OP_SYSTEM_MESSAGE;
use crate::PROTOCOL_LIBERTAS;

/// Represents the type of a Libertas system object
/// 
#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LibertasObjectType {
    PhysicalDevice,
    LogicalDevice,
    App,
    AppTask,
    User,
    Client,
}

/// Represents the severity level of a message in the Libertas system
#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LibertasMessageLevel {
    Debug,
    Info,
    AlertLow,
    AlertGuarded,
    AlertElevated,
    AlertHigh,
    AlertSevere,
}

/// Represents a system object in the Libertas system, such as a device, app, or user
#[repr(C)]
pub struct LibertasObject {
    pub obj_type: LibertasObjectType,
    pub obj_id: u32,
}

/// Represents an argument that can be included in a message sent through the Libertas system
pub enum LibertasMessageArgument<'a> {
    Boolean(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    // A literal string, not resource string
    Text(&'a str),
    // It is the string resource name, not literal string!
    ResourceString (&'a str), 
    UnitSigned {unit_type: &'a str, value: i64},
    UnitUnsigned {unit_type: &'a str, value: u64},
    UnitFloat {unit_type: &'a str, value: f64},
    // A Liberta system object. App task must have access permission to the object.
    Object {obj_type: LibertasObjectType, obj_id: u32},
}

/// Internal representation of a message argument for C API compatibility
#[repr(C)]
struct LibertasMessageArgumentInternal {
    arg_type: u8,
    text: *const u8,
    text_len: usize,
    value: u64,
}

impl LibertasMessageArgumentInternal {
    fn from(arg: &LibertasMessageArgument) -> Self {
        match arg {
            LibertasMessageArgument::Boolean(b) => {
                Self {
                    arg_type: 0,
                    text: core::ptr::null(),
                    text_len: 0,
                    value: if *b { 1 } else { 0 },
                }
            }
            LibertasMessageArgument::Signed(s) => {
                Self {
                    arg_type: 1,
                    text: core::ptr::null(),
                    text_len: 0,
                    value: *s as u64,
                }
            }
            LibertasMessageArgument::Unsigned(u) => {
                Self {
                    arg_type: 2,
                    text: core::ptr::null(),
                    text_len: 0,
                    value: *u,
                }
            }
            LibertasMessageArgument::Float(f) => {
                Self {
                    arg_type: 3,
                    text: core::ptr::null(),
                    text_len: 0,
                    value: f.to_bits(),
                }
            }
            LibertasMessageArgument::Text(s) => {
                Self {
                    arg_type: 4,
                    text: s.as_ptr(),
                    text_len: s.len(),
                    value: 0,
                }
            }
            LibertasMessageArgument::ResourceString(slice) => {
                Self {
                    arg_type: 5,
                    text: slice.as_ptr(),
                    text_len: slice.len(),
                    value: 0,
                }
            }
            LibertasMessageArgument::UnitSigned {unit_type, value} => {
                Self {
                    arg_type: 6,
                    text: unit_type.as_ptr(),
                    text_len: unit_type.len(),
                    value: *value as u64,
                }
            }
            LibertasMessageArgument::UnitUnsigned {unit_type, value} => {
                Self {
                    arg_type: 7,
                    text: unit_type.as_ptr(),
                    text_len: unit_type.len(),
                    value: *value,
                }
            }
            LibertasMessageArgument::UnitFloat {unit_type, value} => {
                Self {
                    arg_type: 8,
                    text: unit_type.as_ptr(),
                    text_len: unit_type.len(),
                    value: value.to_bits(),
                }
            }
            LibertasMessageArgument::Object {obj_type, obj_id} => {
                let v = *obj_type as u64;
                Self {
                    arg_type: 9,
                    text: core::ptr::null(),
                    text_len: 0,
                    value: (v << 32) | *obj_id as u64
                }
            }
        }
    }
}

#[repr(C)]
struct LibertasUserMessage {
    level: LibertasMessageLevel,
    recipients: *const u32,
    recipients_size: usize,
    source: LibertasObject,
    resource_name: *const u8,
    resource_name_len: usize,
    arguments: *const LibertasMessageArgumentInternal,
    arguments_len: usize,
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
pub fn libertas_send_message(recipients: &[u32], level: LibertasMessageLevel, source: Option<LibertasObject>, resource_name: &str, args: &[LibertasMessageArgument]) {
    let mut arg_internals: Vec<LibertasMessageArgumentInternal> = Vec::with_capacity(args.len());
    for arg in args {
        arg_internals.push(LibertasMessageArgumentInternal::from(arg));
    }
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            let src = match source {
                Some(s) => s,
                None => LibertasObject { obj_type: LibertasObjectType::App, obj_id: 0 },
            };
            let message = LibertasUserMessage {
                level,
                recipients: recipients.as_ptr(),
                recipients_size: recipients.len(),
                source: src,
                resource_name: resource_name.as_ptr(),
                resource_name_len: resource_name.len(),
                arguments: arg_internals.as_ptr(),
                arguments_len: arg_internals.len(),
            };
            (runtime_api.device_send)(PROTOCOL_LIBERTAS, DEVICE_SYSTEM, 0, OP_SYSTEM_MESSAGE, &raw const message as *const u8, core::mem::size_of::<LibertasUserMessage>(), 0);
        } else {
            unreachable!();
        }
    }
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
pub fn libertas_send_message_literal(recipients: &[u32], level: LibertasMessageLevel, text: &str) {
    let arg = LibertasMessageArgument::Text(text);
    libertas_send_message(recipients, level, None, "SYS_LITERAL", &[arg]);
}
