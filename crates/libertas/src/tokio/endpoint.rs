use super::*;
use core::sync::atomic::AtomicU64;
use hashbrown::HashSet;

#[derive(LibertasAvroEncode, LibertasAvroDecode)]
enum Command {
    Request(Vec<u8>),
    Subscribe(Vec<u8>),
    Response {
        transaction: u32,
        peer: LibertasPeer,
        edit: bool,
        data: Vec<u8>,
    },
    Status {
        transaction: u32,
        peer: LibertasPeer,
        status: u8,
        subscription: bool,
    },
    Report {
        peer: Option<LibertasPeer>,
        data: Vec<u8>,
    },
    Alive(Option<LibertasPeer>),
    Remove(LibertasPeer),
    ForgetSubscription(u32),
}
#[derive(LibertasAvroEncode, LibertasAvroDecode)]
enum Message {
    Data(Vec<u8>),
    Invalid,
    Status(u8),
    Signal,
}
#[derive(LibertasAvroEncode, LibertasAvroDecode)]
struct EventFrame {
    opcode: u8,
    transaction: u32,
    peer: LibertasPeer,
    message: Message,
}
struct Encoded(Vec<u8>);
impl AvroEncode for Encoded {
    fn avro_encode(&self, bytes: &mut (impl crate::ByteBuffer + ?Sized)) {
        bytes.extend_from_slice(&self.0);
    }
}

/// Thread-agnostic endpoint commands. One handle owns the endpoint listener;
/// clone it for simultaneous calls, including calls to the same endpoint.
pub struct Endpoint<T> {
    handle: Handle,
    service: u64,
    endpoint: LibertasEndpoint,
    marker: PhantomData<fn() -> T>,
}
impl<T> Clone for Endpoint<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            service: self.service,
            endpoint: self.endpoint,
            marker: PhantomData,
        }
    }
}
impl<T: AvroEncode + AvroDecode + 'static> Endpoint<T> {
    pub fn id(&self) -> LibertasEndpoint {
        self.endpoint
    }
    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    /// Await the correlated Response or EditRequest, preserving standard status
    /// and invalid-message results. Subscription data never completes this call.
    pub fn request(
        &self,
        value: T,
    ) -> impl Future<Output = Result<EndpointEvent<T>, Error>> + Send + 'static + use<T> {
        let command = encode(&Command::Request(encode(&value)));
        let endpoint = self.clone();
        async move {
            let frame = endpoint.handle.submit(endpoint.service, &command).await?;
            endpoint.event(frame)
        }
    }

    /// Send a subscription request. Data, statuses, and signaling arrive through
    /// [`Events`]. The returned ID identifies this subscription's request.
    pub fn subscribe(
        &self,
        value: T,
    ) -> impl Future<Output = Result<LibertasTransId, Error>> + Send + 'static + use<T> {
        self.command(Command::Subscribe(encode(&value)))
    }

    /// Abandon an outstanding subscription opening reply locally. Correlation
    /// is normally removed automatically when its opening Response arrives.
    /// This does not revoke host membership or stop uncorrelated Data reports;
    /// use the application protocol and normal endpoint lifecycle for that.
    pub fn forget_subscription(
        &self,
        transaction: LibertasTransId,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        self.unit(Command::ForgetSubscription(transaction))
    }

    pub fn report(
        &self,
        value: T,
        peer: Option<LibertasPeer>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        self.unit(Command::Report {
            peer,
            data: encode(&value),
        })
    }
    pub fn peer_alive(
        &self,
        peer: Option<LibertasPeer>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        self.unit(Command::Alive(peer))
    }
    pub fn remove_subscriber(
        &self,
        peer: LibertasPeer,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        self.unit(Command::Remove(peer))
    }
    fn command<R: AvroDecode + 'static>(
        &self,
        command: Command,
    ) -> impl Future<Output = Result<R, Error>> + Send + 'static + use<T, R> {
        let bytes = encode(&command);
        let handle = self.handle.clone();
        let service = self.service;
        async move { handle.submit(service, &bytes).await }
    }
    fn unit(
        &self,
        command: Command,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        let call = self.command::<bool>(command);
        async move { call.await.map(|_| ()) }
    }
    fn event(&self, frame: EventFrame) -> Result<EndpointEvent<T>, Error> {
        let message = match frame.message {
            Message::Data(data) => LibertasEndpointMessage::Data(decode(&data)?),
            Message::Invalid => LibertasEndpointMessage::InvalidMessage,
            Message::Status(code) => LibertasEndpointMessage::Status(
                LibertasEndpointStandardStatus::from_code(code)
                    .ok_or(Error::Codec("Invalid status".into()))?,
            ),
            Message::Signal => LibertasEndpointMessage::NoPayload,
        };
        Ok(EndpointEvent {
            endpoint: self.clone(),
            opcode: frame.opcode,
            transaction: frame.transaction,
            peer: frame.peer,
            message,
        })
    }
}

/// Owned message and its original reply context. Server peer handles remain
/// opaque; passing this context across awaits never changes the reply target.
pub struct EndpointEvent<T> {
    endpoint: Endpoint<T>,
    pub opcode: u8,
    pub transaction: LibertasTransId,
    pub peer: LibertasPeer,
    pub message: LibertasEndpointMessage<T>,
}
impl<T: AvroEncode + AvroDecode + 'static> EndpointEvent<T> {
    pub fn endpoint(&self) -> LibertasEndpoint {
        self.endpoint.id()
    }
    pub fn response(
        self,
        value: T,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        self.reply(value, false)
    }
    pub fn edit_request(
        self,
        value: T,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        self.reply(value, true)
    }
    fn reply(
        self,
        value: T,
        edit: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        let valid = self.opcode == OP_ENDPOINT_REQ || self.opcode == OP_ENDPOINT_SUB_REQ;
        let future = self.endpoint.unit(Command::Response {
            transaction: self.transaction,
            peer: self.peer,
            edit,
            data: encode(&value),
        });
        async move {
            if !valid {
                return Err(Error::NotARequest);
            }
            future.await
        }
    }
    pub fn status(
        self,
        status: LibertasEndpointStandardStatus,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'static + use<T> {
        let valid = self.opcode == OP_ENDPOINT_REQ || self.opcode == OP_ENDPOINT_SUB_REQ;
        let subscription = self.opcode == OP_ENDPOINT_SUB_REQ;
        let endpoint = self.endpoint;
        let peer = self.peer;
        let future = endpoint.unit(Command::Status {
            transaction: self.transaction,
            peer,
            status: status.code(),
            subscription,
        });
        async move {
            if !valid {
                return Err(Error::NotARequest);
            }
            future.await
        }
    }
}

/// Bounded receiver for unsolicited data, incoming requests, subscription
/// statuses, and peer signaling. Overflow is explicit; it never steals a
/// pending request's reply. There is one receiver, movable between tasks.
pub struct Events<T> {
    receiver: mpsc::Receiver<Vec<u8>>,
    lost: Arc<AtomicU64>,
    endpoint: Endpoint<T>,
}
impl<T: AvroEncode + AvroDecode + 'static> Events<T> {
    pub async fn recv(&mut self) -> Result<EndpointEvent<T>, Error> {
        let lost = self.lost.swap(0, Ordering::AcqRel);
        if lost != 0 {
            return Err(Error::EventsLost(lost));
        }
        let bytes = self.receiver.recv().await.ok_or(Error::Closed)?;
        self.endpoint.event(decode(&bytes)?)
    }
}

struct State {
    pending: RefCell<HashMap<LibertasTransId, Completion>>,
    subscriptions: RefCell<HashSet<LibertasTransId>>,
    events: RefCell<Option<mpsc::Sender<Vec<u8>>>>,
    lost: Arc<AtomicU64>,
    capacity: usize,
    shared: Arc<Shared>,
}
impl Maintenance for State {
    fn sweep(&self) {
        self.pending
            .borrow_mut()
            .retain(|_, reply| !reply.is_closed());
    }
    fn close(&self) {
        let pending = core::mem::take(&mut *self.pending.borrow_mut());
        self.subscriptions.borrow_mut().clear();
        self.events.borrow_mut().take();
        for (_, reply) in pending {
            finish(reply, Err(Error::Closed));
        }
    }
}
impl State {
    fn deliver<T: AvroEncode>(
        &self,
        endpoint: LibertasEndpoint,
        opcode: u8,
        message: LibertasEndpointMessage<T>,
        transaction: LibertasTransId,
        peer: LibertasPeer,
    ) -> LibertasEndpointHandlerResult {
        if !self.shared.is_open() {
            return LibertasEndpointHandlerResult::Status(
                LibertasEndpointStandardStatus::Unavailable,
            );
        }
        let message = match message {
            LibertasEndpointMessage::Data(value) => Message::Data(encode(&value)),
            LibertasEndpointMessage::InvalidMessage => Message::Invalid,
            LibertasEndpointMessage::Status(status) => Message::Status(status.code()),
            LibertasEndpointMessage::NoPayload => Message::Signal,
        };
        let bytes = encode(&EventFrame {
            opcode,
            transaction,
            peer,
            message,
        });
        if opcode == OP_ENDPOINT_RSP || opcode == OP_ENDPOINT_EDIT_REQ {
            // For client responses the ABI peer is the serving endpoint. Server
            // request peers remain opaque and are never compared to identities.
            if peer != endpoint {
                return LibertasEndpointHandlerResult::Handled;
            }
            let reply = self.pending.borrow_mut().remove(&transaction);
            if let Some(reply) = reply {
                finish(reply, Ok(bytes));
                return LibertasEndpointHandlerResult::Handled;
            }
            if !self.subscriptions.borrow_mut().remove(&transaction) {
                return LibertasEndpointHandlerResult::Handled; // late/duplicate/canceled reply
            }
        }
        let sender = self.events.borrow().clone();
        let sent = sender
            .as_ref()
            .is_some_and(|sender| sender.try_send(bytes).is_ok());
        if !sent {
            if opcode == OP_ENDPOINT_REQ || opcode == OP_ENDPOINT_SUB_REQ {
                return LibertasEndpointHandlerResult::Status(
                    LibertasEndpointStandardStatus::ResourceExhausted,
                );
            }
            self.lost.fetch_add(1, Ordering::Release);
        }
        LibertasEndpointHandlerResult::Handled
    }
}

impl Dispatcher {
    /// Register this task's sole typed listener for the endpoint. `capacity`
    /// independently bounds queued events, pending requests, and subscription
    /// correlations. Existing listeners are rejected without replacing them.
    pub fn endpoint<T>(
        &self,
        endpoint: LibertasEndpoint,
        capacity: usize,
    ) -> Result<(Endpoint<T>, Events<T>), Error>
    where
        T: AvroEncode + AvroDecode + 'static,
    {
        self.check_context()?;
        if capacity == 0 || capacity > ::tokio::sync::Semaphore::MAX_PERMITS {
            return Err(Error::InvalidCapacity);
        }
        let occupied = unsafe {
            let env_ptr = &raw const ENV;
            (*env_ptr)
                .as_ref()
                .and_then(|env| env.contexts.get(&self.handle.task))
                .is_none_or(|context| context.device_callbacks.contains_key(&endpoint))
        };
        if occupied {
            return Err(Error::EndpointInUse);
        }
        let (sender, receiver) = mpsc::channel(capacity);
        let lost = Arc::new(AtomicU64::new(0));
        let state = Rc::new(State {
            pending: RefCell::new(HashMap::new()),
            subscriptions: RefCell::new(HashSet::new()),
            events: RefCell::new(Some(sender)),
            lost: lost.clone(),
            capacity,
            shared: self.handle.shared.clone(),
        });
        let command_state = state.clone();
        let service =
            self.driver
                .register(move |bytes, reply| match decode::<Command>(&bytes) {
                    Ok(command) => dispatch(endpoint, &command_state, command, reply),
                    Err(error) => {
                        finish(reply, Err(error));
                    }
                })?;
        let listener_state = state.clone();
        libertas_register_endpoint_status_listener::<T, _>(
            endpoint,
            move |id, opcode, message, _, transaction, peer| {
                listener_state.deliver(id, opcode, message, transaction, peer)
            },
            Box::new(()),
        );
        self.driver.maintenance.borrow_mut().push(state);
        let endpoint = Endpoint {
            handle: self.handle.clone(),
            service,
            endpoint,
            marker: PhantomData,
        };
        Ok((
            endpoint.clone(),
            Events {
                receiver,
                lost,
                endpoint,
            },
        ))
    }
}

fn transaction_id() -> Result<LibertasTransId, Error> {
    unsafe {
        let env_ptr = &raw mut ENV;
        let env = (*env_ptr).as_mut().ok_or(Error::Closed)?;
        let id = env.trans_id.checked_add(1).ok_or(Error::IdExhausted)?;
        env.trans_id = id;
        Ok(id)
    }
}
fn dispatch(endpoint: LibertasEndpoint, state: &State, command: Command, reply: Completion) {
    match command {
        Command::Request(data) => send_request(endpoint, state, data, reply, false),
        Command::Subscribe(data) => send_request(endpoint, state, data, reply, true),
        Command::Response {
            transaction,
            peer,
            edit,
            data,
        } => {
            if edit {
                libertas_endpoint_edit_request(endpoint, &Encoded(data), transaction, peer);
            } else {
                libertas_endpoint_response(endpoint, &Encoded(data), transaction, peer);
            }
            finish(reply, Ok(encode(&true)));
        }
        Command::Status {
            transaction,
            peer,
            status,
            subscription,
        } => {
            if let Some(status) = LibertasEndpointStandardStatus::from_code(status) {
                libertas_endpoint_status_response(endpoint, status, transaction, peer);
                if subscription {
                    libertas_endpoint_remove_subscriber(endpoint, peer);
                }
                finish(reply, Ok(encode(&true)));
            } else {
                finish(reply, Err(Error::Codec("Invalid status".into())));
            }
        }
        Command::Report { peer, data } => {
            libertas_endpoint_report(endpoint, &Encoded(data), peer);
            finish(reply, Ok(encode(&true)));
        }
        Command::Alive(peer) => {
            libertas_endpoint_peer_alive(endpoint, peer);
            finish(reply, Ok(encode(&true)));
        }
        Command::Remove(peer) => {
            libertas_endpoint_remove_subscriber(endpoint, peer);
            finish(reply, Ok(encode(&true)));
        }
        Command::ForgetSubscription(id) => {
            state.subscriptions.borrow_mut().remove(&id);
            finish(reply, Ok(encode(&true)));
        }
    }
}
fn send_request(
    endpoint: LibertasEndpoint,
    state: &State,
    data: Vec<u8>,
    reply: Completion,
    subscription: bool,
) {
    state.sweep();
    let count = if subscription {
        state.subscriptions.borrow().len()
    } else {
        state.pending.borrow().len()
    };
    if count >= state.capacity {
        finish(reply, Err(Error::TooManyRequests));
        return;
    }
    let transaction = match transaction_id() {
        Ok(id) => id,
        Err(error) => {
            finish(reply, Err(error));
            return;
        }
    };
    let mut bytes = InlineByteBuffer::with_capacity(data.len() + 1);
    bytes.push(LibertasEndpointStatus::Success as u8);
    bytes.extend_from_slice(&data);
    let subscription_reply = if subscription {
        state.subscriptions.borrow_mut().insert(transaction);
        Some(reply)
    } else {
        // Install the waiter before sending, with no map borrow across the FFI.
        state.pending.borrow_mut().insert(transaction, reply);
        None
    };
    __libertas_device_send_raw(
        PROTOCOL_LIBERTAS,
        endpoint,
        if subscription {
            OP_ENDPOINT_SUB_REQ
        } else {
            OP_ENDPOINT_REQ
        },
        0,
        transaction,
        bytes.as_slice().as_ptr(),
        bytes.len(),
    );
    if let Some(reply) = subscription_reply {
        finish(reply, Ok(encode(&transaction)));
    }
}
