//! Libertas SDK Rust Bindings
//!
//! This library provides Rust bindings for the Libertas IoT system, allowing applications
//! to interact with devices, manage timers, subscribe to events, and handle data exchange.
//!
//! # Core Features
//!
//! - **Device Communication**: Send commands and read data from devices via `libertas_device_invoke_req()`,
//!   `libertas_device_read_req()`, and `libertas_device_write_req()`
//! - **Virtual Devices**: Respond to requests for virtual devices with `libertas_virtual_device_invoke_rsp()`,
//!   `libertas_virtual_device_write_rsp()`, and `libertas_virtual_device_status_rsp()`
//! - **Timers**: Create and manage both interval and deadline timers for scheduling tasks
//! - **Subscriptions**: Subscribe to device attributes and events for reactive programming
//! - **Time Access**: Query system ticks and current time (UTC/local) via timing functions
//!
//! # Example
//!
//! ```ignore
//! // Create a subscription to device attributes
//! let mut dev_sub = LibertasDeviceSubscribeReq::new(device_id);
//! let mut cluster_sub = LibertasClusterSubscribeReq::new(0x0006, 0, 300);
//! cluster_sub.attributes.push(0x0000);
//! dev_sub.cluster_subs.push(cluster_sub);
//! libertas_app_subscribe_req(&[dev_sub]);
//!
//! // Create an interval timer
//! let timer_id = libertas_timer_new_interval(1000, |id, time, context| {
//!     println!("Timer {} fired at {}", id, time);
//! }, Box::new(()));
//! ```

#![no_std]
// Bring in the alloc crate
extern crate alloc;

mod system_message;

pub use system_message::{LibertasObjectType, LibertasObject, LibertasMessageLevel, LibertasMessageArgument, libertas_send_message, libertas_send_message_literal};

use alloc::{slice, boxed::Box, rc::Rc, vec::Vec};
use core::ffi::c_void;
use core::{any::Any, cell::RefCell};
use core::mem::{self, MaybeUninit};
use core::cmp::Reverse;

use hashbrown::HashMap;
use hashbrown::DefaultHashBuilder;
use priority_queue::PriorityQueue;

/// A LibertasDevice is key component of this world. It represents anything that has information, 
/// while Libertas is the platform for "Internet of Everything."
/// 
/// A LibertasDevice is a logical representation of "anything, " such as:
/// * A light bulb
/// * An email account
/// * A PDF file
pub type LibertasDevice = u32;

/// A dynamically created Libertas device. It's life time
/// is bound to the task that creates it.
/// It will be automatically deleted when the corresponding task is deleted.
pub type LibertasVirtualDevice = LibertasDevice;

/// A special LibertasVirtualDevice for exchanging structural data between tasks.
/// The data schema is defined by the original server developer and must be published.
pub type LibertasDataExchange = LibertasVirtualDevice;

/// Timestamp representing date and time
pub type LibertasDateTime = u64;

/// Timestamp representing time only (without date)
pub type LibertasTimeOnly = u32;

/// Identifier for a LAN device
pub type LibertasLanDevice = u32;

/// Identifier for a user
pub type LibertasUser = u32;

/// Identifier for an action
pub type LibertasAction = u32;

/// Transaction identifier
pub type LibertasTransId = u32;

pub const STACK_BUF_SIZE: usize = 1000;

pub const LIBERTAS_BROADCAST_DEST: u32 = 0xffffffff;

const CURRENT_VERSION: u32 = 0x000204;     // Version 0.2.4, 1.0 shall be 0x10000, each sub version must be within [0,255]
const PROTOCOL_LIBERTAS: u16 = 0;
const DEVICE_SYSTEM: u32 = 0;
const OP_SYSTEM_MESSAGE: u8 = 0xfe;

pub const OP_DATA_EXCHANGE_SUB_REQ:u8 = 3;
pub const OP_DATA_EXCHANGE_DATA: u8 = 5;
pub const OP_DATA_EXCHANGE_REQ: u8 = 8;
pub const OP_DATA_EXCHANGE_RSP: u8 = 9;
pub const OP_DATA_EXCHANGE_PEER_DOWN: u8 = 20;
const OP_DATA_EXCHANGE_REMOVE_PEER: u8 = 21;    // device_send

type LibertasTimerCallback = dyn FnMut(u32, u64, &mut Box<dyn Any>);
type LibertasDeviceCallback = dyn FnMut(LibertasDevice, u8, &[u8], &mut Box<dyn Any>, Option<LibertasTransId>, u32);

pub trait LibertasExport {
}

/// Currently a fixed size (1000 bytes) uninitialized stack buffer for message construction.
/// Will switch to [MaybeUninit::array_assume_init](https://doc.rust-lang.org/beta/std/mem/union.MaybeUninit.html#method.array_assume_init) in the future.
/// 
pub struct LibertasUninitStackbuf {
    data: [u8; STACK_BUF_SIZE],
}

impl LibertasUninitStackbuf {
    /// Creates a new uninitialized stack buffer
    pub fn new() -> Self {
        Self {
            data: {
                let data: [MaybeUninit<u8>; STACK_BUF_SIZE] = [const { MaybeUninit::uninit() }; STACK_BUF_SIZE];
                unsafe { mem::transmute::<_, [u8; STACK_BUF_SIZE]>(data) }
            },
        }
    }

    /// Returns a slice to the underlying byte data
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

/// Represents the Daylight Saving Time (DST) offset information for a specific time zone
#[repr(C)]
pub struct LibertasDstOffset {
    /// DST offset in seconds
    pub offset: i32,
    /// UTC timestamp when current DST period begins
    pub begin: u64,
    /// UTC timestamp when current DST period ends
    pub end: u64,
}

/// Represents the time zone information for the current local time, including current DST offset and next DST change information
#[repr(C)]
pub struct LibertasTimeZoneInfo {
    /// Current time zone offset from UTC in seconds
    pub current_offset: LibertasDstOffset,
    /// Next DST offset change information
    pub next_offset: LibertasDstOffset,
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

#[repr(C)]
struct TimerDriverResult {
    next_monotonic: u64,
    next_utc: u64,
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
    get_time_zone_info: extern "C" fn(*mut LibertasTimeZoneInfo) -> bool,
    get_utc_time: extern "C" fn() -> u64,
    device_send: extern "C" fn(u16, u32, u32, u8, *const u8, usize, u32),               // Protocol, device (virtual device src), trans_id, op_code, data, data_len, ack_dest (virtual device & data exchange)
}

// Use &pt as *const LibertasPackageCallback to pass back to C
#[repr(C)]
pub struct LibertasPackageCallback {
    version: u32,
    init_task: extern "C" fn(u32),
    remove_task: extern "C" fn(u32),
    timer_driver: extern "C" fn(u64, u64)->TimerDriverResult,
    device_callback: extern "C" fn(u32, u32, u8, *const u32, *const c_void, usize, u32),       // task, device (virtual device dst), op_code, trans_id, data, data_len, source (virtual device & data exchange)
}

struct Context {
    device_callbacks: HashMap<u32, DeviceCallback>,
}

impl Context {
    fn new() -> Self {
        Self {device_callbacks: HashMap::new()}
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

extern "C" fn libertas_impl_device_callback(task: u32, device: u32, op_code: u8, trans_id: *const u32, data: *const c_void, data_len: usize, peer: u32) {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                if let Some(context) = env.contexts.get_mut(&task) {
                    if let Some(dcb) = context.device_callbacks.get_mut(&device) {
                        let d = slice::from_raw_parts(data as *const u8, data_len);
                        (dcb.cb.borrow_mut())(device, op_code, d, &mut *dcb.context.borrow_mut(), if trans_id.is_null() { None } else { Some(*trans_id) }, peer);
                    }
                }
            }
            _ => { unreachable!(); }
        }
    }   
}

extern "C" fn libertas_impl_timer_driver(monotonic: u64, utc: u64) -> TimerDriverResult {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            match ENV {
                Some(ref mut env) => {
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
    trans_id: u32,
    timers: HashMap<u32, Timer>,
    timers_active_interval: PriorityQueue<u32, Reverse<u64>, DefaultHashBuilder>,
    timers_active_deadline: PriorityQueue<u32, Reverse<u64>, DefaultHashBuilder>,
    contexts: HashMap<u32, Context>,    // Key is task ID
    callback: LibertasPackageCallback,
    time_zone_info: Option<LibertasTimeZoneInfo>,
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
            callback: LibertasPackageCallback {
                version: CURRENT_VERSION,
                init_task: libertas_impl_init_task,
                remove_task: libertas_impl_remove_task,
                timer_driver: libertas_impl_timer_driver,
                device_callback: libertas_impl_device_callback,
            },
            time_zone_info: None,
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

    /**
     * Must be non-zero. Zero value i None.
     */
    #[inline(always)]
    fn new_trans_id(&mut self) -> u32 {
        self.trans_id += 1;
        if self.trans_id == 0 {
            self.trans_id = 1;
        }
        self.trans_id
    }
}

static mut RUNTIME_API: *mut LibertasRuntimeApi = core::ptr::null_mut();
static mut ENV: Option<LibertasPackageEnv> = None;

/**
 * Internal use only by Libertas SDK. The Libertas Apps are driven by a single thread. No locking is required.
 * This API is called only once as soon as the dynamic library is loaded by Libertas OS App engine. 
 * Attempting to call it again is considered a security attack and the caller App will be banned.
 * This function will be re-exported with a new unique name assigned by Libertas builder for each App package.
 */
pub fn __libertas_init_package(runtime_api:*mut c_void) -> *const LibertasPackageCallback {
    unsafe {
        RUNTIME_API = runtime_api as *mut LibertasRuntimeApi;
        ENV = Some(LibertasPackageEnv::new());
        let env_ptr = &raw mut ENV;
        // We use .as_ref() on the *pointer* dereference, or better yet:
        if let Some(ref env) = *env_ptr {
            let callback_ptr = &raw const (env.callback);
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
pub fn __libertas_release_package() {
    unsafe {
        ENV = None;     // Drop all memory
    }
}

/// Gets random bytes from the system's random number generator
/// 
/// # Arguments
/// * `bytes` - The number of random bytes to generate (up to 8)
/// # Returns
/// A 64-bit unsigned integer containing the random bytes in the least significant bits.
///
/// # Notes 
/// * Only the least significant `bytes` bytes of the returned value are valid random data. For example, if `bytes` is 4, only the least significant 4 bytes (32 bits) of the returned value contain random data, and the upper 4 bytes will be zero.
/// * The `bytes` argument must be between 1 and 8. If `bytes` is greater than 8 will be treated as if it were 8. If `bytes` is less than one, it will be treated as if it were 1.
/// * On many embedded MCUs, the random number generator may only provide 1 byte (8 bits) of randomness at a time. That's the reason why the API accepts a `bytes` argument to specify how many random bytes are needed, and the implementation may call the underlying RNG multiple times if more than 1 byte is requested.
/// 
pub fn libertas_get_random(bytes: u8) -> u64 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            return (runtime_api.get_random)(bytes);
        } else {
            unreachable!();
        }
    }
}

/// Gets the current system tick count
/// 
/// Returns the monotonic system clock in ticks. This can be used to measure
/// elapsed time between events.
/// 
/// # Returns
/// The current system tick count in milliseconds as a 64-bit unsigned integer
pub fn libertas_get_sys_ticks() -> u64 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            return (runtime_api.get_sys_ticks)();
        } else {
            unreachable!();
        }
    }
}

/// Gets the current UTC time in microseconds since Unix epoch
/// 
/// # Returns
/// The current UTC time in microseconds since Unix epoch as a 64-bit unsigned integer (typically Unix timestamp)
/// 
/// The function may return `None` if the system doesn't have the capability to provide UTC time, or the UTC time is not set. For example, MCUs may not have 
/// a real-time clock and the UTC from network may not be ready when the API is called.
/// 
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

fn libertas_update_time_zone_info(utc: u64, env: & mut LibertasPackageEnv, runtime_api: & LibertasRuntimeApi) {
    unsafe {
        let update_tz = match env.time_zone_info {
            Some(ref tz) => {
                utc >= tz.current_offset.end
            }
            None => true,
        };
        if update_tz {
            let mut info = MaybeUninit::<LibertasTimeZoneInfo>::uninit();
            if (runtime_api.get_time_zone_info)(info.as_mut_ptr()) {
                env.time_zone_info = Some(info.assume_init());
            }
        }
    }
}

/// Gets the current local time in microseconds since Unix epoch by applying the local time zone offset to the current UTC time.
/// 
/// # Returns
/// The current local time in microseconds since Unix epoch as a 64-bit unsigned integer
/// 
/// The function may return `None` if the system doesn't have the capability to provide UTC time, or the UTC time is not set. For example, MCUs may not have 
/// a real-time clock and the UTC from network may not be ready when the API is called.
/// 
/// # Remarks
/// We always assume the `LibertasTimeZoneInfo` is retrieved within a year of current time.
/// 
pub fn libertas_get_local_time() -> Option<u64> {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            let utc = (runtime_api.get_utc_time)();
            if utc == 0 {
                return None;
            }
            match ENV {
                Some(ref mut env) => {
                    libertas_update_time_zone_info(utc, env, runtime_api);
                    if let Some(ref tz) = env.time_zone_info {
                        let offset = if utc < tz.current_offset.end {
                            tz.current_offset.offset
                        } else {
                            tz.next_offset.offset
                        };
                        return Some((utc as i64 + (offset as i64) * 1000 * 1000) as u64);
                    } else {
                        return None;
                    }
                }
                _ => { unreachable!(); }
            }
        } else {
            unreachable!();
        }
    }
}

/// Converts a local time in microseconds since Unix epoch to UTC time by applying the local time zone offset.
/// 
/// # Remarks
/// The result is only accurate within the end of next Daylight Saving Time cycle (over a year). It is intended to be used on low-power MCUs.
/// 
pub fn libertas_local_time_to_utc(local: u64) -> Option<u64> {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            let now = (runtime_api.get_utc_time)();
            if now == 0 {
                return None;
            }            
            match ENV {
                Some(ref mut env) => {
                    libertas_update_time_zone_info(now, env, runtime_api);
                    if let Some(ref tz) = env.time_zone_info {
                        let mut try_utc = (local as i64 - (tz.current_offset.offset as i64) * 1000 * 1000) as u64;
                        if try_utc > tz.current_offset.end {
                            try_utc = (local as i64 - (tz.next_offset.offset as i64) * 1000 * 1000) as u64;
                        }
                        return Some(try_utc);
                    } else {
                        return None;
                    }
                }
                _ => { unreachable!(); }
            }
        } else {
            unreachable!();
        }
    }
}

/// Converts a UTC time in microseconds since Unix epoch to local time by applying the local time zone offset.
/// 
/// # Remarks
/// The result is only accurate within the end of next Daylight Saving Time cycle (over a year). It is intended to be used on low-power MCUs.
/// 
pub fn libertas_utc_time_to_local(utc: u64) -> Option<u64> {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            let now = (runtime_api.get_utc_time)();
            if now == 0 {
                return None;
            }            
            match ENV {
                Some(ref mut env) => {
                    libertas_update_time_zone_info(now, env, runtime_api);
                    if let Some(ref tz) = env.time_zone_info {
                        return if utc >= tz.current_offset.end {
                            Some((utc as i64 + (tz.next_offset.offset as i64) * 1000 * 1000) as u64)
                        } else {
                            Some((utc as i64 + (tz.current_offset.offset as i64) * 1000 * 1000) as u64)
                        }
                    } else {
                        return None;
                    }
                }
                _ => { unreachable!(); }
            }
        } else {
            unreachable!();
        }
    }
}

fn libertas_get_task_id() -> u32 {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            return (runtime_api.get_task_id)();
        } else {
            unreachable!();
        }
    }
}

fn libertas_timer_new(exipration: u64, interval: bool, callback: Box<LibertasTimerCallback>, context: Box<dyn Any>) -> u32 {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let task_id = libertas_get_task_id();
                let id = env.new_timer_id();
                let wrapped_cb = Rc::new(RefCell::new(callback));
                let wrapped_tag = Rc::new(RefCell::new(context));
                if exipration > 0 {
                    if interval {
                        env.timers.insert(id, Timer {
                            task: task_id,
                            state: TimerState::ArmedInterval(exipration),
                            cb: wrapped_cb,
                            context: wrapped_tag,
                        });
                        env.timers_active_interval.push(id, Reverse(exipration));
                    } else {
                        env.timers.insert(id, Timer {
                            task: task_id,
                            state: TimerState::ArmedDeadline(exipration),
                            cb: wrapped_cb,
                            context: wrapped_tag,
                        });
                        env.timers_active_deadline.push(id, Reverse(exipration));
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

/// Creates a new interval timer based on system ticks, which is a monotonic clock that is not affected by changes in the system time.
/// 
/// Please note that the system tick is in microseconds.
/// 
/// # Arguments
/// * `exipration` - The expiration time in system ticks in microseconds.
/// * `callback` - Closure called when the timer fires. Receives timer ID, current time, and context.
/// * `context` - User-defined data associated with the timer
/// 
/// # Returns
/// Timer identifier for use with other timer functions
/// 
#[inline(always)]
pub fn libertas_timer_new_interval<F>(exipration: u64, callback: F, context: Box<dyn Any>) -> u32 where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    return libertas_timer_new(exipration, true, Box::new(callback), context);
}

/// Creates a new deadline timer. Unlike interval timers, deadline timers waits until the specified UTC time.
/// 
/// Creates a one-shot timer that fires at a specific deadline. The callback
/// is invoked once when the deadline is reached.
/// 
/// # Arguments
/// * `exipration` - The deadline in UTC time in milliseconds
/// * `callback` - Closure called when the deadline is reached. Receives timer ID, deadline time, and context.
/// * `new_context` - User-defined data associated with the timer
/// 
/// # Returns
/// Timer identifier for use with other timer functions
/// 
#[inline(always)]
pub fn libertas_timer_new_deadline<F>(exipration: u64, callback: F, context: Box<dyn Any>) -> u32 where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    return libertas_timer_new(exipration, false, Box::new(callback), context);
}

fn libertas_timer_update(timer: u32, exipration: u64, interval: bool, callback: Option<Box<LibertasTimerCallback>>, new_context: Option<Box<dyn Any>>) {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                if let Some(t) = env.timers.get_mut(&timer) {
                    if t.task != libertas_get_task_id() {
                        panic!("libertas_timer_update task_id");
                    }
                    if interval {
                        if let TimerState::ArmedInterval(old_exp) = t.state {
                            if old_exp != exipration {
                                t.state = TimerState::ArmedInterval(exipration);
                                env.timers_active_interval.change_priority(&timer, Reverse(exipration));
                            }
                        } else {
                            if let TimerState::ArmedDeadline(_) = t.state {
                                env.timers_active_deadline.remove(&timer);
                            }
                            t.state = TimerState::ArmedInterval(exipration);
                            env.timers_active_interval.push(timer, Reverse(exipration));
                        }
                    } else {
                        if let TimerState::ArmedDeadline(old_exp) = t.state {
                            if old_exp != exipration {
                                t.state = TimerState::ArmedDeadline(exipration);
                                env.timers_active_deadline.change_priority(&timer, Reverse(exipration));
                            }
                        } else {
                            if let TimerState::ArmedInterval(_) = t.state {
                                env.timers_active_interval.remove(&timer);
                            }
                            t.state = TimerState::ArmedDeadline(exipration);
                            env.timers_active_deadline.push(timer, Reverse(exipration));
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

/// Updates the expiration time of an existing interval timer
/// 
/// Changes the expiration time for a running interval timer. The callback and context
/// remain unchanged.
/// 
/// # Arguments
/// * `timer` - The timer ID to update
/// * `exipration` - The new expiration time in system ticks in milliseconds
///
/// # Panics
/// Panics if the timer ID is invalid or belongs to a different task
#[inline(always)]
pub fn libertas_timer_update_interval(timer: u32, exipration: u64) {
    libertas_timer_update(timer, exipration, true, None, None);
}

/// Updates the deadline of an existing deadline timer
/// 
/// Changes the deadline for a running deadline timer. The callback and context
/// remain unchanged.
/// 
/// # Arguments
/// * `timer` - The timer ID to update
/// * `exipration` - The new deadline in UTC milliseconds ticks
///
/// # Panics
/// Panics if the timer ID is invalid or belongs to a different task
/// 
#[inline(always)]
pub fn libertas_timer_update_deadline(timer: u32, exipration: u64) {
    libertas_timer_update(timer, exipration, false, None, None);
}

/// Updates an interval timer with new callback and/or expiration time
/// 
/// Changes the expiration time and/or callback for a running interval timer.
/// 
/// # Arguments
/// * `timer` - The timer ID to update
/// * `exipration` - The new expiration time in system ticks in milliseconds
/// * `callback` - New closure to invoke on each interval expiration
/// * `new_context` - New user-defined data to associate with the timer
///
/// # Panics
/// Panics if the timer ID is invalid or belongs to a different task
/// 
#[inline(always)]
pub fn libertas_timer_update_interval_callback<F>(timer: u32, exipration: u64, callback: F, new_context: Box<dyn Any>) where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    libertas_timer_update(timer, exipration, true, Some(Box::new(callback)), Some(new_context));
}

/// Updates a deadline timer with new callback and/or deadline
/// 
/// Changes the deadline and/or callback for a running deadline timer.
/// 
/// # Arguments
/// * `timer` - The timer ID to update
/// * `exipration` - The new deadline in system ticks
/// * `callback` - New closure to invoke when the deadline is reached
/// * `new_context` - New user-defined data to associate with the timer
///
/// # Panics
/// Panics if the timer ID is invalid or belongs to a different task
/// 
#[inline(always)]
pub fn libertas_timer_update_deadline_callback<F>(timer: u32, exipration: u64, callback: F, new_context: Box<dyn Any>) where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    libertas_timer_update(timer, exipration, false, Some(Box::new(callback)), Some(new_context));
}

/// Cancels a running timer
/// 
/// Stops an active timer without deleting it. The timer can be restarted
/// by updating it again.
/// 
/// # Arguments
/// * `timer` - The timer ID to cancel
///
/// # Panics
/// Panics if the timer ID is invalid or belongs to a different task
/// 
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

/// Deletes a timer and frees its resources
/// 
/// Removes a timer completely from the system. The timer cannot be used again
/// without creating a new one.
/// 
/// # Arguments
/// * `timer` - The timer ID to delete
///
/// # Panics
/// Panics if the timer ID is invalid or belongs to a different task
/// 
pub fn libertas_timer_delete(timer: u32) {
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
                    panic!("timer_delete invalid timer");
                }
            }
            _ => { unreachable!(); }
        }
    }    
}

fn libertas_register_device_callback_impl(device: LibertasDevice, callback: Box<LibertasDeviceCallback>, tag: Box<dyn Any>) {
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
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasDataExchange`.
/// * `callback` - Closure called when device events occur. Receives device ID, operation code, event data, mutable context, and optional transaction ID.
/// * `context` - Developer-defined data passed to the callback function
/// 
/// # Remarks
/// Only one callback can be registered per device. Registering a new callback for the same device will cause panic. For devices that the task doesn't
/// have "read access", only responses to its own requests will trigger the callback. App can not subscribe from "write-only" devices.
/// 
/// A task is advised to collect all devices from function arguments before registering any callback to eliminate potential duplicates.
/// 
#[inline(always)]
pub fn libertas_register_device_callback<F>(device: LibertasDevice, callback: F, context: Box<dyn Any>) where F: FnMut(LibertasDevice, u8, &[u8], &mut Box<dyn Any>, Option<LibertasTransId>, u32) + Sized + 'static {
    libertas_register_device_callback_impl(device, Box::new(callback), context);
}

/// Sends a request to a LibertasDevice. A response is always expected as a transaction. Thus, a unique transaction ID is generated and returned.
/// 
/// # Remarks
/// Developers shall not call this API directly. Instead, they should use the protocol-specific APIs built on top of this API. For example, for Matter protocol, developers should use `matter_send_invoke_request` instead of this API. The protocol-specific APIs will handle the protocol details and provide a more user-friendly interface.
/// 
/// # Arguments
/// * `protocol` - The protocol ID for the request.
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasDataExchange`.
/// * `op` - Operation code for the request. This is an application defined.
/// * `data` - Binary blob containing the request data. The content is application defined.
/// 
/// # Returns
/// Unique transaction ID for tracking the response
/// 
pub fn libertas_device_send_request(protocol: u16, device: LibertasDevice, op: u8, data: &[u8]) -> u32 {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let ret = env.new_trans_id();
                if let Some(runtime_api) = RUNTIME_API.as_ref() {
                    (runtime_api.device_send)(protocol, device, ret, op, data.as_ptr(), data.len(), 0);
                } else {
                    unreachable!();
                }
                return ret;
            }
            _ => { unreachable!(); }
        }
    }    
}

/// Sends a response to a device request. This API is used to respond to a request received from a device, using the transaction ID provided in the request callback.
/// 
/// # Remarks
/// Developers shall not call this API directly. Instead, they should use the protocol-specific APIs built on top of this API. For example, for Matter protocol, developers should use `matter_send_invoke_response` instead of this API. The protocol-specific APIs will handle the protocol details and provide a more user-friendly interface.
/// 
/// # Arguments
/// * `protocol` - The protocol ID for the request.
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasDataExchange`.
/// * `op` - Operation code for the response. This is an application defined and typically defined in relation to the request op code.
/// * `data` - Binary blob containing the response data
/// * `trans_id` - Transaction ID from the original request to correlate the response.
/// * `peer` - The peer that sent the original request.
/// 
pub fn libertas_device_send_response(protocol: u16, device: LibertasDevice, op: u8, data: &[u8], trans_id: u32, peer: u32) {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.device_send)(protocol, device, trans_id, op, data.as_ptr(), data.len(), peer);
        } else {
            unreachable!();
        }
    }
}

/// Sends a report to a device without expecting a response. This API can be used for unsolicited updates to the device.
/// 
/// # Remarks
/// Developers shall not call this API directly. Instead, they should use the protocol-specific APIs built on top of this API. For example, for Matter protocol, developers should use `matter_send_report` instead of this API. The protocol-specific APIs will handle the protocol details and provide a more user
/// 
/// # Arguments
/// * `protocol` - The protocol ID for the request.
/// * `device` - A `LibertasDevice` or `LibertasVirtualDevice` or `LibertasDataExchange`.
/// * `op` - Operation code for the report. This is an application defined.
/// * `data` - Binary blob containing the report data.
/// * `peer` - The peer that sent the original request.
/// 
pub fn libertas_device_send_report(protocol: u16, device: LibertasDevice, op: u8, data: &[u8], peer: u32) {
    unsafe {
        if let Some(runtime_api) = RUNTIME_API.as_ref() {
            (runtime_api.device_send)(protocol, device, 0, op, data.as_ptr(), data.len(), peer);
        } else {
            unreachable!();
        }
    }
}

/// Sends a data exchange request to a server
///
/// Sends a data exchange request to the specified server with the given operation code
/// and data payload. Returns a transaction ID for tracking the response.
///
/// # Arguments
/// * `server` - A data exchange server. The link created when a Libertas machine is created.
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
pub fn libertas_data_exchange_request(server: u32, data: &[u8]) -> u32 {
    libertas_device_send_request(PROTOCOL_LIBERTAS, server, OP_DATA_EXCHANGE_REQ, data)
}

/// Sends a data exchange request to a server
///
/// Sends a data exchange request to the specified server with the given operation code
/// and data payload. Returns a transaction ID for tracking the response.
///
/// # Arguments
/// * `server` - A data exchange server. The link created when a Libertas machine is created.
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
pub fn libertas_data_exchange_subscribe_request(server: u32, data: &[u8]) -> u32 {
    libertas_device_send_request(PROTOCOL_LIBERTAS, server, OP_DATA_EXCHANGE_SUB_REQ, data)
}

/// Sends a data exchange response to a device
///
/// Sends a response to a data exchange request with the specified operation code
/// and data payload using the provided transaction ID.
///
/// # Arguments
/// * `server` - A data exchange server id. The link created when a Libertas machine is created.
/// * `data` - The data payload to send with the response
/// * `trans_id` - The transaction ID from the original request
/// * `dest` - The source of the original request as the destination of the request.
///
/// # Note
/// Unlike Matter protocol, the data can be any data structure defined and published by the server developer, encoded with Apache Avro format.
#[inline(always)]
pub fn libertas_data_exchange_rsp(server: u32, data: &[u8], trans_id: u32, dest: u32) {
    libertas_device_send_response(PROTOCOL_LIBERTAS, server, OP_DATA_EXCHANGE_RSP, data, trans_id, dest);
}

/// Sends a data exchange report to subscribers
///
/// Sends report data to all subscribers of a data exchange point or to a specific device.
/// This is typically used to push data updates to clients that have subscribed to data changes.
///
/// # Arguments
/// * `server` - A data exchange server id. The link created when a Libertas machine is created.
/// * `data` - The data payload to send in the report
/// * `peer` - Optional specific device ID to send the report to. If None, broadcasts to all subscribers.
///
/// # Note
/// Unlike Matter protocol, the data can be any data structure defined and published by the server developer, encoded with Apache Avro format.
/// This function is used to fulfill subscription requests that expect `OpCode::ReportData` messages.
#[inline(always)]
pub fn libertas_data_exchange_report(server: u32, data: &[u8], peer: Option<LibertasDataExchange>) {
    let peer_id = peer.unwrap_or(u32::MAX);
    libertas_device_send_report(PROTOCOL_LIBERTAS, server, OP_DATA_EXCHANGE_DATA, data, peer_id);
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
/// #Arguments
/// * `server` - A data exchange server id. The link created when a Libertas machine is created.
/// * `peer` - The subscriber peer.
/// 
#[inline(always)]
pub fn libertas_data_exchange_remove_subscriber(server: u32, peer: u32) {
    let empty_slice: &[u8] = &[];
    libertas_device_send_response(PROTOCOL_LIBERTAS, server, OP_DATA_EXCHANGE_REMOVE_PEER, empty_slice, 0, peer);
}
