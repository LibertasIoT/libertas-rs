use super::*;
use core::sync::atomic::{AtomicBool, AtomicUsize};
use std::{
    sync::MutexGuard,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
struct Sent {
    endpoint: u64,
    transaction: u32,
    opcode: u8,
    bytes: Vec<u8>,
    peer: u64,
}
static SENT: Mutex<Vec<Sent>> = Mutex::new(Vec::new());
static WAKEUPS: AtomicUsize = AtomicUsize::new(0);
static SHUTDOWNS: AtomicUsize = AtomicUsize::new(0);
static IMMEDIATE: AtomicBool = AtomicBool::new(false);
static ACKS: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static SCRATCH: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static READS: AtomicUsize = AtomicUsize::new(0);

extern "C" fn send(
    _: u16,
    endpoint: u64,
    transaction: u32,
    opcode: u8,
    data: *const u8,
    len: usize,
    peer: u64,
) {
    if endpoint == DEVICE_SYSTEM && opcode == OP_SYSTEM_WAKE_UP {
        WAKEUPS.fetch_add(1, Ordering::Relaxed);
        return;
    }
    if endpoint == DEVICE_SYSTEM && opcode == OP_APP_SHUT_DOWN {
        SHUTDOWNS.fetch_add(1, Ordering::Relaxed);
        return;
    }
    assert!(check_thread().is_ok(), "SDK operation escaped onto worker");
    let bytes = if len == 0 {
        Vec::new()
    } else {
        unsafe { slice::from_raw_parts(data, len) }.to_vec()
    };
    SENT.lock().unwrap().push(Sent {
        endpoint,
        transaction,
        opcode,
        bytes: bytes.clone(),
        peer,
    });
    if opcode == OP_ENDPOINT_REQ && IMMEDIATE.load(Ordering::Relaxed) {
        libertas_impl_device_callback(
            7,
            endpoint,
            OP_ENDPOINT_RSP,
            transaction,
            bytes.as_ptr().cast(),
            bytes.len(),
            endpoint,
        );
    }
}
extern "C" fn ack(dispatched: u8) {
    ACKS.lock().unwrap().push(dispatched);
}
extern "C" fn read(_: u16, _: u64, operation: u8, data: *const u8, len: usize) -> ReadResult {
    if operation == 0xe7 {
        return ReadResult {
            success: true,
            data: core::ptr::null(),
            data_len: 0,
        };
    }
    assert!(check_thread().is_ok());
    let value = READS.fetch_add(1, Ordering::Relaxed) as u32;
    let mut scratch = SCRATCH.lock().unwrap();
    scratch.clear();
    let request: crate::data::wire::Request =
        crate::data::wire::decode(unsafe { slice::from_raw_parts(data, len) }).unwrap();
    let result = match request.command {
        crate::data::wire::Command::Begin { .. } => crate::data::wire::Result::Begun(99),
        crate::data::wire::Command::Commit(_) | crate::data::wire::Command::Rollback(_) => {
            crate::data::wire::Result::Done
        }
        _ => crate::data::wire::Result::Value(Some(encode(&value))),
    };
    crate::data::wire::Response {
        id: request.id,
        result,
    }
    .avro_encode(&mut *scratch);
    ReadResult {
        success: true,
        data: scratch.as_ptr(),
        data_len: scratch.len(),
    }
}
struct Host {
    _guard: MutexGuard<'static, ()>,
    _api: Box<LibertasRuntimeApi>,
}
impl Host {
    fn new() -> Self {
        let guard = crate::tests::TEST_LOCK.lock().unwrap();
        SENT.lock().unwrap().clear();
        ACKS.lock().unwrap().clear();
        WAKEUPS.store(0, Ordering::Relaxed);
        SHUTDOWNS.store(0, Ordering::Relaxed);
        IMMEDIATE.store(false, Ordering::Relaxed);
        READS.store(0, Ordering::Relaxed);
        let mut api = Box::new(crate::tests::fake_runtime_api());
        api.device_send = send;
        api.device_read = read;
        api.endpoint_subscription_delivery_result = ack;
        __libertas_init_package((&mut *api as *mut LibertasRuntimeApi).cast());
        libertas_impl_init_task(7);
        Self {
            _guard: guard,
            _api: api,
        }
    }
    fn drive(&self) {
        libertas_impl_timer_driver(10, 10);
    }
    fn until(&self, mut done: impl FnMut() -> bool) {
        let start = Instant::now();
        while !done() {
            assert!(
                start.elapsed() < Duration::from_secs(5),
                "dispatcher stalled"
            );
            self.drive();
            thread::sleep(Duration::from_millis(1));
        }
    }
    fn deliver(&self, endpoint: u64, opcode: u8, transaction: u32, peer: u64, value: u32) {
        let mut bytes = alloc::vec![0];
        value.avro_encode(&mut bytes);
        self.raw(endpoint, opcode, transaction, peer, &bytes);
    }
    fn raw(&self, endpoint: u64, opcode: u8, transaction: u32, peer: u64, bytes: &[u8]) {
        libertas_impl_device_callback(
            7,
            endpoint,
            opcode,
            transaction,
            bytes.as_ptr().cast(),
            bytes.len(),
            peer,
        );
    }
}
impl Drop for Host {
    fn drop(&mut self) {
        __libertas_release_package();
        unsafe {
            RUNTIME_API = core::ptr::null_mut();
        }
        ASYNC_RUNTIME_API.store(core::ptr::null_mut(), Ordering::Release);
    }
}
fn poll<F: Future>(future: Pin<&mut F>) -> Poll<F::Output> {
    future.poll(&mut TaskContext::from_waker(core::task::Waker::noop()))
}
fn ready<F: Future>(future: Pin<&mut F>) -> F::Output {
    match poll(future) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("expected completion"),
    }
}
fn value(event: EndpointEvent<u32>) -> u32 {
    match event.message {
        LibertasEndpointMessage::Data(v) => v,
        _ => panic!("expected data"),
    }
}
fn runtime() -> ::tokio::runtime::Runtime {
    ::tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
}

#[test]
fn avro_services_work_on_both_executors_and_copy_host_results_before_reuse() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(4).unwrap();
    // The captured state stays on the host and deliberately is not Send.
    let seen = Rc::new(RefCell::new(Vec::new()));
    let local = seen.clone();
    #[derive(libertas_macros::LibertasKeyCodec)]
    enum Key {
        Record,
    }
    crate::data::__bind_schema::<Key, u32>();
    let data = crate::data::Data::<Key, u32>::current();
    let service = dispatcher
        .bind(move |command: u32| {
            local.borrow_mut().push(command);
            data.get(&crate::data::Target::all(), (&Key::Record).into())
                .unwrap()
        })
        .unwrap();
    let executor = ::tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_time()
        .build()
        .unwrap();
    let mut joins = Vec::new();
    for n in 0..40 {
        joins.push(executor.spawn(service.call(n)));
    }
    host.until(|| joins.iter().all(|join| join.is_finished()));
    let mut results: Vec<_> = joins
        .into_iter()
        .map(|join| executor.block_on(join).unwrap().unwrap())
        .collect();
    results.sort();
    assert_eq!(results, (0..40).collect::<Vec<_>>());
    assert_eq!(seen.borrow().len(), 40);
    let worker = thread::spawn(move || runtime().block_on(service.call(99)).unwrap());
    host.until(|| worker.is_finished());
    assert_eq!(worker.join().unwrap(), 40);
    assert_eq!(seen.borrow().last(), Some(&99));
}

#[test]
fn bounded_queue_cancellation_and_wakeup_callback_coexist() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(1).unwrap();
    let calls = Rc::new(core::cell::Cell::new(0));
    let local = calls.clone();
    let service = dispatcher
        .bind(move |n: u32| {
            local.set(local.get() + 1);
            n
        })
        .unwrap();
    let first = service.try_call(1).unwrap();
    assert!(matches!(service.try_call(2), Err(Error::QueueFull)));
    drop(first);
    let mut waiting = Box::pin(service.call(3));
    assert!(poll(waiting.as_mut()).is_pending());
    let wake_calls = Rc::new(core::cell::Cell::new(0));
    let local = wake_calls.clone();
    libertas_register_wakeup_callback(
        move |_| {
            local.set(local.get() + 1);
        },
        Box::new(()),
    );
    host.drive();
    assert_eq!(calls.get(), 0); // canceled before execution
    assert!(poll(waiting.as_mut()).is_pending()); // reserve capacity and enqueue
    host.drive();
    assert_eq!(ready(waiting.as_mut()), Ok(3));
    assert_eq!(calls.get(), 1);
    assert_eq!(wake_calls.get(), 2);
}

#[test]
fn concurrent_endpoint_replies_match_id_peer_and_opcode() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(8).unwrap();
    let (endpoint, mut events) = dispatcher.endpoint::<u32>(55, 4).unwrap();
    let mut first = Box::pin(endpoint.request(10));
    let mut second = Box::pin(endpoint.request(20));
    assert!(poll(first.as_mut()).is_pending());
    assert!(poll(second.as_mut()).is_pending());
    host.drive();
    let sent = SENT.lock().unwrap().clone();
    assert_eq!(sent.len(), 2);
    assert!(sent.iter().all(|entry| entry.endpoint == 55));
    assert_ne!(sent[0].transaction, sent[1].transaction);
    host.deliver(55, OP_ENDPOINT_RSP, sent[0].transaction, 999, 100);
    host.deliver(55, OP_ENDPOINT_DATA, sent[0].transaction, 55, 200);
    assert!(poll(first.as_mut()).is_pending());
    assert_eq!(value(runtime().block_on(events.recv()).unwrap()), 200);
    host.deliver(55, OP_ENDPOINT_EDIT_REQ, sent[1].transaction, 55, 20);
    let response = ready(second.as_mut()).unwrap();
    assert_eq!(response.opcode, OP_ENDPOINT_EDIT_REQ);
    assert_eq!(value(response), 20);
    host.deliver(55, OP_ENDPOINT_RSP, sent[0].transaction, 55, 10);
    assert_eq!(value(ready(first.as_mut()).unwrap()), 10);
    host.deliver(55, OP_ENDPOINT_RSP, sent[0].transaction, 55, 999); // duplicate
    assert!(poll(Box::pin(events.recv()).as_mut()).is_pending());
}

#[test]
fn waiter_precedes_immediate_reply_and_canceled_calls_release_pending_capacity() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(4).unwrap();
    let (endpoint, _events) = dispatcher.endpoint::<u32>(55, 1).unwrap();
    IMMEDIATE.store(true, Ordering::Relaxed);
    let mut call = Box::pin(endpoint.request(123));
    assert!(poll(call.as_mut()).is_pending());
    host.drive();
    assert_eq!(value(ready(call.as_mut()).unwrap()), 123);
    IMMEDIATE.store(false, Ordering::Relaxed);
    let mut abandoned = Box::pin(endpoint.request(1));
    assert!(poll(abandoned.as_mut()).is_pending());
    host.drive();
    let old_id = SENT.lock().unwrap().last().unwrap().transaction;
    let mut overflow = Box::pin(endpoint.request(2));
    assert!(poll(overflow.as_mut()).is_pending());
    host.drive();
    assert!(matches!(
        ready(overflow.as_mut()),
        Err(Error::TooManyRequests)
    ));
    drop(abandoned);
    host.drive();
    let mut replacement = Box::pin(endpoint.request(3));
    assert!(poll(replacement.as_mut()).is_pending());
    host.drive();
    let new_id = SENT.lock().unwrap().last().unwrap().transaction;
    assert_ne!(old_id, new_id);
    host.deliver(55, OP_ENDPOINT_RSP, old_id, 55, 999);
    assert!(poll(replacement.as_mut()).is_pending());
    host.deliver(55, OP_ENDPOINT_RSP, new_id, 55, 3);
    assert_eq!(value(ready(replacement.as_mut()).unwrap()), 3);
    unsafe {
        (*(&raw mut ENV)).as_mut().unwrap().trans_id = u32::MAX;
    }
    let mut exhausted = Box::pin(endpoint.request(4));
    assert!(poll(exhausted.as_mut()).is_pending());
    host.drive();
    assert!(matches!(ready(exhausted.as_mut()), Err(Error::IdExhausted)));
}

#[test]
fn subscription_validation_bounded_events_and_deferred_reply_context() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(4).unwrap();
    let (_endpoint, mut events) = dispatcher.endpoint::<u32>(55, 1).unwrap();
    host.raw(55, OP_ENDPOINT_SUB_REQ, 9, 1001, &[0, 0x80]); // truncated
    assert_eq!(*ACKS.lock().unwrap(), [0]);
    host.deliver(55, OP_ENDPOINT_SUB_REQ, 10, 1002, 7);
    assert_eq!(*ACKS.lock().unwrap(), [0, 1]);
    host.deliver(55, OP_ENDPOINT_SUB_REQ, 11, 1003, 8); // full queue, explicit rejection
    let sent = SENT.lock().unwrap().clone();
    assert!(sent.iter().any(|s| s.opcode == OP_ENDPOINT_RSP
        && s.transaction == 11
        && s.peer == 1003
        && s.bytes == [2, 8]));
    assert!(
        sent.iter()
            .any(|s| s.opcode == OP_ENDPOINT_REMOVE_PEER && s.peer == 1003)
    );
    let event = runtime().block_on(events.recv()).unwrap();
    assert_eq!(event.peer, 1002);
    assert_eq!(event.transaction, 10);
    let mut reply = Box::pin(event.status(LibertasEndpointStandardStatus::Unavailable));
    assert!(poll(reply.as_mut()).is_pending());
    host.drive();
    assert_eq!(ready(reply.as_mut()), Ok(()));
    let sent = SENT.lock().unwrap().clone();
    let tail = &sent[sent.len() - 2..];
    assert_eq!(tail[0].opcode, OP_ENDPOINT_RSP);
    assert_eq!(tail[0].transaction, 10);
    assert_eq!(tail[0].peer, 1002);
    assert_eq!(tail[1].opcode, OP_ENDPOINT_REMOVE_PEER);
    host.deliver(55, OP_ENDPOINT_DATA, 0, 55, 4);
    host.deliver(55, OP_ENDPOINT_DATA, 0, 55, 5);
    assert!(matches!(
        runtime().block_on(events.recv()),
        Err(Error::EventsLost(1))
    ));
    assert_eq!(value(runtime().block_on(events.recv()).unwrap()), 4);
}

#[test]
fn subscription_status_and_signals_do_not_complete_regular_requests() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(4).unwrap();
    let (endpoint, mut events) = dispatcher.endpoint::<u32>(55, 2).unwrap();
    let mut subscription = Box::pin(endpoint.subscribe(8));
    assert!(poll(subscription.as_mut()).is_pending());
    host.drive();
    let id = ready(subscription.as_mut()).unwrap();
    let mut request = Box::pin(endpoint.request(9));
    assert!(poll(request.as_mut()).is_pending());
    host.drive();
    host.raw(55, OP_ENDPOINT_RSP, id, 55, &[2, 14]);
    let status = runtime().block_on(events.recv()).unwrap();
    assert_eq!(
        status.message,
        LibertasEndpointMessage::Status(LibertasEndpointStandardStatus::Unavailable)
    );
    host.raw(55, OP_ENDPOINT_PEER_DOWN, 0, 55, &[]);
    assert_eq!(
        runtime().block_on(events.recv()).unwrap().opcode,
        OP_ENDPOINT_PEER_DOWN
    );
    assert!(poll(request.as_mut()).is_pending());
    host.raw(55, OP_ENDPOINT_RSP, id, 55, &[2, 14]); // duplicate opening reply
    assert!(poll(Box::pin(events.recv()).as_mut()).is_pending());
    dispatcher.close();
    assert!(matches!(ready(request.as_mut()), Err(Error::Closed)));
    assert!(matches!(
        runtime().block_on(events.recv()),
        Err(Error::Closed)
    ));
}

#[test]
fn callbacks_reenter_sdk_and_shutdown_bypasses_full_queue() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(1).unwrap();
    let service = dispatcher
        .bind(|_: bool| {
            let id = libertas_timer_new_interval(
                1,
                |timer, _, _| libertas_timer_destroy(timer),
                Box::new(()),
            );
            libertas_timer_update_interval(id, 2);
            libertas_register_device_listener(
                66,
                |_, _, _, _, _, _| {
                    libertas_timer_new_interval(0, |_, _, _| {}, Box::new(()));
                },
                Box::new(()),
            );
            true
        })
        .unwrap();
    let mut call = Box::pin(service.try_call(true).unwrap());
    host.drive();
    assert_eq!(ready(call.as_mut()), Ok(true));
    host.raw(66, OP_ENDPOINT_DATA, 0, 66, &[]);
    let mut queued = Box::pin(service.try_call(true).unwrap()); // must not run twice
    let handle = dispatcher.handle();
    assert_eq!(handle.complete_shutdown(), Err(Error::ShutdownNotRequested));
    host.raw(DEVICE_SYSTEM, OP_APP_SHUT_DOWN, 0, 0, &[]);
    runtime().block_on(handle.shutdown_requested());
    assert_eq!(SHUTDOWNS.load(Ordering::Relaxed), 0);
    let worker = thread::spawn(move || {
        handle.complete_shutdown().unwrap();
        handle
    });
    let retained = worker.join().unwrap();
    host.drive();
    assert_eq!(SHUTDOWNS.load(Ordering::Relaxed), 1);
    assert_eq!(ready(queued.as_mut()), Err(Error::Closed));
    let wakes = WAKEUPS.load(Ordering::Relaxed);
    assert!(matches!(service.try_call(false), Err(Error::Closed)));
    assert_eq!(retained.complete_shutdown(), Err(Error::Closed));
    assert_eq!(WAKEUPS.load(Ordering::Relaxed), wakes);
}

#[test]
fn teardown_closes_retained_handles_and_preserves_existing_endpoint_listener() {
    let _host = Host::new();
    let dispatcher = Dispatcher::attach(2).unwrap();
    assert!(matches!(Dispatcher::attach(2), Err(Error::AlreadyAttached)));
    assert_eq!(
        thread::spawn(|| Dispatcher::attach(2).err().unwrap())
            .join()
            .unwrap(),
        Error::WrongThread
    );
    libertas_register_endpoint_status_listener::<u32, _>(
        44,
        |_, _, _, _, _, _| LibertasEndpointHandlerResult::Handled,
        Box::new(()),
    );
    assert!(matches!(
        dispatcher.endpoint::<u32>(44, 1),
        Err(Error::EndpointInUse)
    ));
    let service = dispatcher.bind(|n: u32| n).unwrap();
    let mut queued = Box::pin(service.try_call(1).unwrap());
    libertas_impl_remove_task(7);
    assert_eq!(ready(queued.as_mut()), Err(Error::Closed));
    __libertas_release_package();
    let wakes = WAKEUPS.load(Ordering::Relaxed);
    assert!(matches!(service.try_call(2), Err(Error::Closed)));
    assert_eq!(runtime().block_on(service.call(3)), Err(Error::Closed));
    assert_eq!(WAKEUPS.load(Ordering::Relaxed), wakes);
}

#[test]
fn command_frames_are_avro_records_and_reject_trailing_bytes() {
    let command = CommandFrame {
        service: 3,
        payload: encode(&42u32),
    };
    let mut bytes = encode(&command);
    assert_eq!(bytes, [6, 2, 84]); // Avro long route, bytes length, encoded u32
    let decoded = decode::<CommandFrame>(&bytes).unwrap();
    assert_eq!(decoded.service, 3);
    assert_eq!(decode::<u32>(&decoded.payload), Ok(42));
    bytes.push(0);
    assert!(matches!(
        decode::<CommandFrame>(&bytes),
        Err(Error::Codec(_))
    ));
    let failure = encode(&ReplyFrame::Failure(Error::Codec("bad datum".into())));
    assert_eq!(
        decode_reply::<u32>(&failure),
        Err(Error::Codec("bad datum".into()))
    );
    let success = encode(&ReplyFrame::Value(encode(&123u32)));
    assert_eq!(success, [0, 4, 246, 1]); // Value union branch, bytes, encoded u32
    assert_eq!(decode_reply::<u32>(&success), Ok(123));
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Handle>();
    send_sync::<Service<u32, u32>>();
    send_sync::<Endpoint<u32>>();
}

#[test]
fn parallel_coroutines_share_one_endpoint_listener() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(8).unwrap();
    let (endpoint, _events) = dispatcher.endpoint::<u32>(55, 16).unwrap();
    let executor = ::tokio::runtime::Builder::new_multi_thread()
        .worker_threads(3)
        .build()
        .unwrap();
    let joins: Vec<_> = (0..16)
        .map(|n| executor.spawn(endpoint.request(n)))
        .collect();
    host.until(|| SENT.lock().unwrap().len() == 16);
    for sent in SENT.lock().unwrap().clone().into_iter().rev() {
        let request = decode::<u32>(&sent.bytes[1..]).unwrap();
        host.deliver(
            sent.endpoint,
            OP_ENDPOINT_RSP,
            sent.transaction,
            sent.endpoint,
            request,
        );
    }
    for (n, join) in joins.into_iter().enumerate() {
        assert_eq!(value(executor.block_on(join).unwrap().unwrap()), n as u32);
    }
}

#[test]
fn close_racing_with_producers_never_wakes_a_closed_runtime() {
    let host = Host::new();
    let dispatcher = Dispatcher::attach(2).unwrap();
    let service = dispatcher.bind(|n: u32| n).unwrap();
    let start = Arc::new(std::sync::Barrier::new(5));
    let producers: Vec<_> = (0..4)
        .map(|_| {
            let start = start.clone();
            let service = service.clone();
            thread::spawn(move || {
                start.wait();
                loop {
                    match service.try_call(1) {
                        Err(Error::Closed) => break,
                        Ok(reply) => drop(reply),
                        Err(Error::QueueFull) => thread::yield_now(),
                        _ => panic!("unexpected dispatch result"),
                    }
                }
            })
        })
        .collect();
    start.wait();
    host.drive();
    dispatcher.close();
    let wakes = WAKEUPS.load(Ordering::Relaxed);
    for producer in producers {
        producer.join().unwrap();
    }
    assert_eq!(WAKEUPS.load(Ordering::Relaxed), wakes);
}

#[test]
fn canceled_begin_releases_a_handle_already_delivered_to_the_coroutine() {
    let host = Host::new();
    #[derive(libertas_macros::LibertasKeyCodec)]
    enum Key {
        Record,
    }
    crate::data::__bind_schema::<Key, u32>();
    let local = crate::data::Data::<Key, u32>::current();
    let dispatcher = Dispatcher::attach(4).unwrap();
    let data = dispatcher.data(&local).unwrap();
    fn send_sync<T: Send + Sync>(_: &T) {}
    send_sync(&data);
    let target = crate::data::Target::all();
    let mut future = Box::pin(data.try_begin(&target, crate::data::Mode::ReadWrite));
    assert!(poll(future.as_mut()).is_pending());
    host.drive();
    assert!(poll(future.as_mut()).is_pending());
    host.drive();
    // Delivery has completed, but the caller has not received/owned its handle.
    drop(future);
    host.drive();
    let sent = SENT.lock().unwrap();
    assert!(sent.iter().any(|s| s.opcode == 0xeb
        && crate::data::wire::decode::<(bool, u64)>(&s.bytes) == Ok((true, 99))));
}
