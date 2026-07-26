use crate::*;

/// The log level for a log message, indicating the severity or importance of the log. 
/// This enum is used as a parameter in the `libertas_log` function to specify the level of the log message.
/// The log levels are defined as follows:
/// - `Trace`: The least severe log level, used for very detailed information that is typically only useful for debugging purposes. This level may include information about the internal state of the application, the flow of execution, and other fine-grained details that can help developers understand how the application is behaving. Log messages at this level are usually very verbose and may not be enabled in production environments due to their high volume.
/// - `Debug`: A log level used for debugging information that is less detailed than `Trace` but still provides useful insights into the application's behavior. This level may include information about the values of variables, the results of function calls, and other information that can help developers identify and fix issues during development. Log messages at this level are typically more concise than `Trace` and may be enabled in development environments while being disabled in production.
/// - `Info`: A log level used for general informational messages that indicate the normal operation of the application. This level may include messages about the successful completion of operations, the startup and shutdown of the application, and other events that are noteworthy but not indicative of any problems. Log messages at this level are usually concise and may be enabled in both development and production environments to provide insights into the application's behavior without overwhelming the logs with too much detail.
/// - `Warn`: A log level used for warning messages that indicate potential issues or situations that may require attention but are not necessarily errors. This level may include messages about deprecated features, unexpected input, or other conditions that could lead to problems if not addressed. Log messages at this level are typically concise and may be enabled in both development and production environments to help identify potential issues before they become critical.
/// - `Error`: A log level used for error messages that indicate actual problems that have occurred in the application. This level may include messages about exceptions, failed operations, or other conditions that indicate something has gone wrong. Log messages at this level are usually concise and may be enabled in both development and production environments to help identify and diagnose issues that need to be fixed.
/// - `Fatal`: The most severe log level, used for critical error messages that indicate conditions that may cause the application to crash or become unusable. This level may include messages about unrecoverable errors, critical failures, or other conditions that require immediate attention. Log messages at this level are typically concise and may be enabled in both development and production environments to help identify and address critical issues that need to be resolved to ensure the stability and reliability of the application.
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

/// Write a log message with the specified log level and message content. This function allows you to log messages from your application, which can be useful for debugging, monitoring, and analyzing the behavior of your application. 
/// The `level` parameter specifies the severity or importance of the log message using the `LogLevel` enum, 
/// while the `message` parameter is a string slice that contains the content of the log message you want to send. 
///
pub fn libertas_log(level: LogLevel, message: &str) {
    let log_internal = LogInternal {
        level,
        message: message.as_ptr(),
        message_len: message.len(),
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM, OP_SYSTEM_LOG, 0, 0, &log_internal as *const LogInternal as *const u8, core::mem::size_of::<LogInternal>());
}
