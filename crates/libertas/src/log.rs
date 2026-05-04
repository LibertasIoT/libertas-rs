use crate::*;

#[repr(u8)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[repr(C)]
struct LogInternal {
    level: LogLevel,
    message: *const u8,
    message_len: usize,
}

pub fn libertas_log(level: LogLevel, message: &str) {
    let log_internal = LogInternal {
        level,
        message: message.as_ptr(),
        message_len: message.len(),
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM, OP_SYSTEM_LOG, 0, 0, &log_internal as *const LogInternal as *const u8, core::mem::size_of::<LogInternal>());
}
