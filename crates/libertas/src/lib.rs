//! Libertas SDK Rust Bindings
//!
//! Rust bindings for Libertas Apps, enabling interaction with peers, timer management, and persistent data handling.
//!
//! # Core Features
//!
//! - **Datetime**: Access UTC/local time and monotonic system ticks for scheduling.
//! - **Timers**: Create interval and deadline timers for periodic or delayed tasks.
//! - **Peer Communication**: Send/receive messages via developer-defined protocols with peers (humans, LLMs, or other apps).
//! - **Logging and Notifications**: Log messages and send notifications with configurable importance.
//! - **Persistent Data**: Store/retrieve data in standalone or indexed databases using Avro encoding.
//!
//! # Example
//!
//! ```ignore
//! // Create a recurring timer (timers are one-off; update in callback for recurrence)
//! let timer_id = libertas_timer_new_interval(libertas_get_sys_ticks() + 1000, |timer, cur_ticks, _tag| {
//!     libertas_log(LogLevel::Info, b"timer fired");
//!     // Handle event and reschedule
//!     libertas_timer_update_interval(timer, cur_ticks + 1000);
//! }, Box::new(()));
//! ```

#![no_std]
// Bring in the alloc crate
extern crate alloc;

extern crate self as libertas;

mod avro;
mod notification;
mod data;
mod log;

pub use avro::{AvroDecode, AvroEncode, NotBytesDecode, NotBytesEncode};
pub use notification::{NotificationImportance, NotificationArgument, libertas_notification_send, libertas_notification_send_literal};
pub use data::{DataName, IndexedData, IndexDirection, IndexedDataStat, libertas_data_get_names, libertas_data_get_indexed_names, libertas_data_write, libertas_data_write_indexed, libertas_data_read, libertas_data_read_indexed, libertas_data_read_indexed_range, libertas_data_remove, libertas_data_remove_indexed, libertas_data_remove_indexed_records, libertas_data_open_indexed};
pub use log::{LogLevel, libertas_log};
pub use libertas_hub::{
    EndpointMessageStatus,
    EndpointMessageStatus as LibertasEndpointStatus,
    EndpointOperation,
    OP_ENDPOINT_DATA,
    OP_ENDPOINT_PEER_DOWN,
    OP_ENDPOINT_PEER_TIMEOUT,
    OP_ENDPOINT_REQ,
    OP_ENDPOINT_RSP,
    OP_ENDPOINT_SUB_REQ,
};
pub use libertas_utils::{InlineByteBuffer, STACK_BUF_SIZE};

use alloc::{slice, boxed::Box, rc::Rc, vec::Vec};
use core::ffi::c_void;
use core::{any::Any, cell::RefCell};
use core::cmp::Reverse;
use core::sync::atomic::{AtomicPtr, AtomicU8, Ordering};

use hashbrown::HashMap;
use hashbrown::DefaultHashBuilder;
use priority_queue::PriorityQueue;

/// Core entity representing any information source in the Libertas ecosystem.
/// Examples: physical devices (lights, sensors), virtual entities (email accounts, files), or services.
pub type LibertasDevice = u32;

/// Dynamically created virtual device bound to its creating task.
/// Automatically deleted when the task is removed.
pub type LibertasVirtualDevice = LibertasDevice;

/// Specialized virtual device for endpoint interactions.
/// Data schema must be predefined and published by the server developer.
pub type LibertasEndpoint = LibertasVirtualDevice;

/// Timestamp in seconds since Unix epoch (full date and time).
pub type LibertasDateTime = u64;

/// Time-only timestamp in seconds (no date component).
pub type LibertasTimeOnly = u32;

/// Identifier for a local area network device.
pub type LibertasLanDevice = u32;

/// User identifier.
pub type LibertasUser = u32;

/// Action identifier.
pub type LibertasAction = u32;

/// Transaction identifier carried by every Libertas interaction.
///
/// A `LibertasTransId` is never optional at the runtime boundary. Libertas does
/// not reserve a universal value to mean "missing" or "ignored"; each
/// interaction protocol is responsible for designating and documenting such a
/// value when needed.
pub type LibertasTransId = u32;

/// Transaction ID that the built-in endpoint protocol designates as ignored
/// when an interaction does not correlate with a transaction.
pub const LIBERTAS_ENDPOINT_IGNORED_TRANS_ID: LibertasTransId = 0;

/// Handle for an opened indexed data store, obtained via `libertas_data_open_indexed`.
/// Valid until closed or the opening task terminates.
pub type LibertasDataStore = u32;

/// Broadcast destination for sending to all peers.
pub const LIBERTAS_BROADCAST_DEST: u32 = 0xffffffff;

const OP_SYSTEM_LOG: u8 = 0xe0;

/// Send by endpoint server to notify underlying Libertas OS to remove the peer
/// from subscription list. Future broadcasts will not include that peer.
const OP_ENDPOINT_REMOVE_PEER: u8 = 22;

// Internal App lifecycle opcode. It is delivered Host -> App through
// LibertasPackageCallback::device_callback and acknowledged App -> Host through
// LibertasRuntimeApi::device_send. App code uses the shutdown APIs below and
// never handles the opcode directly.
const OP_APP_SHUT_DOWN: u8 = 0xfd;
const OP_SYSTEM_WAKE_UP: u8 = 255;         // See 
const PROTOCOL_LIBERTAS: u16 = 0;

const DEVICE_SYSTEM: u32 = 0;
const DEVICE_SYSTEM_DATABASE_STADNALONE: u32 = 0;
const DEVICE_SYSTEM_DATABASE_INDEXED: u32 = 1;

const OP_SYSTEM_MESSAGE: u8 = 0xfe;
const OP_SYSTEM_DATABASE_GET_NAMES: u8 = 0xf0;
const OP_SYSTEM_DATABASE_OPEN_INDEXED_DATA: u8 = 0xf1;
const OP_SYSTEM_DATABASE_WRITE_DATA: u8 = 0xf2;
const OP_SYSTEM_DATABASE_READ_DATA: u8 = 0xf3;
const OP_SYSTEM_DATABASE_REMOVE_DATA: u8 = 0xf4;
const OP_SYSTEM_DATABASE_REMOVE_RECORD: u8 = 0xf5;

const CURRENT_VERSION: u32 = 0x000206;     // Version 0.2.6, 1.0 shall be 0x10000, each sub version must be within [0,255]

type LibertasTimerCallback = dyn FnMut(u32, u64, &mut Box<dyn Any>);
type LibertasDeviceCallback = dyn FnMut(LibertasDevice, u8, &[u8], &mut Box<dyn Any>, LibertasTransId, u32);
type LibertasWakeupCallback = dyn FnMut(&mut Box<dyn Any>);
type LibertasShutdownHandler = dyn FnMut(&mut Box<dyn Any>);

const SHUTDOWN_RUNNING: u8 = 0;
const SHUTDOWN_REQUESTED: u8 = 1;
const SHUTDOWN_COMPLETED: u8 = 2;

#[doc(hidden)]
pub trait LibertasExport {
}

// A count down timer
enum TimerState {
    Idle,
    ArmedInterval ( u64 ),
    ArmedDeadline ( u64 ),
}
struct Timer {
    task: u32,
    state: TimerState,
    cb: Rc<RefCell<Box<LibertasTimerCallback>>>,
    context: Rc<RefCell<Box<dyn Any>>>,
}

struct DeviceCallback {
    cb: Rc<RefCell<Box<LibertasDeviceCallback>>>,
    context: Rc<RefCell<Box<dyn Any>>>,
}

struct WakeupCallback {
    cb: Rc<RefCell<Box<LibertasWakeupCallback>>>,
    context: Rc<RefCell<Box<dyn Any>>>,
}

struct ShutdownHandler {
    cb: Rc<RefCell<Box<LibertasShutdownHandler>>>,
    context: Rc<RefCell<Box<dyn Any>>>,
}

#[repr(C)]
struct TimerDriverResult {
    next_monotonic: u64,
    next_utc: u64,
}

#[repr(C)]
struct ReadResult {
    success: bool,
    data: *const u8, 
    data_len: usize,
}

/// C API provided by Libertas OS runtime to the App package. This struct is passed to the App package during 
/// initialization and contains function pointers for all interactions with the system.
#[repr(C)]
struct LibertasRuntimeApi {
    version: u32,                       // Version of runtime. For compatibility check. Currently not used
    get_random: extern "C" fn(u8) -> u64,
    get_task_id: extern "C" fn() -> u32,
    set_task_id: extern "C" fn(u32),    // Used internally in callback driver. Only useful if we are driving multiple tasks within one thread.
    get_sys_ticks: extern "C" fn() -> u64,
    get_utc_time: extern "C" fn() -> u64,
    utc_to_local: extern "C" fn(i64) -> i64,
    local_to_utc: extern "C" fn(i64) -> i64,
    device_send: extern "C" fn(u16, u32, LibertasTransId, u8, *const u8, usize, u32),           // Protocol, device (virtual device src), trans_id, op_code, data, data_len, ack_dest (virtual device & endpoint)
    device_read: extern "C" fn(u16, u32, u8, *const u8, usize) -> ReadResult,       // Synchronous kernel operation for current task. protocol, device, op_code, data, data_len
}

// Use &pt as *const LibertasPackageCallback to pass back to C
#[doc(hidden)]
#[repr(C)]
pub struct LibertasPackageCallback {
    version: u32,
    init_task: extern "C" fn(u32),
    remove_task: extern "C" fn(u32),
    timer_driver: extern "C" fn(u64, u64)->TimerDriverResult,
    device_callback: extern "C" fn(u32, u32, u8, LibertasTransId, *const c_void, usize, u32),       // task, device (virtual device dst), op_code, trans_id, data, data_len, source (virtual device & endpoint)
}

struct Context {
    device_callbacks: HashMap<u32, DeviceCallback>,
    shutdown_handler: Option<ShutdownHandler>,
}

impl Context {
    fn new() -> Self {
        Self {
            device_callbacks: HashMap::new(),
            shutdown_handler: None,
        }
    }
}

extern "C" fn libertas_impl_init_task(task_id: u32) {
    unsafe {
        match ENV {
            Some(ref mut env) => env.init_task(task_id),
            None => { unreachable!(); }
        }
    }
}

extern "C" fn libertas_impl_remove_task(task_id: u32) {
    unsafe {
        match ENV {
            Some(ref mut env) => env.remove_task(task_id),
            None => { unreachable!(); }
        }
    }
}

extern "C" fn libertas_impl_device_callback(task: u32, device: u32, op_code: u8, trans_id: LibertasTransId, data: *const c_void, data_len: usize, peer: u32) {
    if device == DEVICE_SYSTEM && op_code == OP_APP_SHUT_DOWN {
        if SHUTDOWN_STATE.compare_exchange(
                SHUTDOWN_RUNNING,
                SHUTDOWN_REQUESTED,
                Ordering::AcqRel,
                Ordering::Acquire).is_err() {
            return;
        }

        let handler = unsafe {
            let env_ptr = &raw mut ENV;
            match *env_ptr {
                Some(ref mut env) => env.contexts.get_mut(&task)
                    .and_then(|context| context.shutdown_handler.as_ref())
                    .map(|handler| (Rc::clone(&handler.cb), Rc::clone(&handler.context))),
                None => None,
            }
        };
        if let Some((callback, context)) = handler {
            (callback.borrow_mut())(&mut *context.borrow_mut());
        } else {
            libertas_shutdown_complete();
        }
        return;
    }

    if SHUTDOWN_STATE.load(Ordering::Acquire) == SHUTDOWN_COMPLETED {
        return;
    }

    unsafe {
        match ENV {
            Some(ref mut env) => {
                if let Some(context) = env.contexts.get_mut(&task) {
                    if let Some(dcb) = context.device_callbacks.get_mut(&device) {
                        let d = if data_len == 0 {
                            &[]
                        } else {
                            if data.is_null() {
                                return;
                            }
                            slice::from_raw_parts(data as *const u8, data_len)
                        };
                        (dcb.cb.borrow_mut())(device, op_code, d, &mut *dcb.context.borrow_mut(), trans_id, peer);
                    }
                }
            }
            _ => { unreachable!(); }
        }
    }   
}

extern "C" fn libertas_impl_timer_driver(monotonic: u64, utc: u64) -> TimerDriverResult {
    if SHUTDOWN_STATE.load(Ordering::Acquire) == SHUTDOWN_COMPLETED {
        return TimerDriverResult {
            next_monotonic: u64::MAX,
            next_utc: u64::MAX,
        };
    }
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            match ENV {
                Some(ref mut env) => {
                    // Process wakeup callback first if exists.
                    if let Some(ref wakeup_callback) = env.wakeup_callbacks {
                        (wakeup_callback.cb.borrow_mut())(&mut *wakeup_callback.context.borrow_mut());
                    }
                    // A wakeup callback may complete asynchronous shutdown.
                    // Once completed, no timer or other App callback may start.
                    if SHUTDOWN_STATE.load(Ordering::Acquire) == SHUTDOWN_COMPLETED {
                        return TimerDriverResult {
                            next_monotonic: u64::MAX,
                            next_utc: u64::MAX,
                        };
                    }
                    let mut next_mono = u64::MAX;
                    let mut next_time = u64::MAX;
                    loop {
                        if let Some(mono) = env.timers_active_interval.peek() {
                            if *mono.1 > Reverse(monotonic) {
                                let timer = *mono.0;
                                if let Some(t) = env.timers.get_mut(&timer) {
                                    t.state = TimerState::Idle;
                                    env.timers_active_interval.pop();
                                    (runtime_api.set_task_id)(t.task);
                                    (t.cb.borrow_mut())(timer, monotonic, &mut *t.context.borrow_mut());
                                    if SHUTDOWN_STATE.load(Ordering::Acquire) == SHUTDOWN_COMPLETED {
                                        return TimerDriverResult {
                                            next_monotonic: u64::MAX,
                                            next_utc: u64::MAX,
                                        };
                                    }
                                }
                            } else {
                                next_mono = mono.1.0;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    loop {
                        if let Some(time) = env.timers_active_deadline.peek() {
                            if *time.1 > Reverse(utc) {
                                let timer = *time.0;
                                if let Some(t) = env.timers.get_mut(&timer) {
                                    t.state = TimerState::Idle;
                                    env.timers_active_deadline.pop();
                                    (runtime_api.set_task_id)(t.task);
                                    (t.cb.borrow_mut())(timer, utc, &mut *t.context.borrow_mut());
                                    if SHUTDOWN_STATE.load(Ordering::Acquire) == SHUTDOWN_COMPLETED {
                                        return TimerDriverResult {
                                            next_monotonic: u64::MAX,
                                            next_utc: u64::MAX,
                                        };
                                    }
                                }
                            } else {
                                next_time = time.1.0;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    TimerDriverResult {
                        next_monotonic: next_mono,
                        next_utc: next_time
                    }
                }
                _ => { unreachable!(); }
            }
        } else {
            unreachable!();
        }
    }

}

struct LibertasPackageEnv {
    timer_id: u32,
    trans_id: LibertasTransId,
    timers: HashMap<u32, Timer>,
    timers_active_interval: PriorityQueue<u32, Reverse<u64>, DefaultHashBuilder>,
    timers_active_deadline: PriorityQueue<u32, Reverse<u64>, DefaultHashBuilder>,
    contexts: HashMap<u32, Context>,    // Key is task ID
    package_callback: LibertasPackageCallback,
    wakeup_callbacks: Option<WakeupCallback>,
}

impl LibertasPackageEnv{
    #[inline(always)]
    fn new() -> Self {
        Self { 
            timer_id: 0, 
            trans_id:0, 
            timers: HashMap::new(), 
            timers_active_interval: PriorityQueue::with_default_hasher(), 
            timers_active_deadline: PriorityQueue::with_default_hasher(), 
            contexts: HashMap::new(), 
            package_callback: LibertasPackageCallback {
                version: CURRENT_VERSION,
                init_task: libertas_impl_init_task,
                remove_task: libertas_impl_remove_task,
                timer_driver: libertas_impl_timer_driver,
                device_callback: libertas_impl_device_callback,
            },
            wakeup_callbacks: None,
        }
    }

    #[inline(always)]
    fn init_task(&mut self, task_id: u32) {
        self.contexts.insert(task_id, Context::new());
    }

    #[inline(always)]
    fn remove_task(&mut self, task_id: u32) {
        self.contexts.remove(&task_id);
        self.timers.retain( |k, timer| {
            if *k == task_id {
                if let TimerState::ArmedInterval(_) = timer.state {
                    self.timers_active_interval.remove(k);
                } else if let TimerState::ArmedDeadline(_) = timer.state {
                    self.timers_active_deadline.remove(k);
                }
                return false;
            }
            true
        });
    }

    fn new_timer_id(&mut self) -> u32 {
        loop {
            self.timer_id += 1;
            if !self.timers.contains_key(&self.timer_id) {
                return self.timer_id;
            }
        }
    }

    /// Allocates a nonzero transaction ID for SDK-originated requests.
    ///
    /// Skipping zero is an allocator policy, not a universal sentinel rule for
    /// `LibertasTransId`. Protocol-specific APIs decide whether any value means
    /// "ignored."
    #[inline(always)]
    fn new_trans_id(&mut self) -> LibertasTransId {
        self.trans_id += 1;
        if self.trans_id == 0 {
            self.trans_id = 1;
        }
        self.trans_id
    }
}

static mut RUNTIME_API: *mut LibertasRuntimeApi = core::ptr::null_mut();
// The runtime API is initialized before App code starts and remains valid for
// the lifetime of the host process. Async shutdown completion reads it without
// touching the main-thread-only package environment.
static ASYNC_RUNTIME_API: AtomicPtr<LibertasRuntimeApi> = AtomicPtr::new(core::ptr::null_mut());
static SHUTDOWN_STATE: AtomicU8 = AtomicU8::new(SHUTDOWN_RUNNING);
static mut ENV: Option<LibertasPackageEnv> = None;

/**
 * Internal use only by Libertas SDK. The Libertas Apps are driven by a single thread. No locking is required.
 * This API is called only once as soon as the dynamic library is loaded by Libertas OS App engine. 
 * Attempting to call it again is considered a security attack and the caller App will be banned.
 * This function will be re-exported with a new unique name assigned by Libertas builder for each App package.
 */
#[doc(hidden)]
pub fn __libertas_init_package(runtime_api:*mut c_void) -> *const LibertasPackageCallback {
    unsafe {
        RUNTIME_API = runtime_api as *mut LibertasRuntimeApi;
        ASYNC_RUNTIME_API.store(RUNTIME_API, Ordering::Release);
        SHUTDOWN_STATE.store(SHUTDOWN_RUNNING, Ordering::Release);
        ENV = Some(LibertasPackageEnv::new());
        let env_ptr = &raw mut ENV;
        // We use .as_ref() on the *pointer* dereference, or better yet:
        if let Some(ref env) = *env_ptr {
            let callback_ptr = &raw const (env.package_callback);
            callback_ptr
        } else {
            core::ptr::null()
        }        
    }
}

/**
 * Called before dlclose call to unload the dynamic library.
 * This API shall be called only by the Libertas OS App engine. Apps calling the API will end up with being banned.
 * This function will be re-exported with a new unique name assigned by Libertas builder for each App package.
 */
#[doc(hidden)]
pub fn __libertas_release_package() {
    unsafe {
        ENV = None;     // Drop all memory
    }
}

/// Registers the task's shutdown handler.
///
/// The host calls this handler on the App main thread when shutdown is
/// requested. The handler may finish synchronously by calling
/// [`libertas_shutdown_complete`], or it may start asynchronous cleanup and
/// return. During asynchronous cleanup the App continues to operate normally.
///
/// Only one shutdown handler may be registered for a task.
pub fn libertas_register_shutdown_handler<F>(callback: F, context: Box<dyn Any>)
where
    F: FnMut(&mut Box<dyn Any>) + Sized + 'static,
{
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let task_id = libertas_get_task_id();
                let task_context = env.contexts.get_mut(&task_id).unwrap_or_else(|| {
                    panic!("libertas_register_shutdown_handler invalid task")
                });
                if task_context.shutdown_handler.is_some() {
                    panic!("Duplicate shutdown handler registered");
                }
                task_context.shutdown_handler = Some(ShutdownHandler {
                    cb: Rc::new(RefCell::new(Box::new(callback))),
                    context: Rc::new(RefCell::new(context)),
                });
            }
            _ => {
                unreachable!();
            }
        }
    }
}

/// Completes an App shutdown request.
///
/// This function may be called from any App thread. Before calling it, the App
/// must stop all other work that could call a Libertas API. The calling thread
/// must make no further Libertas API calls and return immediately after this
/// function returns.
///
/// The first call made after the host's shutdown request notifies the host.
/// Calls made before a request or duplicate completion calls are ignored.
pub fn libertas_shutdown_complete() {
    if SHUTDOWN_STATE.compare_exchange(
            SHUTDOWN_REQUESTED,
            SHUTDOWN_COMPLETED,
            Ordering::AcqRel,
            Ordering::Acquire).is_err() {
        return;
    }

    let runtime_api = ASYNC_RUNTIME_API.load(Ordering::Acquire);
    if runtime_api.is_null() {
        return;
    }
    unsafe {
        ((*runtime_api).device_send)(
            PROTOCOL_LIBERTAS,
            DEVICE_SYSTEM,
            0,
            OP_APP_SHUT_DOWN,
            core::ptr::null(),
            0,
            0);
    }
}

/// Wake up the main thread loop. This function is usually called from another thread, for example, after sending to the main thread.
/// 
/// The callback registered by [`libertas_register_wakeup_callback`] will be called the first when main thread wakes up.
/// 
/// Multi-threading communication with main thread must be non-blocking on both sides. If the sender from another thread is blocking, 
/// it will not have a chance to call [`libertas_wake_up`]. The wake up callback implementation must ensure both sending and receiving operations are non-blocking.
/// 
pub fn libertas_wake_up() {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.device_send)(PROTOCOL_LIBERTAS, DEVICE_SYSTEM, 0, OP_SYSTEM_WAKE_UP, core::ptr::null(), 0, 0);
        } else {
            unreachable!();
        }
    }
}

/// Generates random bytes from the system RNG.
/// 
/// # Arguments
/// * `bytes` - Number of random bytes to generate (1-8).
/// 
/// # Returns
/// Random data in the least significant bits of a u64.
/// 
/// # Notes
/// Only the lower `bytes` are valid. May call RNG multiple times for embedded systems.
pub fn libertas_get_random(bytes: u8) -> u64 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            return (runtime_api.get_random)(bytes);
        } else {
            unreachable!();
        }
    }
}

/// Returns the current monotonic system tick count in microseconds.
/// Useful for measuring elapsed time and timer expiration.
pub fn libertas_get_sys_ticks() -> u64 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            return (runtime_api.get_sys_ticks)();
        } else {
            unreachable!();
        }
    }
}

/// Returns current UTC time in microseconds since Unix epoch.
/// Returns `None` if UTC is unavailable (e.g., no RTC or network time).
pub fn libertas_get_utc_time() -> Option<u64> {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            let ret = (runtime_api.get_utc_time)();
            return if ret == 0 { None } else { Some(ret) };
        } else {
            unreachable!();
        }
    }
}

/// Returns current local time in microseconds since Unix epoch.
/// Applies timezone offset to UTC. Returns `None` if UTC unavailable.
pub fn libertas_get_local_time() -> Option<u64> {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            let utc = (runtime_api.get_utc_time)();
            if utc == 0 {
                return None;
            }
            Some((runtime_api.utc_to_local)(utc as i64) as u64)
        } else {
            unreachable!();
        }
    }
}

/// Converts local time to UTC by applying timezone offset.
/// Accurate within the next DST cycle (typically >1 year).
pub fn libertas_local_time_to_utc(local: i64) -> i64 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.local_to_utc)(local)
        } else {
            unreachable!();
        }
    }
}

/// Converts UTC time to local time by applying timezone offset.
/// Accurate within the next DST cycle (typically >1 year).
pub fn libertas_utc_time_to_local(utc: i64) -> i64 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.utc_to_local)(utc)
        } else {
            unreachable!();
        }
    }
}

/// Returns the current task's unique identifier.
pub fn libertas_get_task_id() -> u32 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            return (runtime_api.get_task_id)();
        } else {
            unreachable!();
        }
    }
}

fn libertas_timer_new(expiration: u64, interval: bool, callback: Box<LibertasTimerCallback>, context: Box<dyn Any>) -> u32 {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let task_id = libertas_get_task_id();
                let id = env.new_timer_id();
                let wrapped_cb = Rc::new(RefCell::new(callback));
                let wrapped_tag = Rc::new(RefCell::new(context));
                if expiration > 0 {
                    if interval {
                        env.timers.insert(id, Timer {
                            task: task_id,
                            state: TimerState::ArmedInterval(expiration),
                            cb: wrapped_cb,
                            context: wrapped_tag,
                        });
                        env.timers_active_interval.push(id, Reverse(expiration));
                    } else {
                        env.timers.insert(id, Timer {
                            task: task_id,
                            state: TimerState::ArmedDeadline(expiration),
                            cb: wrapped_cb,
                            context: wrapped_tag,
                        });
                        env.timers_active_deadline.push(id, Reverse(expiration));
                    }
                } else {
                    env.timers.insert(id, Timer {
                        task: task_id,
                        state: TimerState::Idle,
                        cb: wrapped_cb,
                        context: wrapped_tag,
                    });
                }
                id
            }
            _ => { unreachable!(); }
        }
    }
}

/// Creates an interval timer that fires at the specified system tick.
/// 
/// # Arguments
/// * `expiration` - Expiration time in system ticks (microseconds).
/// * `callback` - Called on fire with (timer_id, current_ticks, context).
/// * `context` - User data passed to callback.
/// 
/// # Returns
/// Timer ID for management.
pub fn libertas_timer_new_interval<F>(expiration: u64, callback: F, context: Box<dyn Any>) -> u32 where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    return libertas_timer_new(expiration, true, Box::new(callback), context);
}

/// Creates a deadline timer that fires at the specified UTC time.
/// 
/// # Arguments
/// * `expiration` - Deadline in UTC microseconds.
/// * `callback` - Called on fire with (timer_id, deadline_time, context).
/// * `context` - User data passed to callback.
/// 
/// # Returns
/// Timer ID for management.
pub fn libertas_timer_new_deadline<F>(expiration: u64, callback: F, context: Box<dyn Any>) -> u32 where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    return libertas_timer_new(expiration, false, Box::new(callback), context);
}

fn libertas_timer_update(timer: u32, expiration: u64, interval: bool, callback: Option<Box<LibertasTimerCallback>>, new_context: Option<Box<dyn Any>>) {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                if let Some(t) = env.timers.get_mut(&timer) {
                    if t.task != libertas_get_task_id() {
                        panic!("libertas_timer_update task_id");
                    }
                    if interval {
                        if let TimerState::ArmedInterval(old_exp) = t.state {
                            if old_exp != expiration {
                                t.state = TimerState::ArmedInterval(expiration);
                                env.timers_active_interval.change_priority(&timer, Reverse(expiration));
                            }
                        } else {
                            if let TimerState::ArmedDeadline(_) = t.state {
                                env.timers_active_deadline.remove(&timer);
                            }
                            t.state = TimerState::ArmedInterval(expiration);
                            env.timers_active_interval.push(timer, Reverse(expiration));
                        }
                    } else {
                        if let TimerState::ArmedDeadline(old_exp) = t.state {
                            if old_exp != expiration {
                                t.state = TimerState::ArmedDeadline(expiration);
                                env.timers_active_deadline.change_priority(&timer, Reverse(expiration));
                            }
                        } else {
                            if let TimerState::ArmedInterval(_) = t.state {
                                env.timers_active_interval.remove(&timer);
                            }
                            t.state = TimerState::ArmedDeadline(expiration);
                            env.timers_active_deadline.push(timer, Reverse(expiration));
                        }
                    }
                    if let Some(new_cb) = callback {
                        t.cb = Rc::new(RefCell::new(new_cb));
                    }
                    if let Some(context) = new_context {
                        t.context = Rc::new(RefCell::new(context));
                    }
                } else {
                    panic!("libertas_timer_update invalid timer");
                }
            }
            _ => { unreachable!(); }
        }
    }
}

/// Updates an interval timer's expiration time.
/// 
/// # Arguments
/// * `timer` - Timer ID.
/// * `expiration` - New expiration in system ticks.
/// 
/// # Panics
/// If timer invalid or not owned by current task.
pub fn libertas_timer_update_interval(timer: u32, expiration: u64) {
    libertas_timer_update(timer, expiration, true, None, None);
}

/// Updates a deadline timer's deadline.
/// 
/// # Arguments
/// * `timer` - Timer ID.
/// * `expiration` - New deadline in UTC microseconds.
/// 
/// # Panics
/// If timer invalid or not owned by current task.
pub fn libertas_timer_update_deadline(timer: u32, expiration: u64) {
    libertas_timer_update(timer, expiration, false, None, None);
}

/// Updates an interval timer with new expiration and callback.
/// 
/// # Arguments
/// * `timer` - Timer ID.
/// * `expiration` - New expiration in system ticks.
/// * `callback` - New callback closure.
/// * `new_context` - New user data.
/// 
/// # Panics
/// If timer invalid or not owned by current task.
pub fn libertas_timer_update_interval_callback<F>(timer: u32, expiration: u64, callback: F, new_context: Box<dyn Any>) where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    libertas_timer_update(timer, expiration, true, Some(Box::new(callback)), Some(new_context));
}

/// Updates a deadline timer with new deadline and callback.
/// 
/// # Arguments
/// * `timer` - Timer ID.
/// * `expiration` - New deadline in UTC microseconds.
/// * `callback` - New callback closure.
/// * `new_context` - New user data.
/// 
/// # Panics
/// If timer invalid or not owned by current task.
pub fn libertas_timer_update_deadline_callback<F>(timer: u32, expiration: u64, callback: F, new_context: Box<dyn Any>) where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    libertas_timer_update(timer, expiration, false, Some(Box::new(callback)), Some(new_context));
}

/// Cancels a running timer without deleting it.
/// Timer can be restarted later.
/// 
/// # Arguments
/// * `timer` - Timer ID.
/// 
/// # Panics
/// If timer invalid or not owned by current task.
pub fn libertas_timer_cancel(timer: u32) {  
    unsafe {
        match ENV {
            Some(ref mut env) => {
                if let Some(t) = env.timers.get_mut(&timer) {
                    if t.task != libertas_get_task_id() {
                        panic!("timer_cancel task_id");
                    }
                    if let TimerState::ArmedInterval(_) = t.state {
                        t.state = TimerState::Idle;
                        env.timers_active_interval.remove(&timer);
                    } else if let TimerState::ArmedDeadline(_) = t.state {
                        t.state = TimerState::Idle;
                        env.timers_active_deadline.remove(&timer);
                    }
                } else {
                    panic!("timer_cancel invalid timer");
                }
            }
            _ => { unreachable!(); }
        }
    }
}

/// Destroys a timer and frees its resources.
/// Timer cannot be used again.
/// 
/// # Arguments
/// * `timer` - Timer ID.
/// 
/// # Panics
/// If timer invalid or not owned by current task.
pub fn libertas_timer_destroy(timer: u32) {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                if let Some(t) = env.timers.get_mut(&timer) {
                    if t.task != libertas_get_task_id() {
                        panic!("timer_delete task_id");
                    }
                    if let TimerState::ArmedInterval(_) = t.state {
                        env.timers_active_interval.remove(&timer);
                    } else if let TimerState::ArmedDeadline(_) = t.state {
                        env.timers_active_deadline.remove(&timer);
                    }
                    env.timers.remove(&timer);
                } else {
                    panic!("timer_destroy invalid timer");
                }
            }
            _ => { unreachable!(); }
        }
    }    
}

fn libertas_register_device_listener_impl(device: LibertasDevice, callback: Box<LibertasDeviceCallback>, tag: Box<dyn Any>) {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let task_id = libertas_get_task_id();
                let wrapped_cb = Rc::new(RefCell::new(callback));
                let wrapped_tag = Rc::new(RefCell::new(tag));                
                if let Some(context) = env.contexts.get_mut(&task_id) {
                    if let Some(_v) = context.device_callbacks.insert(device, DeviceCallback { cb: wrapped_cb, context: wrapped_tag }) {
                        panic!("Duplicate device callback registered");
                    }
                } else {
                    unreachable!();
                }
            }
            _ => { unreachable!(); }
        }
    }
}

/// Registers a callback function for device events
///
/// Registers a callback that will be invoked when events occur on the specified device.
/// The callback receives device responses, attribute updates, and other device-related events.
/// 
/// # Remarks
/// Developer shall not call this API directly. Instead, they should use the protocol-specific APIs built on top of this API. For example, for Matter protocol, developers should use `matter_register_device_callback` instead of this API. The protocol-specific APIs will handle the protocol details and provide a more user-friendly interface.
///
/// # Arguments
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasEndpoint`.
/// * `callback` - Closure called when device events occur. Receives device ID, operation code, event data, mutable context, transaction ID, and peer ID.
/// * `context` - Developer-defined data passed to the callback function
/// 
/// # Remarks
/// The transaction ID is always supplied. Libertas does not interpret any
/// transaction ID as absent; the interaction protocol may designate and
/// document a value that applications should ignore.
///
/// Only one callback can be registered per device. Registering a new callback for the same device will cause panic. For devices that the task doesn't
/// have "read access", only responses to its own requests will trigger the callback. App can not subscribe from "write-only" devices.
/// 
/// A task is advised to collect all devices from function arguments before registering any callback to eliminate potential duplicates.
/// 
#[doc(hidden)]
#[inline(always)]
pub fn libertas_register_device_listener<F>(device: LibertasDevice, callback: F, context: Box<dyn Any>) where F: FnMut(LibertasDevice, u8, &[u8], &mut Box<dyn Any>, LibertasTransId, u32) + Sized + 'static {
    libertas_register_device_listener_impl(device, Box::new(callback), context);
}

/// Registers a callback function for wakeup events. The callback will be called first thing after the task is waken up before timers are processed.
/// 
/// All multi-threadng communication with main thread must be performed in this callback.
/// 
/// This callback must be non-blocking on both sending and receiving! Read [`libertas_wake_up`] for more details.
/// 
/// # Arguments
/// * `callback` - Closure called on wakeup with mutable context.
/// * `context` - Developer-defined data passed to the callback function.
/// 
pub fn libertas_register_wakeup_callback<F>(callback: F, context: Box<dyn Any>) where F: FnMut(&mut Box<dyn Any>) + Sized + 'static {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                env.wakeup_callbacks = Some(WakeupCallback {
                    cb: Rc::new(RefCell::new(Box::new(callback))),
                    context: Rc::new(RefCell::new(context)),
                });
            }
            _ => { unreachable!(); }
        }
    }
}

type EndpointCallback<T> = dyn FnMut(
    LibertasEndpoint,
    u8,
    Option<T>,
    &mut Box<dyn Any>,
    LibertasTransId,
    u32,
) -> LibertasEndpointStatus;
struct EndpointListener<T> {
    callback: Box<EndpointCallback<T>>,
    context: Box<dyn Any>,
}

fn libertas_endpoint_send_invalid_message(
    server: LibertasEndpoint,
    opcode: u8,
    trans_id: LibertasTransId,
    peer: u32,
) {
    let status = [LibertasEndpointStatus::InvalidMessage as u8];
    libertas_device_send_response(
        PROTOCOL_LIBERTAS,
        server,
        OP_ENDPOINT_RSP,
        &status,
        trans_id,
        peer,
    );
    if opcode == OP_ENDPOINT_SUB_REQ {
        libertas_endpoint_remove_subscriber(server, peer);
    }
}

/// Registers a callback for endpoint events.
/// 
/// # Arguments
/// * `id` - Endpoint device ID.
/// * `callback` - Called with (device, opcode, decoded_data, context, trans_id, peer).
///   It must return [`LibertasEndpointStatus::Success`] after handling the
///   message, or [`LibertasEndpointStatus::InvalidMessage`] to reject a decoded
///   request or subscription request.
/// * `context` - User data.
/// 
/// The endpoint protocol always supplies `trans_id` and designates
/// [`LIBERTAS_ENDPOINT_IGNORED_TRANS_ID`] for interactions that do not correlate
/// with a transaction.
///
/// Payload byte zero is the platform status. A `Success` status must be
/// followed by exactly one Avro value of type `T`; malformed, truncated, or
/// trailing data never reaches the callback. Invalid requests are answered
/// automatically with `InvalidMessage`. Invalid responses and reports are
/// surfaced as `None` without sending another message, which prevents response
/// loops. Peer-down and peer-timeout notifications also carry `None`.
pub fn libertas_register_endpoint_listener<T, F>(id: LibertasEndpoint, callback: F, context: Box<dyn Any>)
        where 
            T: AvroDecode + 'static,
            F: FnMut(
                LibertasEndpoint,
                u8,
                Option<T>,
                &mut Box<dyn Any>,
                LibertasTransId,
                u32,
            ) -> LibertasEndpointStatus + Sized + 'static {
    let listener = Box::new(EndpointListener {
        callback: Box::new(callback),
        context,
    });
    libertas_register_device_listener(id, |device, opcode, data, tag_any, trans_id, peer| {
        let tag = tag_any.downcast_mut::<EndpointListener<T>>().unwrap();

        if opcode == OP_ENDPOINT_PEER_DOWN || opcode == OP_ENDPOINT_PEER_TIMEOUT {
            (tag.callback)(device, opcode, None, &mut tag.context, trans_id, peer);
            return;
        }

        let protocol_obj = if data.first().copied()
                == Some(LibertasEndpointStatus::Success as u8) {
            let mut offset = 1;
            match T::avro_decode(data, &mut offset) {
                Ok(value) if offset == data.len() => Some(value),
                _ => None,
            }
        } else {
            None
        };

        if protocol_obj.is_none() && (opcode == OP_ENDPOINT_REQ || opcode == OP_ENDPOINT_SUB_REQ) {
            libertas_endpoint_send_invalid_message(device, opcode, trans_id, peer);
            return;
        }

        let status = (tag.callback)(
            device,
            opcode,
            protocol_obj,
            &mut tag.context,
            trans_id,
            peer,
        );
        if status == LibertasEndpointStatus::InvalidMessage &&
                (opcode == OP_ENDPOINT_REQ || opcode == OP_ENDPOINT_SUB_REQ) {
            libertas_endpoint_send_invalid_message(device, opcode, trans_id, peer);
        }
    }, listener);
}

fn __libertas_device_send_raw(protocol: u16, device: LibertasDevice, op: u8, peer: u32, trans_id: LibertasTransId, data: *const u8, data_len: usize) {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.device_send)(protocol, device, trans_id, op, data, data_len, peer);
        } else {
            unreachable!();
        }
    }    
}

fn __libertas_device_read_raw(protocol: u16, device: LibertasDevice, op: u8, data: *const u8, data_len: usize) -> ReadResult{
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.device_read)(protocol, device, op, data, data_len)
        } else {
            unreachable!();
        }
    }
}

#[doc(hidden)]
pub fn __libertas_device_send_raw_req(protocol: u16, device: LibertasDevice, op: u8, peer: u32, data: *const u8, data_len: usize) -> LibertasTransId {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let ret = env.new_trans_id();
                if let Some(runtime_api) = RUNTIME_API.as_ref() {
                    (runtime_api.device_send)(protocol, device, ret, op, data, data_len, peer);
                } else {
                    unreachable!();
                }
                return ret;
            }
            _ => { unreachable!(); }
        }
    }    
}

/// Sends a request to a LibertasDevice. A response is always expected as a transaction. Thus, a unique transaction ID is generated and returned.
/// 
/// # Remarks
/// Developers shall not call this API directly. Instead, they should use the protocol-specific APIs built on top of this API. For example, for Matter protocol, developers should use `matter_send_invoke_request` instead of this API. The protocol-specific APIs will handle the protocol details and provide a more user-friendly interface.
/// 
/// # Arguments
/// * `protocol` - The protocol ID for the request.
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasEndpoint`.
/// * `op` - Operation code for the request. This is an application defined.
/// * `data` - Binary blob containing the request data. The content is application defined.
/// 
/// # Returns
/// Unique transaction ID for tracking the response
/// 
#[doc(hidden)]
#[inline(always)]
pub fn libertas_device_send_request(protocol: u16, device: LibertasDevice, op: u8, data: &[u8]) -> LibertasTransId {
    __libertas_device_send_raw_req(protocol, device, op, 0, data.as_ptr(), data.len())
}

/// Sends a response to a device request. This API is used to respond to a request received from a device, using the transaction ID provided in the request callback.
/// 
/// # Remarks
/// Developers shall not call this API directly. Instead, they should use the protocol-specific APIs built on top of this API. For example, for Matter protocol, developers should use `matter_send_invoke_response` instead of this API. The protocol-specific APIs will handle the protocol details and provide a more user-friendly interface.
/// 
/// # Arguments
/// * `protocol` - The protocol ID for the request.
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasEndpoint`.
/// * `op` - Operation code for the response. This is an application defined and typically defined in relation to the request op code.
/// * `data` - Binary blob containing the response data
/// * `trans_id` - Transaction ID from the original request to correlate the response.
/// * `peer` - The peer that sent the original request.
/// 
#[doc(hidden)]
#[inline(always)]
pub fn libertas_device_send_response(protocol: u16, device: LibertasDevice, op: u8, data: &[u8], trans_id: LibertasTransId, peer: u32) {
    __libertas_device_send_raw(protocol, device, op, peer, trans_id, data.as_ptr(), data.len());
}

/// Sends a report to a device without expecting a response. This API can be used for unsolicited updates to the device.
/// 
/// # Remarks
/// Developers shall not call this API directly. Instead, they should use the protocol-specific APIs built on top of this API. For example, for Matter protocol, developers should use `matter_send_report` instead of this API. The protocol-specific APIs will handle the protocol details and provide a more user
/// 
/// # Arguments
/// * `protocol` - The protocol ID for the request.
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasEndpoint`.
/// * `op` - Operation code for the report. This is an application defined.
/// * `data` - Binary blob containing the report data.
/// * `trans_id` - Transaction ID required by the interaction protocol. The
///   protocol may designate a special value as ignored for unsolicited reports.
/// * `peer` - The peer that sent the original request.
/// 
#[doc(hidden)]
pub fn libertas_device_send_report(protocol: u16, device: LibertasDevice, op: u8, data: &[u8], trans_id: LibertasTransId, peer: u32) {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.device_send)(protocol, device, trans_id, op, data.as_ptr(), data.len(), peer);
        } else {
            unreachable!();
        }
    }
}

/// Sends a endpoint request to a server
///
/// Sends a endpoint request to the specified server with the given operation code
/// and data payload. Returns a transaction ID for tracking the response.
///
/// # Arguments
/// * `server` - A endpoint server. The link created when a Libertas machine is created.
/// * `data` - The data payload to send with the request
///
/// # Returns
/// Transaction ID that can be used to correlate responses from the device
/// 
/// # Note
/// Unlike Matter protocol, the data can be any data structure defined and published by the server developer, encoded with Apache Avro format.
/// We don't need any other Matter interactions. For example, Read and Write can all be implemented as InvokeRequest with custom data. We 
/// even included default request in the standard like in HTTP.
#[inline(always)]
pub fn libertas_endpoint_request<T>(server: LibertasEndpoint, data: &T) -> LibertasTransId
    where 
        T: AvroEncode + 'static {
    let mut bytes: Vec<_> = Vec::new();
    bytes.push(LibertasEndpointStatus::Success as u8);
    data.avro_encode(&mut bytes);
    libertas_device_send_request(PROTOCOL_LIBERTAS, server, OP_ENDPOINT_REQ, &bytes)
}

/// Sends a endpoint request to a server
///
/// Sends a endpoint request to the specified server with the given operation code
/// and data payload. Returns a transaction ID for tracking the response.
///
/// # Arguments
/// * `server` - A endpoint server. The link created when a Libertas machine is created.
/// * `data` - The data payload to send with the request
///
/// # Returns
/// Transaction ID that can be used to correlate responses from the device
/// 
/// # Note
/// Unlike Matter protocol, the data can be any data structure defined and published by the server developer, encoded with Apache Avro format.
/// We don't need any other Matter interactions. For example, Read and Write can all be implemented as InvokeRequest with custom data. We 
/// even included default request in the standard like in HTTP.
#[inline(always)]
pub fn libertas_endpoint_subscribe_request<T>(server: LibertasEndpoint, data: &T) -> LibertasTransId
    where 
        T: AvroEncode + 'static {
    let mut bytes: Vec<_> = Vec::new();
    bytes.push(LibertasEndpointStatus::Success as u8);
    data.avro_encode(&mut bytes);
    libertas_device_send_request(PROTOCOL_LIBERTAS, server, OP_ENDPOINT_SUB_REQ, &bytes)
}

/// Sends a endpoint response to a device
///
/// Sends a response to a endpoint request with the specified operation code
/// and data payload using the provided transaction ID.
///
/// # Arguments
/// * `server` - A endpoint server id. The link created when a Libertas machine is created.
/// * `data` - The data payload to send with the response
/// * `trans_id` - The transaction ID from the original request
/// * `dest` - The source of the original request as the destination of the request.
///
/// # Note
/// Unlike Matter protocol, the data can be any data structure defined and published by the server developer, encoded with Apache Avro format.
#[inline(always)]
pub fn libertas_endpoint_response<T>(server: LibertasEndpoint, data: &T, trans_id: LibertasTransId, dest: u32)
    where 
        T: AvroEncode + 'static {
    let mut bytes: Vec<_> = Vec::new();
    bytes.push(LibertasEndpointStatus::Success as u8);
    data.avro_encode(&mut bytes);
    libertas_device_send_response(PROTOCOL_LIBERTAS, server, OP_ENDPOINT_RSP, &bytes, trans_id, dest);
}

/// Sends a endpoint report to subscribers
///
/// Sends report data to all subscribers of a endpoint point or to a specific device.
/// This is typically used to push data updates to clients that have subscribed to data changes.
///
/// # Arguments
/// * `server` - A endpoint server id. The link created when a Libertas machine is created.
/// * `data` - The data payload to send in the report
/// * `peer` - Optional specific device ID to send the report to. If None, broadcasts to all subscribers.
///
/// # Note
/// Unlike Matter protocol, the data can be any data structure defined and published by the server developer, encoded with Apache Avro format.
/// This function is used to fulfill subscription requests that expect `OpCode::ReportData` messages.
/// The endpoint protocol uses [`LIBERTAS_ENDPOINT_IGNORED_TRANS_ID`] because
/// unsolicited reports do not correlate with a transaction.
#[inline(always)]
pub fn libertas_endpoint_report<T>(server: LibertasEndpoint, data: &T, peer: Option<LibertasEndpoint>) 
    where 
        T: AvroEncode + 'static {
    let peer_id = peer.unwrap_or(LIBERTAS_BROADCAST_DEST);
    let mut bytes: Vec<_> = Vec::new();
    bytes.push(LibertasEndpointStatus::Success as u8);
    data.avro_encode(&mut bytes);
    libertas_device_send_report(PROTOCOL_LIBERTAS, server, OP_ENDPOINT_DATA, &bytes, LIBERTAS_ENDPOINT_IGNORED_TRANS_ID, peer_id);
}

/// Remove a subscriber. 
/// 
/// Because the protocol can be anything designed by the developer. There is
/// no way for Libertas OS runtime to know whether a subscribe request has failed. After the 
/// task sends back a reject response, it has to notify the OS runtime to remove the subscriber
/// from the OS maintained list.
/// * A broadcast data report will nopt be send to this peer.
/// * OS will not try to maintain the session to keep it alive.
/// 
/// # Arguments
/// * `server` - A endpoint server id. The link created when a Libertas machine is created.
/// * `peer` - The subscriber peer.
/// 
#[inline(always)]
pub fn libertas_endpoint_remove_subscriber(server: LibertasEndpoint, peer: u32) {
    let empty_slice: &[u8] = &[];
    libertas_device_send_response(PROTOCOL_LIBERTAS, server, OP_ENDPOINT_REMOVE_PEER, empty_slice, LIBERTAS_ENDPOINT_IGNORED_TRANS_ID, peer);
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use std::{sync::Mutex, thread};

    static TEST_LOCK: Mutex<()> = Mutex::new(());
    static SHUTDOWN_SENDS: AtomicUsize = AtomicUsize::new(0);
    static SHUTDOWN_HANDLERS: AtomicUsize = AtomicUsize::new(0);
    static ENDPOINT_CALLBACKS_WITH_DATA: AtomicUsize = AtomicUsize::new(0);
    static ENDPOINT_CALLBACKS_WITHOUT_DATA: AtomicUsize = AtomicUsize::new(0);
    static ENDPOINT_INVALID_RESPONSES: AtomicUsize = AtomicUsize::new(0);
    static ENDPOINT_REMOVALS: AtomicUsize = AtomicUsize::new(0);
    static ENDPOINT_LAST_DEVICE: AtomicUsize = AtomicUsize::new(0);
    static ENDPOINT_LAST_TRANS_ID: AtomicUsize = AtomicUsize::new(0);
    static ENDPOINT_LAST_PEER: AtomicUsize = AtomicUsize::new(0);

    extern "C" fn fake_get_random(_: u8) -> u64 { 0 }
    extern "C" fn fake_get_task_id() -> u32 { 7 }
    extern "C" fn fake_set_task_id(_: u32) {}
    extern "C" fn fake_get_time() -> u64 { 0 }
    extern "C" fn fake_convert_time(value: i64) -> i64 { value }
    extern "C" fn fake_device_send(
        protocol: u16,
        device: u32,
        trans_id: LibertasTransId,
        op_code: u8,
        data: *const u8,
        data_len: usize,
        peer: u32,
    ) {
        if protocol == PROTOCOL_LIBERTAS &&
                device == DEVICE_SYSTEM &&
                op_code == OP_APP_SHUT_DOWN {
            SHUTDOWN_SENDS.fetch_add(1, Ordering::AcqRel);
        } else if protocol == PROTOCOL_LIBERTAS && op_code == OP_ENDPOINT_RSP {
            if data_len == 1 && !data.is_null() &&
                    unsafe { *data } == LibertasEndpointStatus::InvalidMessage as u8 {
                ENDPOINT_INVALID_RESPONSES.fetch_add(1, Ordering::AcqRel);
                ENDPOINT_LAST_DEVICE.store(device as usize, Ordering::Release);
                ENDPOINT_LAST_TRANS_ID.store(trans_id as usize, Ordering::Release);
                ENDPOINT_LAST_PEER.store(peer as usize, Ordering::Release);
            }
        } else if protocol == PROTOCOL_LIBERTAS && op_code == OP_ENDPOINT_REMOVE_PEER {
            ENDPOINT_REMOVALS.fetch_add(1, Ordering::AcqRel);
        }
    }
    extern "C" fn fake_device_read(
        _: u16,
        _: u32,
        _: u8,
        _: *const u8,
        _: usize,
    ) -> ReadResult {
        ReadResult {
            success: false,
            data: core::ptr::null(),
            data_len: 0,
        }
    }

    fn fake_runtime_api() -> LibertasRuntimeApi {
        LibertasRuntimeApi {
            version: CURRENT_VERSION,
            get_random: fake_get_random,
            get_task_id: fake_get_task_id,
            set_task_id: fake_set_task_id,
            get_sys_ticks: fake_get_time,
            get_utc_time: fake_get_time,
            utc_to_local: fake_convert_time,
            local_to_utc: fake_convert_time,
            device_send: fake_device_send,
            device_read: fake_device_read,
        }
    }

    #[test]
    fn shutdown_defaults_to_immediate_and_supports_async_completion() {
        let _guard = TEST_LOCK.lock().unwrap();
        SHUTDOWN_SENDS.store(0, Ordering::Release);
        SHUTDOWN_HANDLERS.store(0, Ordering::Release);

        let mut runtime_api = fake_runtime_api();
        let callbacks = __libertas_init_package(
            &raw mut runtime_api as *mut LibertasRuntimeApi as *mut c_void);
        unsafe {
            ((*callbacks).init_task)(7);
            ((*callbacks).device_callback)(
                7,
                DEVICE_SYSTEM,
                OP_APP_SHUT_DOWN,
                0,
                core::ptr::null(),
                0,
                0);
        }
        assert_eq!(SHUTDOWN_SENDS.load(Ordering::Acquire), 1);
        assert_eq!(SHUTDOWN_STATE.load(Ordering::Acquire), SHUTDOWN_COMPLETED);

        // Duplicate Host requests and App completions are idempotent.
        unsafe {
            ((*callbacks).device_callback)(
                7,
                DEVICE_SYSTEM,
                OP_APP_SHUT_DOWN,
                0,
                core::ptr::null(),
                0,
                0);
        }
        libertas_shutdown_complete();
        assert_eq!(SHUTDOWN_SENDS.load(Ordering::Acquire), 1);
        __libertas_release_package();

        let callbacks = __libertas_init_package(
            &raw mut runtime_api as *mut LibertasRuntimeApi as *mut c_void);
        unsafe {
            ((*callbacks).init_task)(7);
        }
        libertas_register_shutdown_handler(
            |_| {
                SHUTDOWN_HANDLERS.fetch_add(1, Ordering::AcqRel);
            },
            Box::new(()));
        unsafe {
            ((*callbacks).device_callback)(
                7,
                DEVICE_SYSTEM,
                OP_APP_SHUT_DOWN,
                0,
                core::ptr::null(),
                0,
                0);
        }
        assert_eq!(SHUTDOWN_HANDLERS.load(Ordering::Acquire), 1);
        assert_eq!(SHUTDOWN_SENDS.load(Ordering::Acquire), 1);
        assert_eq!(SHUTDOWN_STATE.load(Ordering::Acquire), SHUTDOWN_REQUESTED);

        thread::spawn(libertas_shutdown_complete).join().unwrap();
        assert_eq!(SHUTDOWN_SENDS.load(Ordering::Acquire), 2);
        assert_eq!(SHUTDOWN_STATE.load(Ordering::Acquire), SHUTDOWN_COMPLETED);

        let timer_result = unsafe {
            ((*callbacks).timer_driver)(0, 0)
        };
        assert_eq!(timer_result.next_monotonic, u64::MAX);
        assert_eq!(timer_result.next_utc, u64::MAX);
        __libertas_release_package();
    }

    #[test]
    fn endpoint_listener_rejects_invalid_messages_without_panicking_or_looping() {
        let _guard = TEST_LOCK.lock().unwrap();
        const ENDPOINT: LibertasEndpoint = 41;
        const PEER: u32 = 73;

        ENDPOINT_CALLBACKS_WITH_DATA.store(0, Ordering::Release);
        ENDPOINT_CALLBACKS_WITHOUT_DATA.store(0, Ordering::Release);
        ENDPOINT_INVALID_RESPONSES.store(0, Ordering::Release);
        ENDPOINT_REMOVALS.store(0, Ordering::Release);

        let mut runtime_api = fake_runtime_api();
        let callbacks = __libertas_init_package(
            &raw mut runtime_api as *mut LibertasRuntimeApi as *mut c_void);
        unsafe {
            ((*callbacks).init_task)(7);
        }
        libertas_register_endpoint_listener::<bool, _>(
            ENDPOINT,
            |_, _, data, _, _, _| {
                match data {
                    Some(true) => {
                        ENDPOINT_CALLBACKS_WITH_DATA.fetch_add(1, Ordering::AcqRel);
                        LibertasEndpointStatus::Success
                    }
                    Some(false) => {
                        ENDPOINT_CALLBACKS_WITH_DATA.fetch_add(1, Ordering::AcqRel);
                        LibertasEndpointStatus::InvalidMessage
                    }
                    None => {
                        ENDPOINT_CALLBACKS_WITHOUT_DATA.fetch_add(1, Ordering::AcqRel);
                        LibertasEndpointStatus::Success
                    }
                }
            },
            Box::new(()),
        );

        let deliver = |opcode: u8, trans_id: LibertasTransId, data: &[u8]| unsafe {
            ((*callbacks).device_callback)(
                7,
                ENDPOINT,
                opcode,
                trans_id,
                data.as_ptr() as *const c_void,
                data.len(),
                PEER,
            );
        };

        // Invalid Avro and semantic rejection both produce InvalidMessage.
        deliver(OP_ENDPOINT_REQ, 101, &[LibertasEndpointStatus::Success as u8, 2]);
        deliver(OP_ENDPOINT_REQ, 102, &[LibertasEndpointStatus::Success as u8, 0]);
        assert_eq!(ENDPOINT_INVALID_RESPONSES.load(Ordering::Acquire), 2);
        assert_eq!(ENDPOINT_CALLBACKS_WITH_DATA.load(Ordering::Acquire), 1);

        // A rejected subscription is also removed from the host subscriber set.
        deliver(OP_ENDPOINT_SUB_REQ, 103, &[LibertasEndpointStatus::Success as u8]);
        assert_eq!(ENDPOINT_INVALID_RESPONSES.load(Ordering::Acquire), 3);
        assert_eq!(ENDPOINT_REMOVALS.load(Ordering::Acquire), 1);
        assert_eq!(ENDPOINT_LAST_DEVICE.load(Ordering::Acquire), ENDPOINT as usize);
        assert_eq!(ENDPOINT_LAST_TRANS_ID.load(Ordering::Acquire), 103);
        assert_eq!(ENDPOINT_LAST_PEER.load(Ordering::Acquire), PEER as usize);

        // Invalid responses and reports are surfaced without answering the peer.
        deliver(
            OP_ENDPOINT_RSP,
            104,
            &[LibertasEndpointStatus::InvalidMessage as u8],
        );
        deliver(OP_ENDPOINT_DATA, 0, &[LibertasEndpointStatus::Success as u8, 2]);
        assert_eq!(ENDPOINT_CALLBACKS_WITHOUT_DATA.load(Ordering::Acquire), 2);
        assert_eq!(ENDPOINT_INVALID_RESPONSES.load(Ordering::Acquire), 3);

        // Peer status has no Avro body, including an empty/null FFI payload.
        unsafe {
            ((*callbacks).device_callback)(
                7,
                ENDPOINT,
                OP_ENDPOINT_PEER_TIMEOUT,
                0,
                core::ptr::null(),
                0,
                PEER,
            );
        }
        assert_eq!(ENDPOINT_CALLBACKS_WITHOUT_DATA.load(Ordering::Acquire), 3);

        // Valid data is decoded, while trailing bytes and unknown statuses reject.
        deliver(OP_ENDPOINT_RSP, 105, &[LibertasEndpointStatus::Success as u8, 1]);
        deliver(
            OP_ENDPOINT_REQ,
            106,
            &[LibertasEndpointStatus::Success as u8, 1, 0],
        );
        deliver(OP_ENDPOINT_REQ, 107, &[2]);
        assert_eq!(ENDPOINT_CALLBACKS_WITH_DATA.load(Ordering::Acquire), 2);
        assert_eq!(ENDPOINT_INVALID_RESPONSES.load(Ordering::Acquire), 5);

        __libertas_release_package();
    }
}
