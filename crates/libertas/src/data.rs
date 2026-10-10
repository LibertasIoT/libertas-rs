//! Schema-bound ordered key/value storage. Core calls run on the host thread.
use crate::{AvroDecode, AvroEncode, LibertasTaskId};
use alloc::{rc::Rc, vec::Vec};
use core::{any::TypeId, marker::PhantomData, ops::Bound};
mod key_codec;
pub(crate) mod request;
#[doc(hidden)]
pub use key_codec::__private as __key_codec;
pub use key_codec::{
    KeyCodec, KeyDecode, KeyDecodeError, KeyEncodeError, KeyFieldCodec, KeyFieldDecode, KeyWriter,
};
#[doc(hidden)]
pub mod wire;
pub use wire::{Direction, Mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DataError {
    InvalidRequest = 2,
    StorageError = 3,
    DataIntegrityError = 4,
    ResourceLimit = 5,
}
impl DataError {
    pub(crate) fn status(status: u32) -> Self {
        match status {
            2 => Self::InvalidRequest,
            3 => Self::StorageError,
            5 => Self::ResourceLimit,
            _ => Self::DataIntegrityError,
        }
    }
}

pub struct EncodedKey<K> {
    pub(crate) bytes: Vec<u8>,
    marker: PhantomData<fn() -> K>,
}
impl<K> Clone for EncodedKey<K> {
    fn clone(&self) -> Self {
        Self::from_bytes(self.bytes.clone())
    }
}
impl<K> PartialEq for EncodedKey<K> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}
impl<K> Eq for EncodedKey<K> {}
impl<K> PartialOrd for EncodedKey<K> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<K> Ord for EncodedKey<K> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.bytes.cmp(&other.bytes)
    }
}
impl<K> core::fmt::Debug for EncodedKey<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.bytes.fmt(f)
    }
}
impl<K> EncodedKey<K> {
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            marker: PhantomData,
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
impl<K: KeyCodec> EncodedKey<K> {
    pub fn new(key: &K) -> Result<Self, KeyEncodeError> {
        Ok(Self::from_bytes(key.to_key_bytes()?))
    }
}
impl<K: KeyDecode> EncodedKey<K> {
    pub fn decode(&self) -> Result<K, KeyDecodeError> {
        K::decode_key(&self.bytes)
    }
}
pub enum KeyRef<'a, K> {
    Typed(&'a K),
    Encoded(&'a EncodedKey<K>),
}
impl<'a, K> From<&'a K> for KeyRef<'a, K> {
    fn from(key: &'a K) -> Self {
        Self::Typed(key)
    }
}
impl<'a, K> From<&'a EncodedKey<K>> for KeyRef<'a, K> {
    fn from(key: &'a EncodedKey<K>) -> Self {
        Self::Encoded(key)
    }
}
impl<K: KeyCodec> KeyRef<'_, K> {
    pub(crate) fn bytes(&self) -> Result<Vec<u8>, DataError> {
        match self {
            Self::Typed(k) => k.to_key_bytes().map_err(|_| DataError::InvalidRequest),
            Self::Encoded(k) => Ok(k.bytes.clone()),
        }
    }
}

pub struct Target<K> {
    pub(crate) raw: wire::Target,
    marker: PhantomData<fn() -> K>,
}
impl<K> Clone for Target<K> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            marker: PhantomData,
        }
    }
}
impl<K: KeyCodec> Target<K> {
    /// Exact keys in strictly ascending encoded order; duplicates are rejected.
    pub fn keys(keys: &[KeyRef<'_, K>]) -> Result<Self, DataError> {
        let raw = wire::Target::Keys(keys.iter().map(KeyRef::bytes).collect::<Result<_, _>>()?);
        if !raw.validate() {
            return Err(DataError::InvalidRequest);
        }
        Ok(Self {
            raw,
            marker: PhantomData,
        })
    }
    /// All complete keys starting with these ordered key-codec bytes.
    pub fn prefix(prefix: &[u8]) -> Result<Self, DataError> {
        let raw = wire::Target::Prefix(prefix.to_vec());
        if !raw.validate() {
            return Err(DataError::InvalidRequest);
        }
        Ok(Self {
            raw,
            marker: PhantomData,
        })
    }
    pub fn all() -> Self {
        Self {
            raw: wire::Target::Prefix(Vec::new()),
            marker: PhantomData,
        }
    }
}
pub struct KeyRange<K> {
    pub(crate) raw: wire::Range,
    marker: PhantomData<fn() -> K>,
}
impl<K> Clone for KeyRange<K> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            marker: PhantomData,
        }
    }
}
impl<K> KeyRange<K> {
    pub fn new(start: Bound<EncodedKey<K>>, end: Bound<EncodedKey<K>>) -> Self {
        fn b<K>(v: Bound<EncodedKey<K>>) -> wire::Bound {
            match v {
                Bound::Unbounded => wire::Bound::Unbounded,
                Bound::Included(k) => wire::Bound::Included(k.bytes),
                Bound::Excluded(k) => wire::Bound::Excluded(k.bytes),
            }
        }
        Self {
            raw: wire::Range {
                start: b(start),
                end: b(end),
            },
            marker: PhantomData,
        }
    }
    pub fn all() -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
}
pub struct Continuation<K> {
    pub(crate) bytes: Vec<u8>,
    marker: PhantomData<fn() -> K>,
}
impl<K> Clone for Continuation<K> {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            marker: PhantomData,
        }
    }
}
pub struct ScanOptions<K> {
    pub direction: Direction,
    pub limit: u32,
    pub continuation: Option<Continuation<K>>,
}
impl<K> ScanOptions<K> {
    pub fn new(limit: u32) -> Self {
        Self {
            direction: Direction::Ascending,
            limit,
            continuation: None,
        }
    }
}
pub struct Entry<K, V> {
    pub key: EncodedKey<K>,
    pub value: V,
}
pub struct Page<K, T> {
    pub items: Vec<T>,
    pub next: Option<Continuation<K>>,
}
pub enum Mutation<'a, K, V> {
    Put(KeyRef<'a, K>, &'a V),
    Delete(KeyRef<'a, K>),
}
pub(crate) fn optional<V: AvroDecode>(r: wire::Result) -> Result<Option<V>, DataError> {
    if let wire::Result::Value(value) = r {
        value
            .map(|v| wire::decode(&v).map_err(|_| DataError::DataIntegrityError))
            .transpose()
    } else {
        Err(DataError::DataIntegrityError)
    }
}
pub(crate) fn values<V: AvroDecode>(r: wire::Result) -> Result<Vec<Option<V>>, DataError> {
    if let wire::Result::Values(values) = r {
        values
            .into_iter()
            .map(|v| optional(wire::Result::Value(v)))
            .collect()
    } else {
        Err(DataError::DataIntegrityError)
    }
}
pub(crate) fn exists(r: wire::Result) -> Result<bool, DataError> {
    if let wire::Result::Exists(v) = r {
        Ok(v)
    } else {
        Err(DataError::DataIntegrityError)
    }
}
pub(crate) fn done(r: wire::Result) -> Result<(), DataError> {
    if r == wire::Result::Done {
        Ok(())
    } else {
        Err(DataError::DataIntegrityError)
    }
}
pub(crate) fn entries<K, V: AvroDecode>(
    r: wire::Result,
) -> Result<Page<K, Entry<K, V>>, DataError> {
    if let wire::Result::Entries {
        items,
        continuation,
    } = r
    {
        Ok(Page {
            items: items
                .into_iter()
                .map(|e| {
                    Ok(Entry {
                        key: EncodedKey::from_bytes(e.key),
                        value: wire::decode(&e.value).map_err(|_| DataError::DataIntegrityError)?,
                    })
                })
                .collect::<Result<_, _>>()?,
            next: continuation.map(|bytes| Continuation {
                bytes,
                marker: PhantomData,
            }),
        })
    } else {
        Err(DataError::DataIntegrityError)
    }
}
pub(crate) fn keys<K>(r: wire::Result) -> Result<Page<K, EncodedKey<K>>, DataError> {
    if let wire::Result::Keys {
        items,
        continuation,
    } = r
    {
        Ok(Page {
            items: items.into_iter().map(EncodedKey::from_bytes).collect(),
            next: continuation.map(|bytes| Continuation {
                bytes,
                marker: PhantomData,
            }),
        })
    } else {
        Err(DataError::DataIntegrityError)
    }
}

static mut NEXT_ID: u64 = 1;
pub(crate) fn next_id() -> Result<u64, DataError> {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID = NEXT_ID.checked_add(1).ok_or(DataError::ResourceLimit)?;
        Ok(id)
    }
}
#[doc(hidden)]
pub fn __bind_schema<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>() {
    if unsafe { (&*(&raw const crate::ENV)).is_none() } {
        return;
    }
    let binding = (
        TypeId::of::<K>(),
        TypeId::of::<V>(),
        next_id().unwrap_or_else(fatal),
    );
    let task = crate::libertas_get_task_id();
    unsafe {
        let env = &mut *(&raw mut crate::ENV);
        env.as_mut()
            .and_then(|e| e.contexts.get_mut(&task))
            .unwrap_or_else(|| panic!("Data schema binding requires a task"))
            .data_binding = Some(binding);
    }
}
pub struct Data<K, V> {
    pub(crate) task: LibertasTaskId,
    pub(crate) binding: u64,
    marker: PhantomData<(fn() -> (K, V), Rc<()>)>,
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Data<K, V> {
    pub fn current() -> Self {
        Self::try_current().unwrap_or_else(fatal)
    }
    pub fn try_current() -> Result<Self, DataError> {
        if unsafe { crate::RUNTIME_API.is_null() || (&*(&raw const crate::ENV)).is_none() } {
            return Err(DataError::InvalidRequest);
        }
        let task = crate::libertas_get_task_id();
        let binding = unsafe {
            let env = &*(&raw const crate::ENV);
            env.as_ref()
                .and_then(|e| e.contexts.get(&task))
                .and_then(|c| c.data_binding)
        };
        match binding {
            Some((k, v, binding)) if k == TypeId::of::<K>() && v == TypeId::of::<V>() => Ok(Self {
                task,
                binding,
                marker: PhantomData,
            }),
            _ => Err(DataError::InvalidRequest),
        }
    }
    pub(crate) fn duplicate(&self) -> Self {
        Self {
            task: self.task,
            binding: self.binding,
            marker: PhantomData,
        }
    }
    pub(crate) fn valid(&self) -> bool {
        Self::try_current().is_ok_and(|d| d.task == self.task && d.binding == self.binding)
    }
    #[cfg(feature = "tokio")]
    pub(crate) fn request(
        &self,
        checked: bool,
        command: wire::Command,
    ) -> Result<wire::Request, DataError> {
        if !self.valid() {
            return Err(DataError::InvalidRequest);
        }
        Ok(wire::Request {
            version: wire::VERSION,
            id: next_id()?,
            checked,
            command,
        })
    }
    fn call(
        &self,
        checked: bool,
        command: request::Command<'_, K, V>,
    ) -> Result<wire::Result, DataError> {
        if !self.valid() {
            return Err(DataError::InvalidRequest);
        }
        let id = next_id()?;
        let bytes = request::frame(id, checked, command)?;
        host_call_bytes(0xe8, id, bytes.as_slice())?.ok_or(DataError::DataIntegrityError)
    }
    fn operation(
        &self,
        target: &Target<K>,
        checked: bool,
        operation: request::Operation<'_, K, V>,
    ) -> Result<wire::Result, DataError> {
        self.call(checked, request::Command::Execute { target, operation })
    }
    pub fn begin(&self, target: &Target<K>, mode: Mode) -> Transaction<K, V> {
        self.begin_with(target, mode, false).unwrap_or_else(fatal)
    }
    pub fn try_begin(
        &self,
        target: &Target<K>,
        mode: Mode,
    ) -> Result<Transaction<K, V>, DataError> {
        authorize_checked()?;
        self.begin_with(target, mode, true)
    }
    fn begin_with(
        &self,
        target: &Target<K>,
        mode: Mode,
        checked: bool,
    ) -> Result<Transaction<K, V>, DataError> {
        match self.call(checked, request::Command::Begin { target, mode })? {
            wire::Result::Begun(id) if id != 0 => Ok(Transaction {
                data: self.duplicate(),
                id: Some(id),
            }),
            _ => Err(DataError::DataIntegrityError),
        }
    }
}
pub struct Transaction<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> {
    data: Data<K, V>,
    id: Option<u64>,
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Transaction<K, V> {
    fn operation(
        &mut self,
        checked: bool,
        operation: request::Operation<'_, K, V>,
    ) -> Result<wire::Result, DataError> {
        let id = self.id.ok_or(DataError::InvalidRequest)?;
        let result = self.data.call(
            checked,
            request::Command::ExecuteInTransaction {
                transaction: id,
                operation,
            },
        );
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    fn invalidate(&mut self) {
        if let Some(id) = self.id.take() {
            if self.data.valid() {
                cancel(true, id);
            }
        }
    }
    pub fn commit(mut self) {
        let result = self.finish(true, false);
        result.unwrap_or_else(fatal)
    }
    pub fn try_commit(mut self) -> Result<(), DataError> {
        authorize_checked()?;
        self.finish(true, true)
    }
    pub fn rollback(mut self) {
        let result = self.finish(false, false);
        result.unwrap_or_else(fatal)
    }
    pub fn try_rollback(mut self) -> Result<(), DataError> {
        authorize_checked()?;
        self.finish(false, true)
    }
    fn finish(&mut self, commit: bool, checked: bool) -> Result<(), DataError> {
        let id = self.id.take().ok_or(DataError::InvalidRequest)?;
        {
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
                .and_then(done);
            if result.is_err() && self.data.valid() {
                cancel(true, id);
            }
            result
        }
    }
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Drop for Transaction<K, V> {
    fn drop(&mut self) {
        self.invalidate();
    }
}
#[cfg(feature = "tokio")]
pub(crate) fn host_call(
    op: u8,
    request: &wire::Request,
) -> Result<Option<wire::Result>, DataError> {
    let mut bytes = crate::InlineByteBuffer::new();
    request.avro_encode(&mut bytes);
    host_call_bytes(op, request.id, bytes.as_slice())
}
fn host_call_bytes(op: u8, id: u64, bytes: &[u8]) -> Result<Option<wire::Result>, DataError> {
    if bytes.len() > wire::MAX_REQUEST_BYTES {
        return Err(DataError::ResourceLimit);
    }
    let result = crate::__libertas_device_read_raw(
        crate::PROTOCOL_LIBERTAS,
        crate::DEVICE_SYSTEM,
        op,
        bytes.as_ptr(),
        bytes.len(),
    );
    if !result.success {
        if op == 0xe9 {
            return Ok(None);
        }
        return Err(DataError::StorageError);
    }
    reply(id, result).map(Some)
}
pub(crate) fn reply(id: u64, result: crate::ReadResult) -> Result<wire::Result, DataError> {
    if result.data.is_null() || result.data_len > wire::MAX_REQUEST_BYTES {
        return Err(DataError::DataIntegrityError);
    }
    let bytes = unsafe { core::slice::from_raw_parts(result.data, result.data_len) };
    let reply: wire::Response = wire::decode(bytes).map_err(|_| DataError::DataIntegrityError)?;
    if reply.id != id {
        return Err(DataError::DataIntegrityError);
    }
    match reply.result {
        wire::Result::Failure(status) => Err(DataError::status(status)),
        v => Ok(v),
    }
}
pub(crate) fn cancel(transaction: bool, id: u64) {
    let mut bytes = crate::InlineByteBuffer::new();
    (transaction, id).avro_encode(&mut bytes);
    crate::__libertas_device_send_raw(
        crate::PROTOCOL_LIBERTAS,
        crate::DEVICE_SYSTEM,
        0xeb,
        0,
        0,
        bytes.as_slice().as_ptr(),
        bytes.len(),
    );
}
pub(crate) fn fatal<T>(_: DataError) -> T {
    crate::__libertas_device_send_raw(
        crate::PROTOCOL_LIBERTAS,
        crate::DEVICE_SYSTEM,
        0xf7,
        0,
        0,
        core::ptr::null(),
        0,
    );
    panic!("Host returned after fatal Data API failure")
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Data<K, V> {
    pub fn get(&self, target: &Target<K>, key: KeyRef<'_, K>) -> Option<V> {
        let result = (|| {
            let op = request::Operation::Get(key);
            optional(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_get(&self, target: &Target<K>, key: KeyRef<'_, K>) -> Result<Option<V>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Get(key);
            optional(self.operation(target, true, op)?)
        })();
        result
    }
    pub fn get_many(&self, target: &Target<K>, keys: &[KeyRef<'_, K>]) -> Vec<Option<V>> {
        let result = (|| {
            let op = request::Operation::GetMany(keys);
            values(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_get_many(
        &self,
        target: &Target<K>,
        keys: &[KeyRef<'_, K>],
    ) -> Result<Vec<Option<V>>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::GetMany(keys);
            values(self.operation(target, true, op)?)
        })();
        result
    }
    pub fn exists(&self, target: &Target<K>, key: KeyRef<'_, K>) -> bool {
        let result = (|| {
            let op = request::Operation::Exists(key);
            exists(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_exists(&self, target: &Target<K>, key: KeyRef<'_, K>) -> Result<bool, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Exists(key);
            exists(self.operation(target, true, op)?)
        })();
        result
    }
    pub fn scan(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Page<K, Entry<K, V>> {
        let result = (|| {
            let op = request::Operation::Scan(range, options, false);
            entries(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_scan(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Result<Page<K, Entry<K, V>>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Scan(range, options, false);
            entries(self.operation(target, true, op)?)
        })();
        result
    }
    pub fn scan_keys(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Page<K, EncodedKey<K>> {
        let result = (|| {
            let op = request::Operation::Scan(range, options, true);
            keys(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_scan_keys(
        &self,
        target: &Target<K>,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Result<Page<K, EncodedKey<K>>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Scan(range, options, true);
            keys(self.operation(target, true, op)?)
        })();
        result
    }
    pub fn put(&self, target: &Target<K>, key: KeyRef<'_, K>, value: &V) -> () {
        let result = (|| {
            let op = request::Operation::Put { key, value };
            done(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_put(
        &self,
        target: &Target<K>,
        key: KeyRef<'_, K>,
        value: &V,
    ) -> Result<(), DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Put { key, value };
            done(self.operation(target, true, op)?)
        })();
        result
    }
    pub fn delete(&self, target: &Target<K>, key: KeyRef<'_, K>) -> () {
        let result = (|| {
            let op = request::Operation::Delete(key);
            done(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_delete(&self, target: &Target<K>, key: KeyRef<'_, K>) -> Result<(), DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Delete(key);
            done(self.operation(target, true, op)?)
        })();
        result
    }
    pub fn write_batch(&self, target: &Target<K>, batch: &[Mutation<'_, K, V>]) -> () {
        let result = (|| {
            let op = request::Operation::WriteBatch(batch);
            done(self.operation(target, false, op)?)
        })();
        result.unwrap_or_else(fatal)
    }
    pub fn try_write_batch(
        &self,
        target: &Target<K>,
        batch: &[Mutation<'_, K, V>],
    ) -> Result<(), DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::WriteBatch(batch);
            done(self.operation(target, true, op)?)
        })();
        result
    }
}
impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Transaction<K, V> {
    pub fn get(&mut self, key: KeyRef<'_, K>) -> Option<V> {
        let result = (|| {
            let op = request::Operation::Get(key);
            optional(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_get(&mut self, key: KeyRef<'_, K>) -> Result<Option<V>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Get(key);
            optional(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub fn get_many(&mut self, keys: &[KeyRef<'_, K>]) -> Vec<Option<V>> {
        let result = (|| {
            let op = request::Operation::GetMany(keys);
            values(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_get_many(&mut self, keys: &[KeyRef<'_, K>]) -> Result<Vec<Option<V>>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::GetMany(keys);
            values(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub fn exists(&mut self, key: KeyRef<'_, K>) -> bool {
        let result = (|| {
            let op = request::Operation::Exists(key);
            exists(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_exists(&mut self, key: KeyRef<'_, K>) -> Result<bool, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Exists(key);
            exists(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub fn scan(&mut self, range: &KeyRange<K>, options: &ScanOptions<K>) -> Page<K, Entry<K, V>> {
        let result = (|| {
            let op = request::Operation::Scan(range, options, false);
            entries(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_scan(
        &mut self,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Result<Page<K, Entry<K, V>>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Scan(range, options, false);
            entries(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub fn scan_keys(
        &mut self,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Page<K, EncodedKey<K>> {
        let result = (|| {
            let op = request::Operation::Scan(range, options, true);
            keys(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_scan_keys(
        &mut self,
        range: &KeyRange<K>,
        options: &ScanOptions<K>,
    ) -> Result<Page<K, EncodedKey<K>>, DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Scan(range, options, true);
            keys(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub fn put(&mut self, key: KeyRef<'_, K>, value: &V) -> () {
        let result = (|| {
            let op = request::Operation::Put { key, value };
            done(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_put(&mut self, key: KeyRef<'_, K>, value: &V) -> Result<(), DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Put { key, value };
            done(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub fn delete(&mut self, key: KeyRef<'_, K>) -> () {
        let result = (|| {
            let op = request::Operation::Delete(key);
            done(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_delete(&mut self, key: KeyRef<'_, K>) -> Result<(), DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::Delete(key);
            done(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
    pub fn write_batch(&mut self, batch: &[Mutation<'_, K, V>]) -> () {
        let result = (|| {
            let op = request::Operation::WriteBatch(batch);
            done(self.operation(false, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result.unwrap_or_else(fatal)
    }
    pub fn try_write_batch(&mut self, batch: &[Mutation<'_, K, V>]) -> Result<(), DataError> {
        authorize_checked()?;
        let result = (|| {
            let op = request::Operation::WriteBatch(batch);
            done(self.operation(true, op)?)
        })();
        if result.is_err() {
            self.invalidate();
        }
        result
    }
}

pub(crate) fn authorize_checked() -> Result<(), DataError> {
    if unsafe { crate::RUNTIME_API.is_null() } {
        return Err(DataError::InvalidRequest);
    }
    let raw = crate::__libertas_device_read_raw(
        crate::PROTOCOL_LIBERTAS,
        crate::DEVICE_SYSTEM,
        0xe7,
        [].as_ptr(),
        0,
    );
    if raw.success {
        Ok(())
    } else {
        Err(DataError::InvalidRequest)
    }
}
