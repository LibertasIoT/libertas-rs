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
use core::{any::Any, cell::RefCell};
use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use core::ffi::{c_void, c_char};
use core::mem::{self, MaybeUninit};
use alloc::{slice, boxed::Box, rc::Rc, vec::Vec};
use hashbrown::HashMap;
use hashbrown::DefaultHashBuilder;
use num_derive::{FromPrimitive, ToPrimitive};
use num::{FromPrimitive};
use priority_queue::PriorityQueue;
use core::cmp::Reverse;
use libertas_matter::im::{OpCode};

/// Identifier for a device in the Libertas system
pub type LibertasDevice = u32;

/// Timestamp representing date and time
pub type LibertasDateTime = u64;

/// Timestamp representing time only (without date)
pub type LibertasTimeOnly = u32;

/// Identifier for a data exchange transaction
pub type LibertasDataExchange = u32;

/// Identifier for a LAN device
pub type LibertasLanDevice = u32;

/// Identifier for a virtual device
pub type LibertasVirtualDevice = u32;

/// Identifier for a user
pub type LibertasUser = u32;

/// Identifier for an action
pub type LibertasAction = u32;

/// Transaction identifier
pub type LibertasTransId = u32;

pub const STACK_BUF_SIZE: usize = 1000;

type LibertasTimerCallback = dyn FnMut(u32, u64, &mut Box<dyn Any>);
type LibertasDeviceCallback = dyn FnMut(LibertasDevice, libertas_matter::im::OpCode, &[u8], &mut Box<dyn Any>, Option<LibertasTransId>);

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

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, PartialEq, Eq, Debug, Clone, Copy)]
pub enum LibertasObjectType {
    PhysicalDevice,
    LogicalDevice,
    App,
    AppTask,
    User,
    Client,
}

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, PartialEq, Eq, Debug, Clone, Copy)]
pub enum LibertasMessageLevel {
    Debug,
    Info,
    AlertLow,
    AlertGuarded,
    AlertElevated,
    AlertHigh,
    AlertSevere,
}

#[repr(C)]
pub struct LibertasObject {
    pub obj_type: LibertasObjectType,
    pub obj_id: u32,
}

#[repr(C)]
struct LibertasMessageArgumentInternal {
    arg_type: u8,
    text: *const u8,
    value: u64,
}

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

impl LibertasMessageArgumentInternal {
    fn from(arg: &LibertasMessageArgument) -> Self {
        match arg {
            LibertasMessageArgument::Boolean(b) => {
                Self {
                    arg_type: 0,
                    text: core::ptr::null(),
                    value: if *b { 1 } else { 0 },
                }
            }
            LibertasMessageArgument::Signed(s) => {
                Self {
                    arg_type: 1,
                    text: core::ptr::null(),
                    value: *s as u64,
                }
            }
            LibertasMessageArgument::Unsigned(u) => {
                Self {
                    arg_type: 2,
                    text: core::ptr::null(),
                    value: *u,
                }
            }
            LibertasMessageArgument::Float(f) => {
                Self {
                    arg_type: 3,
                    text: core::ptr::null(),
                    value: f.to_bits(),
                }
            }
            LibertasMessageArgument::Text(s) => {
                Self {
                    arg_type: 4,
                    text: s.as_ptr(),
                    value: s.len() as u64,
                }
            }
            LibertasMessageArgument::ResourceString(slice) => {
                Self {
                    arg_type: 5,
                    text: slice.as_ptr(),
                    value: slice.len() as u64,
                }
            }
            LibertasMessageArgument::UnitSigned {unit_type, value} => {
                Self {
                    arg_type: 6,
                    text: unit_type.as_ptr(),
                    value: *value as u64,
                }
            }
            LibertasMessageArgument::UnitUnsigned {unit_type, value} => {
                Self {
                    arg_type: 7,
                    text: unit_type.as_ptr(),
                    value: *value,
                }
            }
            LibertasMessageArgument::UnitFloat {unit_type, value} => {
                Self {
                    arg_type: 8,
                    text: unit_type.as_ptr(),
                    value: value.to_bits(),
                }
            }
            LibertasMessageArgument::Object {obj_type, obj_id} => {
                let v = *obj_type as u64;
                Self {
                    arg_type: 9,
                    text: core::ptr::null(),
                    value: (v << 32) | *obj_id as u64
                }
            }
        }
    }
}

/// Sends a message to a list of recipients. 
/// 
/// The message will be delivered to recipients' smartphones or other supported devices.
/// 
/// # Panics
/// App will panic if the resulting message size exceeds STACK_BUF_SIZE bytes.
/// App will panic on permission violation, including reecipients and LibertasObjects used in source and arguments.
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
        if let Some(c_api) = C_API.as_ref() {
            let src = match source {
                Some(s) => s,
                None => LibertasObject { obj_type: LibertasObjectType::App, obj_id: 0 },
            };
            (c_api.app_send_message)(recipients.as_ptr(), recipients.len(), level as u8, src, resource_name.as_ptr(), arg_internals.as_ptr(), arg_internals.len());
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

/// Represents a subscription to a cluster event
/// 
/// Used to subscribe to specific events within a cluster with urgency levels.
#[repr(C)]
pub struct LibertasClusterEventSubscription {
    /// The ID of the event to subscribe to
    pub event_id: u32,
    /// Whether this event is urgent
    pub urgent: bool,
}

#[repr(C)]
struct LibertasClusterSubscribeReqRaw {
    cluster: u32,
    min_interval: u16,
    max_interval: u16,
    attributes: *const u32,
    attributes_len: usize,
    events:  *const LibertasClusterEventSubscription,
    events_len: usize,
}

#[repr(C)]
struct LibertasDeviceSubscribeReqRaw {
    device: LibertasDevice,
    cluster_subs: *const LibertasClusterSubscribeReqRaw,
    cluster_subs_len: usize,
    event_min: u64,
}

#[repr(C)]
struct LibertasClusterReadReqRaw {
    cluster: u32,
    attributes: *const u32,
    attributes_len: usize,
    events: *const u32,
    events_len: usize,
}

#[repr(C)]
struct LibertasCApi {
    c_alloc: extern "C" fn (size: usize) -> *mut u8,
    c_free: extern "C" fn (ptr: *mut u8),
    c_panic: extern "C" fn (message: *const c_char, file: *const c_char, line: u32),
    get_task_id: extern "C" fn() -> u32,
    set_task_id: extern "C" fn(u32),    // Used in callback driver
    get_sys_ticks: extern "C" fn() -> u64,
    get_utc_time: extern "C" fn() -> u64,
    get_local_time: extern "C" fn() -> u64,
    app_send_message: extern "C" fn(*const u32, usize, u8, LibertasObject, *const u8, *const LibertasMessageArgumentInternal, usize),
    app_subscribe_req: extern "C" fn(*const LibertasDeviceSubscribeReqRaw, usize),
    device_invoke_req: extern "C" fn(u32, u32, *const u8, usize),
    device_read_req: extern "C" fn(u32, u32, *const LibertasClusterReadReqRaw, usize),
    device_write_req: extern "C" fn(u32, u32, *const u8, usize),
    virtual_device_invoke_rsp: extern "C" fn(u32, u32, *const u8, usize),
    virtual_device_write_rsp: extern "C" fn(u32, u32, *const u8, usize),
    virtual_device_status_rsp: extern "C" fn(u32, u32, u32),
}

// Use &pt as *const LibertasPackageCallback to pass back to C
#[repr(C)]
pub struct LibertasPackageCallback {
    init_task: extern "C" fn(u32),
    remove_task: extern "C" fn(u32),
    timer_driver: extern "C" fn(u64, u64)->TimerDriverResult,
    device_callback: extern "C" fn(u32, u32, u8, u32, *const c_void, usize),     // task, device, trans_type, trans_id, data
}

impl LibertasDeviceSubscribeReqRaw {
    // holder has guaranteed capacity.
    fn new(req: &LibertasDeviceSubscribeReq, holder: &mut Vec<Vec<LibertasClusterSubscribeReqRaw>>) -> Self {
        let mut clusters: Vec<LibertasClusterSubscribeReqRaw> = Vec::with_capacity(req.cluster_subs.len());
        for cur in &req.cluster_subs {
            clusters.push(LibertasClusterSubscribeReqRaw::new(cur));
        }
        holder.push(clusters);      // clusters moved
        if let Some(clusters_ref) =  holder.last() {
            Self {
                device: req.device,
                cluster_subs: clusters_ref.as_ptr(),
                cluster_subs_len: clusters_ref.len(),
                event_min: req.event_min.unwrap_or(0),
            }
        } else {
            unreachable!();
        }
    }
}

impl LibertasClusterSubscribeReqRaw {
    fn new(req: &LibertasClusterSubscribeReq) -> Self {
        Self {
            cluster: req.cluster,
            min_interval: req.min_interval,
            max_interval: req.max_interval,
            attributes: req.attributes.as_ptr(),
            attributes_len: req.attributes.len(),
            events:  req.events.as_ptr(),
            events_len: req.events.len(),
        }
    }
}

impl LibertasClusterEventSubscription {
    /// Creates a new cluster event subscription
    /// 
    /// # Arguments
    /// * `event_id` - The ID of the event to subscribe to
    /// * `urgent` - Whether this event should be treated as urgent
    pub fn new(event_id: u32, urgent: bool) -> Self {
        Self {
            event_id: event_id,
            urgent: urgent,
        }
    }
}

/// Request to subscribe to cluster attributes and events
/// 
/// Defines subscription parameters for a specific cluster, including
/// attribute IDs, event subscriptions, and polling intervals.
pub struct LibertasClusterSubscribeReq {
    /// Cluster identifier
    pub cluster: u32,
    /// Minimum polling interval in seconds
    pub min_interval: u16,
    /// Maximum polling interval in seconds
    pub max_interval: u16,
    /// List of attribute IDs to subscribe to
    pub attributes: Vec<u32>,
    /// List of events to subscribe to
    pub events: Vec<LibertasClusterEventSubscription>,
}

impl LibertasClusterSubscribeReq {
    /// Creates a new cluster subscription request
    /// 
    /// # Arguments
    /// * `cluster` - The cluster ID to subscribe to
    /// * `min_interval` - Minimum polling interval in seconds
    /// * `max_interval` - Maximum polling interval in seconds
    pub fn new(cluster: u32, min_interval: u16, max_interval: u16) -> Self {
        Self {
            cluster: cluster,
            min_interval: min_interval,
            max_interval: max_interval,
            attributes: Vec::new(),
            events: Vec::new(),
        }
    }
}

/// Request to subscribe to device cluster data and events
/// 
/// Allows subscribing to multiple clusters within a device with
/// specific attributes and events of interest.
pub struct LibertasDeviceSubscribeReq {
    /// Target device ID
    pub device: LibertasDevice,
    /// List of cluster subscription requests for this device
    pub cluster_subs: Vec<LibertasClusterSubscribeReq>,
    /// Minimum event timestamp to retrieve (optional)
    pub event_min: Option<u64>,
}

impl LibertasDeviceSubscribeReq {
    /// Creates a new device subscription request
    /// 
    /// # Arguments
    /// * `device` - The device ID to subscribe to
    pub fn new(device: LibertasDevice) -> Self {
        Self {
            device: device,
            cluster_subs: Vec::new(),
            event_min: None,
        }
    }
}

struct Context {
    device_callbacks: HashMap<u32, DeviceCallback>,
}

impl Context {
    fn new() -> Self {
        Self {device_callbacks: HashMap::new()}
    }
}

/// Request to read cluster attributes and events
/// 
/// Specifies which attributes and events to read from a specific cluster.
pub struct LibertasClusterReadReq {
    /// Cluster identifier
    pub cluster: u32,
    /// List of attribute IDs to read
    pub attributes: Vec<u32>,
    /// List of event IDs to read
    pub events: Vec<u32>,
}

impl LibertasClusterReadReq {
    /// Creates a new cluster read request
    /// 
    /// # Arguments
    /// * `cluster` - The cluster ID to read from
    pub fn new(cluster: u32) -> Self {
        Self {
            cluster: cluster,
            attributes: Vec::new(),
            events: Vec::new(),
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

extern "C" fn libertas_impl_device_callback(task: u32, device: u32, trans_type: u8, trans_id: u32, data: *const c_void, data_len: usize) {
    let op = OpCode::from_u8(trans_type).unwrap();
    unsafe {
        match ENV {
            Some(ref mut env) => {
                if let Some(context) = env.contexts.get_mut(&task) {
                    if let Some(dcb) = context.device_callbacks.get_mut(&device) {
                        let d = slice::from_raw_parts(data as *const u8, data_len);
                        (dcb.cb.borrow_mut())(device, op, d, &mut *dcb.context.borrow_mut(), if trans_id == 0 { None } else { Some(trans_id) });
                    }
                }
            }
            _ => { unreachable!(); }
        }
    }   
}

extern "C" fn libertas_impl_timer_driver(monotonic: u64, utc: u64) -> TimerDriverResult {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
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
                                    (c_api.set_task_id)(t.task);
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
                                    (c_api.set_task_id)(t.task);
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
                init_task: libertas_impl_init_task,
                remove_task: libertas_impl_remove_task,
                timer_driver: libertas_impl_timer_driver,
                device_callback: libertas_impl_device_callback,
            },
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

static mut C_API: *mut LibertasCApi = core::ptr::null_mut();
static mut ENV: Option<LibertasPackageEnv> = None;

/**
 * Internal use only by Libertas SDK. The Libertas Apps are driven by a single thread. No locking is required.
 * This API is called only once as soon as the dynamic library is loaded by Libertas OS App engine. 
 * Attempting to call it again is considered a security attack and the caller App will be banned.
 * This function will be re-exported with a new unique name assigned by Libertas builder for each App package.
 */
pub fn __libertas_init_package(c_api:*mut c_void) -> *const LibertasPackageCallback {
    unsafe {
        C_API = c_api as *mut LibertasCApi;
        ENV = Some(LibertasPackageEnv::new());
        match ENV {
            Some(ref mut env) => {
                &env.callback
            },
            _ => { unreachable!(); }
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

/// Gets the current system tick count
/// 
/// Returns the monotonic system clock in ticks. This can be used to measure
/// elapsed time between events.
/// 
/// # Returns
/// The current system tick count in milliseconds as a 64-bit unsigned integer
pub fn libertas_get_sys_ticks() -> u64 {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            return (c_api.get_sys_ticks)();
        } else {
            unreachable!();
        }
    }
}

/// Gets the current UTC time
/// 
/// Returns the current time in UTC (Coordinated Universal Time).
/// 
/// # Returns
/// The current UTC time in milliseconds since Unix epoch as a 64-bit unsigned integer (typically Unix timestamp)
pub fn libertas_get_utc_time() -> u64 {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            return (c_api.get_utc_time)();
        } else {
            unreachable!();
        }
    }
}

/// Gets the current local time
/// 
/// Returns the current time adjusted for the local timezone.
/// 
/// # Returns
/// The current local time in milliseconds since Unix epoch as a 64-bit unsigned integer
pub fn libertas_get_local_time() -> u64 {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            return (c_api.get_local_time)();
        } else {
            unreachable!();
        }
    }
}

fn libertas_get_task_id() -> u32 {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            return (c_api.get_task_id)();
        } else {
            unreachable!();
        }
    }
}

fn libertas_timer_new(exipration: u64, interval: bool, callback: Box<LibertasTimerCallback>, context: Box<dyn Any>) -> u32 {
    unsafe {
        let task_id = libertas_get_task_id();
        match ENV {
            Some(ref mut env) => {
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

/// Creates a new interval timer
/// 
/// Creates a repeating timer that fires at regular intervals. The callback
/// is invoked each time the interval expires.
/// 
/// # Arguments
/// * `exipration` - The expiration time in system ticks in milliseconds
/// * `callback` - Closure called when the timer fires. Receives timer ID, current time, and context.
/// * `context` - User-defined data associated with the timer
/// 
/// # Returns
/// Timer identifier for use with other timer functions
pub fn libertas_timer_new_interval<F>(exipration: u64, callback: F, context: Box<dyn Any>) -> u32 where F: FnMut(u32, u64, &mut Box<dyn Any>) + Sized + 'static {
    return libertas_timer_new(exipration, true, Box::new(callback), context);
}

/// Creates a new deadline timer
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

/// Sends an invoke command to a device
/// 
/// Sends a command to a device and returns a transaction ID for tracking the response.
/// The device will invoke the specified command with the provided data.
/// 
/// # Arguments
/// * `device` - Target device ID
/// * `data` - Command data to send to the device
/// 
/// # Returns
/// Transaction ID that can be used to correlate responses from the device
pub fn libertas_device_invoke_req(device: LibertasDevice, data: &[u8]) -> u32 {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let ret = env.new_trans_id();
                if let Some(c_api) = C_API.as_ref() {
                    (c_api.device_invoke_req)(device, ret, data.as_ptr(), data.len());
                } else {
                    unreachable!();
                }
                return ret;
            }
            _ => { unreachable!(); }
        }
    }    
}

/// Sends a read request to a device
/// 
/// Requests the values of specific attributes and events from a device.
/// Returns a transaction ID for tracking the response.
/// 
/// # Arguments
/// * `device` - Target device ID
/// * `data` - List of cluster read requests specifying what to read
/// 
/// # Returns
/// Transaction ID that can be used to correlate responses from the device
pub fn libertas_device_read_req(device: LibertasDevice, data: &[LibertasClusterReadReq]) -> u32 {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let ret = env.new_trans_id();
                if let Some(c_api) = C_API.as_ref() {
                    let mut raw_list: Vec<LibertasClusterReadReqRaw> = Vec::with_capacity(data.len());
                    for cur in data {
                        raw_list.push(
                            LibertasClusterReadReqRaw {
                                cluster: cur.cluster,
                                attributes: cur.attributes.as_ptr(),
                                attributes_len: cur.attributes.len(),
                                events: cur.events.as_ptr(),
                                events_len: cur.events.len(),
                            });
                    }
                    (c_api.device_read_req)(device, ret, raw_list.as_ptr(), data.len());
                } else {
                    unreachable!();
                }
                return ret;
            }
            _ => { unreachable!(); }
        }
    }    
}

/// Sends a writerequest to a device
/// 
/// Sends write request to a device and returns a transaction ID for tracking the response.
/// The device will process the write request with the provided data.
/// 
/// # Arguments
/// * `device` - Target device ID
/// * `data` - Data to write to the device
/// 
/// # Returns
/// Transaction ID that can be used to correlate responses from the device
pub fn libertas_device_write_req(device: LibertasDevice, data: &[u8]) -> u32 {
    unsafe {
        match ENV {
            Some(ref mut env) => {
                let ret = env.new_trans_id();
                if let Some(c_api) = C_API.as_ref() {
                    (c_api.device_write_req)(device, ret, data.as_ptr(), data.len());
                } else {
                    unreachable!();
                }
                return ret;
            }
            _ => { unreachable!(); }
        }        
    }    
}

/// Subscribes to device attributes and events
/// 
/// Registers subscriptions to receive updates about specific device attributes
/// and events. The system will notify the application when subscribed data changes.
/// 
/// # Arguments
/// * `device_subscriptions` - List of device subscription requests
pub fn libertas_app_subscribe_req(device_subscriptions: &[LibertasDeviceSubscribeReq]) {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            let mut device_list: Vec<LibertasDeviceSubscribeReqRaw> = Vec::with_capacity(device_subscriptions.len());
            let mut device_clusters: Vec<Vec<LibertasClusterSubscribeReqRaw>> = Vec::with_capacity(device_subscriptions.len());
            for cur in device_subscriptions {
                device_list.push(LibertasDeviceSubscribeReqRaw::new(cur, &mut device_clusters));
            }
            (c_api.app_subscribe_req)(device_list.as_ptr(), device_list.len());
        } else {
            unreachable!();
        }
    }
}

/// Sends an invoke response from a virtual device
/// 
/// Responds to an invoke request directed at a virtual device implementation.
/// 
/// # Arguments
/// * `device` - Virtual device ID
/// * `seq` - Sequence/transaction ID from the request
/// * `data` - Response data
pub fn libertas_virtual_device_invoke_rsp(device: LibertasDevice, seq: u32, data: &[u8]) {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            (c_api.virtual_device_invoke_rsp)(device, seq, data.as_ptr(), data.len());
        } else {
            unreachable!();
        }
    }    
}

/// Sends a write response from a virtual device
/// 
/// Responds to a write request directed at a virtual device implementation.
/// 
/// # Arguments
/// * `device` - Virtual device ID
/// * `seq` - Sequence/transaction ID from the request
/// * `data` - Response data
pub fn libertas_virtual_device_write_rsp(device: LibertasDevice, seq: u32, data: &[u8]) {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            (c_api.virtual_device_write_rsp)(device, seq, data.as_ptr(), data.len());
        } else {
            unreachable!();
        }
    }    
}

/// Sends a status response from a virtual device
/// 
/// Responds with a status code to a request directed at a virtual device implementation.
/// 
/// # Arguments
/// * `device` - Virtual device ID
/// * `seq` - Sequence/transaction ID from the request
/// * `status` - Status code (0 = success, non-zero = error)
pub fn libertas_virtual_device_status_rsp(device: LibertasDevice, seq: u32, status: u32) {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            (c_api.virtual_device_status_rsp)(device, seq, status);
        } else {
            unreachable!();
        }
    }    
}

pub struct __LibertasAllocator;

unsafe impl GlobalAlloc for __LibertasAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            if let Some(c_api) = C_API.as_ref() {
                (c_api.c_alloc)(layout.size())
            } else {
                unreachable!();
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            if let Some(c_api) = C_API.as_ref() {
                (c_api.c_free)(ptr)
            } else {
                unreachable!();
            }
        }
    }
}

pub fn __libertas_panic(info: &PanicInfo) -> ! {
    unsafe {
        if let Some(c_api) = C_API.as_ref() {
            let message = if let Some(s) = info.message().as_str() {
                s
            } else {
                "Unknown"
            };
            let mut line = 0;
            let loc = if let Some(location) = info.location() {
                line = location.line();
                location.file()
            } else {
                "Unknown"
            };
            (c_api.c_panic)(message.as_ptr() as *const c_char, loc.as_ptr() as *const c_char, line);
        }
        loop {}
    }
}
