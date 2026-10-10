//! Optional coroutine API over the host's single-threaded SDK.
//!
//! Attach a [`Dispatcher`] during App startup, bind typed Avro commands, and
//! move its [`Service`] and [`Endpoint`] handles into your Tokio tasks. The
//! Enable it with `libertas = { version = "0.3", features = ["tokio"] }`.
//! Only Tokio's `sync` feature is required here; select executor, I/O, and time
//! features in your App's own Tokio dependency as needed. The
//! host retains the dispatcher after startup returns. Your App owns its Tokio
//! executor; this module starts no threads and does not block the host loop.
//!
//! ```no_run
//! use libertas::tokio::Dispatcher;
//! // On the host thread, during normal App startup:
//! let dispatcher = Dispatcher::attach(32)?;
//! let clock = dispatcher.bind(|_: bool| libertas::libertas_get_sys_ticks())?;
//! // Move `clock` into a task on the App's executor:
//! # async move {
//! let ticks = clock.call(true).await?;
//! # Ok::<(), libertas::tokio::Error>(())
//! # };
//! # Ok::<(), libertas::tokio::Error>(())
//! ```
//!
//! Handlers execute complete synchronous SDK operations on the host thread,
//! including decoding reads before the host's scratch buffer can be reused.
//! Handlers may retain `Rc<RefCell<_>>` state. Only Avro bytes cross threads.
//! A sequence of Data calls in a handler has exactly the core Data semantics;
//! this API does not turn it into a storage transaction.
//!
//! Queue capacity applies backpressure. Dropping a call before dispatch skips
//! it; dropping a transmitted request only abandons its reply, without undoing
//! effects. Use the App's timeout policy to bound requests to unavailable peers.
//!
//! On host shutdown, [`Handle::shutdown_requested`] resolves. Stop and join all
//! App workers before [`Handle::complete_shutdown`]. Existing custom shutdown
//! handlers must coordinate that same stop boundary. The dispatcher continues
//! to accept cleanup calls until completion is requested. Never block the host
//! thread waiting for a queued call or for a Tokio task that needs the host.

use crate::*;
use ::tokio::sync::{mpsc, oneshot, watch};
use alloc::{string::String, vec::Vec};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode};
use std::{
    sync::{Arc, Mutex},
    thread::ThreadId,
};

pub mod data;
mod endpoint;
pub use endpoint::{Endpoint, EndpointEvent, Events};

/// Local dispatch failures. Persistent-data failures retain the core SDK's
/// no-status / explicit `DataErrorStatus` behavior inside the bound handler.
#[derive(Clone, Debug, PartialEq, Eq, LibertasAvroEncode, LibertasAvroDecode)]
#[non_exhaustive]
pub enum Error {
    NotInitialized,
    WrongThread,
    WrongTask,
    AlreadyAttached,
    InvalidCapacity,
    Closed,
    QueueFull,
    EndpointInUse,
    NotARequest,
    TooManyRequests,
    IdExhausted,
    ShutdownNotRequested,
    /// Invalid or incompletely consumed Avro bytes.
    Codec(String),
    /// Unsolicited events exceeded the configured receiver capacity.
    EventsLost(u64),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Libertas coroutine dispatch: {self:?}")
    }
}
impl std::error::Error for Error {}

type Completion = oneshot::Sender<Vec<u8>>;
type Handler = Rc<RefCell<Box<dyn FnMut(Vec<u8>, Completion)>>>;

// The local completion channel is transport bookkeeping. The command itself
// is one Avro record; application command and reply datums are Avro bytes.
#[derive(LibertasAvroEncode, LibertasAvroDecode)]
struct CommandFrame {
    service: u64,
    payload: Vec<u8>,
}
#[derive(LibertasAvroEncode, LibertasAvroDecode)]
enum ReplyFrame {
    Value(Vec<u8>),
    Failure(Error),
}
fn finish(completion: Completion, result: Result<Vec<u8>, Error>) {
    let frame = match result {
        Ok(value) => ReplyFrame::Value(value),
        Err(error) => ReplyFrame::Failure(error),
    };
    let _ = completion.send(encode(&frame));
}
fn decode_reply<R: AvroDecode>(bytes: &[u8]) -> Result<R, Error> {
    match decode::<ReplyFrame>(bytes)? {
        ReplyFrame::Value(value) => decode(&value),
        ReplyFrame::Failure(error) => Err(error),
    }
}
struct Submission {
    frame: Vec<u8>,
    completion: Completion,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Open,
    Finishing,
    Closed,
}
struct Shared {
    // Serializes enqueue+wakeup against closure. Never held across an await,
    // handler invocation, or response completion.
    phase: Mutex<Phase>,
    shutdown: watch::Sender<bool>,
}
impl Shared {
    fn wake(&self) {
        let phase = self.phase.lock().unwrap();
        if *phase == Phase::Open {
            libertas_wake_up();
        }
    }
    fn is_open(&self) -> bool {
        *self.phase.lock().unwrap() == Phase::Open
    }
}

/// Shareable task-bound lifecycle handle. It never reads main-thread SDK state.
#[derive(Clone)]
pub struct Handle {
    sender: mpsc::Sender<Submission>,
    shared: Arc<Shared>,
    task: LibertasTaskId,
}
impl Handle {
    pub fn task_id(&self) -> LibertasTaskId {
        self.task
    }
    pub fn is_closed(&self) -> bool {
        !self.shared.is_open()
    }

    /// Wait for the host's shutdown signal, even if the command queue is full.
    /// Also resolves when this dispatcher is closed or its task is removed.
    pub async fn shutdown_requested(&self) {
        let mut receiver = self.shared.shutdown.subscribe();
        loop {
            if *receiver.borrow_and_update() {
                return;
            }
            if receiver.changed().await.is_err() {
                return;
            }
        }
    }

    /// Stop submissions and ask the host thread to close outstanding calls and
    /// complete shutdown. Call only after all other App work has stopped. Make
    /// no further Libertas API calls after this succeeds and return promptly.
    pub fn complete_shutdown(&self) -> Result<(), Error> {
        let mut phase = self.shared.phase.lock().unwrap();
        if *phase != Phase::Open {
            return Err(Error::Closed);
        }
        if SHUTDOWN_STATE.load(Ordering::Acquire) != SHUTDOWN_REQUESTED {
            return Err(Error::ShutdownNotRequested);
        }
        *phase = Phase::Finishing;
        libertas_wake_up();
        Ok(())
    }

    fn frame(service: u64, payload: &[u8]) -> Vec<u8> {
        // This frame is retained in the queue. Reserve once and append the
        // borrowed payload without allocating another owned payload object.
        let mut bytes = Vec::with_capacity(payload.len() + 20);
        service.avro_encode(&mut bytes);
        (payload.len() as i64).avro_encode(&mut bytes);
        bytes.extend_from_slice(payload);
        bytes
    }
    fn try_submit<R: AvroDecode>(&self, service: u64, payload: &[u8]) -> Result<Reply<R>, Error> {
        let (completion, receiver) = oneshot::channel();
        let submission = Submission {
            frame: Self::frame(service, payload),
            completion,
        };
        let phase = self.shared.phase.lock().unwrap();
        if *phase != Phase::Open {
            return Err(Error::Closed);
        }
        self.sender.try_send(submission).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => Error::QueueFull,
            mpsc::error::TrySendError::Closed(_) => Error::Closed,
        })?;
        libertas_wake_up();
        Ok(Reply::new(receiver, self.shared.clone()))
    }
    fn submit<R: AvroDecode>(
        &self,
        service: u64,
        payload: &[u8],
    ) -> impl Future<Output = Result<R, Error>> + use<R> {
        // Copy into the single retained frame before returning a future. The
        // caller's inline buffer is never retained across an await or queued.
        let frame = Self::frame(service, payload);
        let handle = self.clone();
        async move {
            let (completion, receiver) = oneshot::channel();
            let submission = Submission { frame, completion };
            {
                let permit = handle.sender.reserve().await.map_err(|_| Error::Closed)?;
                let phase = handle.shared.phase.lock().unwrap();
                if *phase != Phase::Open {
                    return Err(Error::Closed);
                }
                permit.send(submission);
                libertas_wake_up();
            }
            Reply::<R>::new(receiver, handle.shared.clone()).await
        }
    }
}

/// An individually correlated pending reply. Dropping it cancels local waiting.
#[must_use = "await the reply, or drop it to abandon the call"]
pub struct Reply<R> {
    receiver: Option<oneshot::Receiver<Vec<u8>>>,
    shared: Arc<Shared>,
    marker: PhantomData<fn() -> R>,
}
impl<R> Reply<R> {
    fn new(receiver: oneshot::Receiver<Vec<u8>>, shared: Arc<Shared>) -> Self {
        Self {
            receiver: Some(receiver),
            shared,
            marker: PhantomData,
        }
    }
}
impl<R: AvroDecode> Future for Reply<R> {
    type Output = Result<R, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match Pin::new(this.receiver.as_mut().expect("reply already completed")).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                this.receiver = None;
                Poll::Ready(
                    result
                        .map_err(|_| Error::Closed)
                        .and_then(|v| decode_reply(&v)),
                )
            }
        }
    }
}
impl<R> Drop for Reply<R> {
    fn drop(&mut self) {
        if let Some(receiver) = self.receiver.take() {
            drop(receiver); // Mark cancellation before waking the pending-call sweeper.
            self.shared.wake();
        }
    }
}

/// Typed Avro command port. Clones share the dispatcher, never a callback slot.
pub struct Service<C, R> {
    handle: Handle,
    id: u64,
    marker: PhantomData<fn(C) -> R>,
}
impl<C, R> Clone for Service<C, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            id: self.id,
            marker: PhantomData,
        }
    }
}
impl<C: AvroEncode, R: AvroDecode> Service<C, R> {
    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    /// Encode now, then asynchronously await capacity and the individual reply.
    /// The future owns its bytes and does not borrow the command or service.
    pub fn call(
        &self,
        command: C,
    ) -> impl Future<Output = Result<R, Error>> + Send + 'static + use<C, R>
    where
        R: 'static,
    {
        let mut bytes = InlineByteBuffer::new();
        command.avro_encode(&mut bytes);
        self.handle.submit(self.id, bytes.as_slice())
    }

    /// Nonblocking submission, including from an ordinary main-thread callback.
    pub fn try_call(&self, command: C) -> Result<Reply<R>, Error> {
        let mut bytes = InlineByteBuffer::new();
        command.avro_encode(&mut bytes);
        self.handle.try_submit(self.id, bytes.as_slice())
    }
}

/// Host-thread registration handle. It is neither Send nor Sync. Dropping this
/// handle leaves the task-owned dispatcher running; [`Self::close`] closes it.
pub struct Dispatcher {
    driver: Rc<Driver>,
    handle: Handle,
}
impl Dispatcher {
    pub fn attach(capacity: usize) -> Result<Self, Error> {
        check_thread()?;
        if capacity == 0 || capacity > ::tokio::sync::Semaphore::MAX_PERMITS {
            return Err(Error::InvalidCapacity);
        }
        if SHUTDOWN_STATE.load(Ordering::Acquire) != SHUTDOWN_RUNNING {
            return Err(Error::Closed);
        }
        let task = libertas_get_task_id();
        let (sender, receiver) = mpsc::channel(capacity);
        let (shutdown, _) = watch::channel(false);
        let shared = Arc::new(Shared {
            phase: Mutex::new(Phase::Open),
            shutdown,
        });
        let driver = Rc::new(Driver {
            receiver: RefCell::new(receiver),
            handlers: RefCell::new(HashMap::new()),
            maintenance: RefCell::new(Vec::new()),
            next_service: core::cell::Cell::new(0),
            shared: shared.clone(),
            capacity,
            task,
        });
        unsafe {
            let env_ptr = &raw mut ENV;
            let context = (*env_ptr)
                .as_mut()
                .and_then(|env| env.contexts.get_mut(&task))
                .ok_or(Error::NotInitialized)?;
            if context.dispatcher.is_some() {
                return Err(Error::AlreadyAttached);
            }
            context.dispatcher = Some(driver.clone());
        }
        Ok(Self {
            driver,
            handle: Handle {
                sender,
                shared,
                task,
            },
        })
    }
    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    fn check_context(&self) -> Result<(), Error> {
        if !self.handle.shared.is_open() {
            return Err(Error::Closed);
        }
        check_thread()?;
        if libertas_get_task_id() != self.handle.task {
            return Err(Error::WrongTask);
        }
        Ok(())
    }

    /// Bind a nonblocking host-thread handler. Both types use the existing Avro
    /// codec; their Rust values and captured handler state need not be Send.
    pub fn bind<C, R, F>(&self, mut handler: F) -> Result<Service<C, R>, Error>
    where
        C: AvroEncode + AvroDecode + 'static,
        R: AvroEncode + AvroDecode + 'static,
        F: FnMut(C) -> R + 'static,
    {
        self.check_context()?;
        let id = self.driver.register(move |bytes, completion| {
            let result = decode::<C>(&bytes).map(|command| encode(&handler(command)));
            finish(completion, result);
        })?;
        Ok(Service {
            handle: self.handle.clone(),
            id,
            marker: PhantomData,
        })
    }

    /// Close queues and outstanding replies. Existing handles remain closed;
    /// this task cannot attach a replacement dispatcher or recycle its routes.
    pub fn close(&self) {
        self.driver.close();
    }
}

trait Maintenance {
    fn sweep(&self);
    fn close(&self);
}

pub(crate) struct Driver {
    receiver: RefCell<mpsc::Receiver<Submission>>,
    handlers: RefCell<HashMap<u64, Handler>>,
    maintenance: RefCell<Vec<Rc<dyn Maintenance>>>,
    next_service: core::cell::Cell<u64>,
    shared: Arc<Shared>,
    capacity: usize,
    task: LibertasTaskId,
}
impl Driver {
    fn register(&self, handler: impl FnMut(Vec<u8>, Completion) + 'static) -> Result<u64, Error> {
        if !self.shared.is_open() {
            return Err(Error::Closed);
        }
        let id = self
            .next_service
            .get()
            .checked_add(1)
            .ok_or(Error::IdExhausted)?;
        self.next_service.set(id);
        self.handlers
            .borrow_mut()
            .insert(id, Rc::new(RefCell::new(Box::new(handler))));
        Ok(id)
    }
    fn close(&self) {
        *self.shared.phase.lock().unwrap() = Phase::Closed;
        self.shared.shutdown.send_replace(true);
        self.receiver.borrow_mut().close();
        loop {
            let entry = self.receiver.borrow_mut().try_recv().ok();
            let Some(entry) = entry else { break };
            finish(entry.completion, Err(Error::Closed));
        }
        let handlers = core::mem::take(&mut *self.handlers.borrow_mut());
        let maintenance = core::mem::take(&mut *self.maintenance.borrow_mut());
        for state in &maintenance {
            state.close();
        }
        drop(handlers);
    }
    fn drive(&self) {
        let phase = *self.shared.phase.lock().unwrap();
        if phase != Phase::Open {
            self.close();
            if phase == Phase::Finishing {
                libertas_shutdown_complete();
            }
            return;
        }
        let maintenance = self.maintenance.borrow().clone();
        for state in maintenance {
            state.sweep();
        }
        // A bounded turn prevents a producer from starving timers and peers.
        for _ in 0..self.capacity {
            if !self.shared.is_open() {
                break;
            }
            let entry = self.receiver.borrow_mut().try_recv().ok();
            let Some(entry) = entry else { break };
            if entry.completion.is_closed() {
                continue;
            }
            match decode::<CommandFrame>(&entry.frame) {
                Ok(frame) => {
                    let handler = self.handlers.borrow().get(&frame.service).cloned();
                    match handler {
                        Some(handler) => (handler.borrow_mut())(frame.payload, entry.completion),
                        None => {
                            finish(entry.completion, Err(Error::Closed));
                        }
                    }
                }
                Err(error) => {
                    finish(entry.completion, Err(error));
                }
            }
        }
        if !self.receiver.borrow().is_empty() {
            self.shared.wake();
        }
    }
}

fn encode<T: AvroEncode>(value: &T) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.avro_encode(&mut bytes);
    bytes
}
fn decode<T: AvroDecode>(bytes: &[u8]) -> Result<T, Error> {
    let mut offset = 0;
    let value =
        T::avro_decode(bytes, &mut offset).map_err(|message| Error::Codec(message.into()))?;
    if offset != bytes.len() {
        return Err(Error::Codec("Trailing Avro bytes".into()));
    }
    Ok(value)
}

static HOST_THREAD: Mutex<Option<ThreadId>> = Mutex::new(None);
pub(crate) fn initialize_thread() {
    *HOST_THREAD.lock().unwrap() = Some(std::thread::current().id());
}
fn check_thread() -> Result<(), Error> {
    match *HOST_THREAD.lock().unwrap() {
        None => Err(Error::NotInitialized),
        Some(id) if id == std::thread::current().id() => Ok(()),
        Some(_) => Err(Error::WrongThread),
    }
}
fn drivers() -> Vec<Rc<Driver>> {
    unsafe {
        let env_ptr = &raw const ENV;
        (*env_ptr)
            .as_ref()
            .map(|env| {
                env.contexts
                    .values()
                    .filter_map(|ctx| ctx.dispatcher.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}
pub(crate) fn drive_all() {
    let task = libertas_get_task_id();
    for driver in drivers() {
        if SHUTDOWN_STATE.load(Ordering::Acquire) == SHUTDOWN_COMPLETED {
            break;
        }
        unsafe {
            ((*RUNTIME_API).set_task_id)(driver.task);
        }
        driver.drive();
        if SHUTDOWN_STATE.load(Ordering::Acquire) == SHUTDOWN_COMPLETED {
            return;
        }
    }
    unsafe {
        ((*RUNTIME_API).set_task_id)(task);
    }
}
pub(crate) fn request_shutdown(task: LibertasTaskId) -> bool {
    let driver = unsafe {
        let env_ptr = &raw const ENV;
        (*env_ptr)
            .as_ref()
            .and_then(|env| env.contexts.get(&task))
            .and_then(|ctx| ctx.dispatcher.clone())
    };
    if let Some(driver) = driver
        && driver.shared.is_open()
    {
        driver.shared.shutdown.send_replace(true);
        return true;
    }
    false
}
pub(crate) fn close_task(task: LibertasTaskId) {
    let driver = unsafe {
        let env_ptr = &raw const ENV;
        (*env_ptr)
            .as_ref()
            .and_then(|env| env.contexts.get(&task))
            .and_then(|ctx| ctx.dispatcher.clone())
    };
    if let Some(driver) = driver {
        driver.close();
    }
}
pub(crate) fn close_all() {
    for driver in drivers() {
        driver.close();
    }
    *HOST_THREAD.lock().unwrap() = None;
}

#[cfg(test)]
mod tests;
