//! Coroutine storage handles. Host calls stay on the dispatcher; async storage
//! execution and completion are owned by the host's backend.
use super::*;
use crate::data::{
    self as core_data, DataError, EncodedKey, Entry, KeyCodec, KeyRange, KeyRef, Mode, Mutation,
    Page, ScanOptions, Target, request, wire,
};
use core::cell::Cell;
use std::sync::Mutex;

type Work = (u64, bool, wire::Command);
struct Control {
    cancellations: Mutex<Vec<(bool, u64)>>,
    beginnings: Mutex<HashMap<u64, Option<u64>>>,
    next: core::sync::atomic::AtomicU64,
    handle: Handle,
}
impl Control {
    fn cancel(&self, transaction: bool, id: u64) {
        self.cancellations.lock().unwrap().push((transaction, id));
        self.handle.shared.wake();
    }
}
struct Pending {
    calls: RefCell<HashMap<u64, (u64, Completion)>>,
    control: Arc<Control>,
}
impl Pending {
    fn deliver(&self, ticket: u64, completion: Completion, result: wire::Result) {
        if let wire::Result::Begun(id) = result {
            let mut beginnings = self.control.beginnings.lock().unwrap();
            if let Some(owner) = beginnings.get_mut(&ticket) {
                *owner = Some(id);
            } else {
                core_data::cancel(true, id);
            }
        }
        finish(completion, Ok(encode(&result)));
    }
}
// Ownership exists before dispatch and survives delivery into the oneshot. A
// dropped Begin future therefore releases even an already delivered handle.
struct BeginClaim {
    control: Arc<Control>,
    ticket: u64,
}
impl Drop for BeginClaim {
    fn drop(&mut self) {
        let id = self
            .control
            .beginnings
            .lock()
            .unwrap()
            .remove(&self.ticket)
            .flatten();
        if let Some(id) = id {
            self.control.cancel(true, id);
        }
    }
}

impl Maintenance for Pending {
    fn sweep(&self) {
        let cancellations = core::mem::take(&mut *self.control.cancellations.lock().unwrap());
        for (transaction, id) in cancellations {
            core_data::cancel(transaction, id);
        }
        let ids = self.calls.borrow().keys().copied().collect::<Vec<_>>();
        for id in ids {
            let canceled = self
                .calls
                .borrow()
                .get(&id)
                .is_some_and(|(_, c)| c.is_closed());
            if canceled {
                self.calls.borrow_mut().remove(&id);
                core_data::cancel(false, id);
                continue;
            }
            let mut bytes = InlineByteBuffer::new();
            id.avro_encode(&mut bytes);
            let raw = crate::__libertas_device_read_raw(
                crate::PROTOCOL_LIBERTAS,
                crate::DEVICE_SYSTEM,
                0xea,
                bytes.as_slice().as_ptr(),
                bytes.len(),
            );
            if raw.success {
                let result =
                    core_data::reply(id, raw).unwrap_or_else(|e| wire::Result::Failure(e as u32));
                if let Some((ticket, completion)) = self.calls.borrow_mut().remove(&id) {
                    self.deliver(ticket, completion, result);
                }
            }
        }
    }
    fn close(&self) {
        let calls = core::mem::take(&mut *self.calls.borrow_mut());
        for (id, (_, completion)) in calls {
            core_data::cancel(false, id);
            finish(completion, Err(Error::Closed));
        }
        for (t, id) in core::mem::take(&mut *self.control.cancellations.lock().unwrap()) {
            core_data::cancel(t, id);
        }
    }
}
impl Dispatcher {
    pub fn data<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        &self,
        data: &core_data::Data<K, V>,
    ) -> Result<Data<K, V>, Error> {
        self.check_context()?;
        if !data.valid() {
            return Err(Error::WrongTask);
        }
        let core = data.duplicate();
        let control = Arc::new(Control {
            cancellations: Mutex::new(Vec::new()),
            beginnings: Mutex::new(HashMap::new()),
            next: core::sync::atomic::AtomicU64::new(1),
            handle: self.handle.clone(),
        });
        let pending = Rc::new(Pending {
            calls: RefCell::new(HashMap::new()),
            control: control.clone(),
        });
        let state = pending.clone();
        let id = self.driver.register(move |bytes, completion| {
            let work = decode::<Work>(&bytes);
            let (ticket, checked, command) = match work {
                Ok(v) => v,
                Err(e) => {
                    finish(completion, Err(e));
                    return;
                }
            };
            let request = match core.request(checked, command) {
                Ok(r) => r,
                Err(e) => {
                    finish(completion, Ok(encode(&wire::Result::Failure(e as u32))));
                    return;
                }
            };
            match core_data::host_call(0xe9, &request) {
                Ok(Some(result)) => state.deliver(ticket, completion, result),
                Ok(None) => {
                    state
                        .calls
                        .borrow_mut()
                        .insert(request.id, (ticket, completion));
                }
                Err(e) => finish(completion, Ok(encode(&wire::Result::Failure(e as u32)))),
            }
        })?;
        self.driver.maintenance.borrow_mut().push(pending);
        let failure = self.bind::<u32, bool, _>(|status| {
            if status == 0 {
                core_data::authorize_checked().is_ok()
            } else {
                core_data::fatal(DataError::status(status))
            }
        })?;
        Ok(Data {
            service: Service {
                handle: self.handle.clone(),
                id,
                marker: PhantomData,
            },
            failure,
            control,
            marker: PhantomData,
        })
    }
}
pub struct Data<K, V> {
    service: Service<Work, wire::Result>,
    failure: Service<u32, bool>,
    control: Arc<Control>,
    marker: PhantomData<fn() -> (K, V)>,
}
impl<K, V> Clone for Data<K, V> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
            failure: self.failure.clone(),
            control: self.control.clone(),
            marker: PhantomData,
        }
    }
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Data<K, V> {
    async fn call_with_ticket(
        &self,
        ticket: u64,
        checked: bool,
        command: request::Command<'_, K, V>,
    ) -> core::result::Result<wire::Result, DataError> {
        let reply = {
            let bytes = request::work(ticket, checked, command)?;
            self.service
                .handle
                .submit(self.service.id, bytes.as_slice())
        };
        let result = reply.await.map_err(|_| DataError::StorageError)?;
        match result {
            wire::Result::Failure(code) => Err(DataError::status(code)),
            r => Ok(r),
        }
    }
    async fn call(
        &self,
        checked: bool,
        command: request::Command<'_, K, V>,
    ) -> core::result::Result<wire::Result, DataError> {
        self.call_with_ticket(0, checked, command).await
    }
    async fn operation(
        &self,
        target: &Target<K>,
        checked: bool,
        op: request::Operation<'_, K, V>,
    ) -> core::result::Result<wire::Result, DataError> {
        self.call(
            checked,
            request::Command::Execute {
                target,
                operation: op,
            },
        )
        .await
    }
    async fn ordinary<T>(&self, result: core::result::Result<T, DataError>) -> T {
        match result {
            Ok(v) => v,
            Err(e) => {
                let _ = self.failure.call(e as u32).await;
                panic!("Data API failed after task termination")
            }
        }
    }
    pub async fn begin(&self, target: &Target<K>, mode: Mode) -> Transaction<K, V> {
        self.ordinary(self.begin_with(target, mode, false).await)
            .await
    }
    pub async fn try_begin(
        &self,
        target: &Target<K>,
        mode: Mode,
    ) -> core::result::Result<Transaction<K, V>, DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        self.begin_with(target, mode, true).await
    }
    async fn begin_with(
        &self,
        target: &Target<K>,
        mode: Mode,
        checked: bool,
    ) -> core::result::Result<Transaction<K, V>, DataError> {
        let ticket = self
            .control
            .next
            .fetch_update(
                core::sync::atomic::Ordering::Relaxed,
                core::sync::atomic::Ordering::Relaxed,
                |n| n.checked_add(1),
            )
            .map_err(|_| DataError::ResourceLimit)?;
        {
            let mut claims = self.control.beginnings.lock().unwrap();
            if claims.len() >= 256 {
                return Err(DataError::ResourceLimit);
            }
            claims.insert(ticket, None);
        }
        let claim = BeginClaim {
            control: self.control.clone(),
            ticket,
        };
        let result = self
            .call_with_ticket(ticket, checked, request::Command::Begin { target, mode })
            .await?;
        match result {
            wire::Result::Begun(id) if id != 0 => {
                self.control.beginnings.lock().unwrap().remove(&ticket);
                drop(claim);
                Ok(Transaction {
                    data: self.clone(),
                    id: Some(id),
                    marker: PhantomData,
                })
            }
            wire::Result::Failure(s) => Err(DataError::status(s)),
            _ => Err(DataError::DataIntegrityError),
        }
    }
}
struct CancelTransaction {
    id: u64,
    control: Arc<Control>,
    armed: bool,
}
impl Drop for CancelTransaction {
    fn drop(&mut self) {
        if self.armed {
            self.control.cancel(true, self.id);
        }
    }
}
pub struct Transaction<K, V> {
    data: Data<K, V>,
    id: Option<u64>,
    marker: PhantomData<Cell<()>>,
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Transaction<K, V> {
    async fn operation(
        &mut self,
        checked: bool,
        op: request::Operation<'_, K, V>,
    ) -> core::result::Result<wire::Result, DataError> {
        let id = self.id.take().ok_or(DataError::InvalidRequest)?;
        let mut guard = CancelTransaction {
            id,
            control: self.data.control.clone(),
            armed: true,
        };
        let result = self
            .data
            .call(
                checked,
                request::Command::ExecuteInTransaction {
                    transaction: id,
                    operation: op,
                },
            )
            .await;
        if result.is_ok() {
            self.id = Some(id);
            guard.armed = false;
        }
        result
    }
    fn invalidate(&mut self) {
        if let Some(id) = self.id.take() {
            self.data.control.cancel(true, id);
        }
    }
    pub async fn commit(mut self) {
        let result = self.finish(true, false).await;
        self.data.ordinary(result).await
    }
    pub async fn try_commit(mut self) -> core::result::Result<(), DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        self.finish(true, true).await
    }
    pub async fn rollback(mut self) {
        let result = self.finish(false, false).await;
        self.data.ordinary(result).await
    }
    pub async fn try_rollback(mut self) -> core::result::Result<(), DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        self.finish(false, true).await
    }
    async fn finish(&mut self, commit: bool, checked: bool) -> core::result::Result<(), DataError> {
        let id = self.id.take().ok_or(DataError::InvalidRequest)?;
        let mut guard = CancelTransaction {
            id,
            control: self.data.control.clone(),
            armed: true,
        };
        let result = self
            .data
            .call(
                checked,
                if commit {
                    request::Command::Commit(id)
                } else {
                    request::Command::Rollback(id)
                },
            )
            .await;
        guard.armed = false;
        core_data::done(result?)
    }
}
impl<K, V> Drop for Transaction<K, V> {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            self.data.control.cancel(true, id);
        }
    }
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Data<K, V> {
    pub async fn get(&self, target: &Target<K>, key: KeyRef<'_, K>) -> Option<V> {
        let result = async {
            let op = request::Operation::Get(key);
            core_data::optional(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_get(
        &self,
        target: &Target<K>,
        key: KeyRef<'_, K>,
    ) -> core::result::Result<Option<V>, DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Get(key);
            core_data::optional(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
    pub async fn get_many(&self, target: &Target<K>, keys: &[KeyRef<'_, K>]) -> Vec<Option<V>> {
        let result = async {
            let op = request::Operation::GetMany(keys);
            core_data::values(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_get_many(
        &self,
        target: &Target<K>,
        keys: &[KeyRef<'_, K>],
    ) -> core::result::Result<Vec<Option<V>>, DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::GetMany(keys);
            core_data::values(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
    pub async fn exists(&self, target: &Target<K>, key: KeyRef<'_, K>) -> bool {
        let result = async {
            let op = request::Operation::Exists(key);
            core_data::exists(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_exists(
        &self,
        target: &Target<K>,
        key: KeyRef<'_, K>,
    ) -> core::result::Result<bool, DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Exists(key);
            core_data::exists(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
    pub async fn scan(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Page<K, Entry<K, V>> {
        let result = async {
            let op = request::Operation::Scan(range, options, false);
            core_data::entries(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_scan(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> core::result::Result<Page<K, Entry<K, V>>, DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Scan(range, options, false);
            core_data::entries(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
    pub async fn scan_keys(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Page<K, EncodedKey<K>> {
        let result = async {
            let op = request::Operation::Scan(range, options, true);
            core_data::keys(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_scan_keys(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> core::result::Result<Page<K, EncodedKey<K>>, DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Scan(range, options, true);
            core_data::keys(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
    pub async fn put(&self, target: &Target<K>, key: KeyRef<'_, K>, value: &V) -> () {
        let result = async {
            let op = request::Operation::Put { key, value };
            core_data::done(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_put(
        &self,
        target: &Target<K>,
        key: KeyRef<'_, K>,
        value: &V,
    ) -> core::result::Result<(), DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Put { key, value };
            core_data::done(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
    pub async fn delete(&self, target: &Target<K>, key: KeyRef<'_, K>) -> () {
        let result = async {
            let op = request::Operation::Delete(key);
            core_data::done(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_delete(
        &self,
        target: &Target<K>,
        key: KeyRef<'_, K>,
    ) -> core::result::Result<(), DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Delete(key);
            core_data::done(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
    pub async fn write_batch(&self, target: &Target<K>, batch: &[Mutation<'_, K, V>]) -> () {
        let result = async {
            let op = request::Operation::WriteBatch(batch);
            core_data::done(self.operation(target, false, op).await?)
        }
        .await;
        self.ordinary(result).await
    }
    pub async fn try_write_batch(
        &self,
        target: &Target<K>,
        batch: &[Mutation<'_, K, V>],
    ) -> core::result::Result<(), DataError> {
        if !self
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::WriteBatch(batch);
            core_data::done(self.operation(target, true, op).await?)
        }
        .await;
        result
    }
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Transaction<K, V> {
    pub async fn get(&mut self, key: KeyRef<'_, K>) -> Option<V> {
        let result = async {
            let op = request::Operation::Get(key);
            core_data::optional(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_get(
        &mut self,
        key: KeyRef<'_, K>,
    ) -> core::result::Result<Option<V>, DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Get(key);
            core_data::optional(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub async fn get_many(&mut self, keys: &[KeyRef<'_, K>]) -> Vec<Option<V>> {
        let result = async {
            let op = request::Operation::GetMany(keys);
            core_data::values(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_get_many(
        &mut self,
        keys: &[KeyRef<'_, K>],
    ) -> core::result::Result<Vec<Option<V>>, DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::GetMany(keys);
            core_data::values(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub async fn exists(&mut self, key: KeyRef<'_, K>) -> bool {
        let result = async {
            let op = request::Operation::Exists(key);
            core_data::exists(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_exists(
        &mut self,
        key: KeyRef<'_, K>,
    ) -> core::result::Result<bool, DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Exists(key);
            core_data::exists(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub async fn scan(
        &mut self,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Page<K, Entry<K, V>> {
        let result = async {
            let op = request::Operation::Scan(range, options, false);
            core_data::entries(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_scan(
        &mut self,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> core::result::Result<Page<K, Entry<K, V>>, DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Scan(range, options, false);
            core_data::entries(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub async fn scan_keys(
        &mut self,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Page<K, EncodedKey<K>> {
        let result = async {
            let op = request::Operation::Scan(range, options, true);
            core_data::keys(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_scan_keys(
        &mut self,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> core::result::Result<Page<K, EncodedKey<K>>, DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Scan(range, options, true);
            core_data::keys(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub async fn put(&mut self, key: KeyRef<'_, K>, value: &V) -> () {
        let result = async {
            let op = request::Operation::Put { key, value };
            core_data::done(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_put(
        &mut self,
        key: KeyRef<'_, K>,
        value: &V,
    ) -> core::result::Result<(), DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Put { key, value };
            core_data::done(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub async fn delete(&mut self, key: KeyRef<'_, K>) -> () {
        let result = async {
            let op = request::Operation::Delete(key);
            core_data::done(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_delete(&mut self, key: KeyRef<'_, K>) -> core::result::Result<(), DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::Delete(key);
            core_data::done(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub async fn write_batch(&mut self, batch: &[Mutation<'_, K, V>]) -> () {
        let result = async {
            let op = request::Operation::WriteBatch(batch);
            core_data::done(self.operation(false, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        self.data.ordinary(result).await
    }
    pub async fn try_write_batch(
        &mut self,
        batch: &[Mutation<'_, K, V>],
    ) -> core::result::Result<(), DataError> {
        if !self
            .data
            .failure
            .call(0)
            .await
            .map_err(|_| DataError::StorageError)?
        {
            return Err(DataError::InvalidRequest);
        }
        let result = async {
            let op = request::Operation::WriteBatch(batch);
            core_data::done(self.operation(true, op).await?)
        }
        .await;
        if result.is_err() {
            self.invalidate();
        }
        result
    }
}
