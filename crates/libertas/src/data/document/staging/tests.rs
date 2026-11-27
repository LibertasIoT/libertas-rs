use super::super::{
    DocumentSweepSchema, DocumentTransfer, DocumentTransferOwner, DocumentTransferProgress,
    DocumentTransferStart,
};
use super::*;
use crate::{LibertasDevice, LibertasPeer, LibertasTransId, ReadResult};
use alloc::collections::BTreeMap;
use core::{
    cell::{Cell, RefCell},
    ffi::c_void,
};

#[derive(libertas_macros::LibertasKeyCodec, libertas_macros::LibertasKeyDecode)]
enum Key {
    Manifest,
    Stage { generation: [u8; 16] },
    Chunk { generation: [u8; 16], index: u32 },
    Attachment,
}

#[derive(Clone)]
enum Value {
    Manifest(DocumentManifest),
    Stage(DocumentStage),
    Chunk(Vec<u8>),
}

type Records = BTreeMap<Vec<u8>, Vec<u8>>;

#[derive(Default)]
struct Storage {
    records: Records,
    transaction: Option<(wire::Target, Records)>,
    response: Vec<u8>,
}

std::thread_local! {
    static STORAGE: RefCell<Storage> = RefCell::new(Storage::default());
    static VALUES: RefCell<Vec<Value>> = const { RefCell::new(Vec::new()) };
    static NEXT_RANDOM: Cell<u64> = const { Cell::new(0) };
    static NOW: Cell<u64> = const { Cell::new(1) };
}

fn value_id(value: Value) -> u32 {
    VALUES.with_borrow_mut(|values| {
        let id = u32::try_from(values.len()).unwrap();
        values.push(value);
        id
    })
}

fn stored_value(id: u32) -> Value {
    VALUES.with_borrow(|values| values[id as usize].clone())
}

struct Schema;

impl DocumentReadSchema<Key, u32> for Schema {
    fn manifest_key(&self) -> Key {
        Key::Manifest
    }

    fn chunk_key(&self, generation: &[u8; 16], index: u32) -> Key {
        Key::Chunk {
            generation: *generation,
            index,
        }
    }

    fn manifest_from_value(&self, id: u32) -> Option<DocumentManifest> {
        match stored_value(id) {
            Value::Manifest(manifest) => Some(manifest),
            _ => None,
        }
    }

    fn chunk_from_value(&self, id: u32) -> Option<Vec<u8>> {
        match stored_value(id) {
            Value::Chunk(bytes) => Some(bytes),
            _ => None,
        }
    }
}

impl DocumentWriteSchema<Key, u32> for Schema {
    fn stage_key(&self, generation: &[u8; 16]) -> Key {
        Key::Stage {
            generation: *generation,
        }
    }

    fn stage_from_value(&self, id: u32) -> Option<DocumentStage> {
        match stored_value(id) {
            Value::Stage(stage) => Some(stage),
            _ => None,
        }
    }

    fn stage_value(&self, stage: DocumentStage) -> u32 {
        value_id(Value::Stage(stage))
    }
    fn chunk_value(&self, bytes: &[u8]) -> u32 {
        value_id(Value::Chunk(bytes.to_vec()))
    }
    fn manifest_value(&self, manifest: DocumentManifest) -> u32 {
        value_id(Value::Manifest(manifest))
    }
}

impl DocumentSweepSchema<Key, u32> for Schema {
    fn stage_prefix(&self) -> &[u8] {
        &[0, 1]
    }

    fn generation_from_stage_key(&self, key: &EncodedKey<Key>) -> Option<[u8; 16]> {
        match key.decode().ok()? {
            Key::Stage { generation } => Some(generation),
            _ => None,
        }
    }
}

fn apply(records: &mut Records, target: &wire::Target, operation: wire::Operation) -> wire::Result {
    match operation {
        wire::Operation::Get(key) => {
            assert!(target.contains(&key));
            wire::Result::Value(records.get(&key).cloned())
        }
        wire::Operation::Exists(key) => {
            assert!(target.contains(&key));
            wire::Result::Exists(records.contains_key(&key))
        }
        wire::Operation::Put { key, value } => {
            assert!(target.contains(&key));
            records.insert(key, value);
            wire::Result::Done
        }
        wire::Operation::Delete(key) => {
            assert!(target.contains(&key));
            records.remove(&key);
            wire::Result::Done
        }
        wire::Operation::Scan(scan) => {
            assert!(scan.keys_only);
            assert_eq!(scan.range.end, wire::Bound::Unbounded);
            assert_eq!(scan.direction, wire::Direction::Ascending);
            let mut keys: Vec<_> = records
                .keys()
                .filter(|key| {
                    target.contains(key)
                        && match &scan.range.start {
                            wire::Bound::Unbounded => true,
                            wire::Bound::Excluded(after) => *key > after,
                            wire::Bound::Included(_) => unreachable!(),
                        }
                })
                .take(scan.limit as usize + 1)
                .cloned()
                .collect();
            let continuation = (keys.len() > scan.limit as usize).then(|| keys.pop().unwrap());
            wire::Result::Keys {
                items: keys,
                continuation,
            }
        }
        _ => unreachable!(),
    }
}

extern "C" fn request(
    _: u16,
    _: LibertasDevice,
    opcode: u8,
    bytes: *const u8,
    len: usize,
) -> ReadResult {
    if opcode == 0xe7 {
        return ReadResult {
            success: true,
            data: core::ptr::null(),
            data_len: 0,
        };
    }
    assert_eq!(opcode, 0xe8);
    let request: wire::Request =
        wire::decode(unsafe { core::slice::from_raw_parts(bytes, len) }).unwrap();
    STORAGE.with_borrow_mut(|storage| {
        let result = match request.command {
            wire::Command::Begin { target, .. } => {
                assert!(storage.transaction.is_none());
                storage.transaction = Some((target, storage.records.clone()));
                wire::Result::Begun(1)
            }
            wire::Command::Execute { target, operation } => {
                apply(&mut storage.records, &target, operation)
            }
            wire::Command::ExecuteInTransaction {
                transaction: 1,
                operation,
            } => {
                let (target, records) = storage.transaction.as_mut().unwrap();
                apply(records, target, operation)
            }
            wire::Command::Commit(1) => {
                storage.records = storage.transaction.take().unwrap().1;
                wire::Result::Done
            }
            _ => unreachable!(),
        };
        storage.response = wire::encode(&wire::Response {
            id: request.id,
            result,
        });
        ReadResult {
            success: true,
            data: storage.response.as_ptr(),
            data_len: storage.response.len(),
        }
    })
}

extern "C" fn send(
    _: u16,
    _: LibertasDevice,
    _: LibertasTransId,
    opcode: u8,
    _: *const u8,
    _: usize,
    _: LibertasPeer,
) {
    if opcode == 0xeb {
        STORAGE.with_borrow_mut(|storage| {
            storage.transaction = None;
        });
    }
}

extern "C" fn random(_: u8) -> u64 {
    NEXT_RANDOM.with(|next| next.replace(next.get() + 1))
}

extern "C" fn utc() -> u64 {
    NOW.get()
}

#[test]
fn only_publishes_the_complete_generation_and_keeps_retries_idempotent() {
    let _guard = crate::tests::TEST_LOCK.lock().unwrap();
    let mut runtime = crate::tests::fake_runtime_api();
    runtime.device_read = request;
    runtime.device_send = send;
    runtime.get_random = random;
    runtime.get_utc_time = utc;
    crate::__libertas_init_package(&raw mut runtime as *mut c_void);
    crate::libertas_impl_init_task(7);
    crate::data::__bind_schema::<Key, u32>();
    let data = Data::current();

    let mut upload = data
        .begin_document_upload(&Schema, 4, "application/pdf".into())
        .unwrap();
    assert_eq!(
        data.append_document_chunk(&Schema, &mut upload, 0, b"ab"),
        Ok(2)
    );
    assert_eq!(
        data.append_document_chunk(&Schema, &mut upload, 0, b"ab"),
        Ok(2)
    );
    assert_eq!(
        data.append_document_chunk(&Schema, &mut upload, 0, b"ac"),
        Err(DocumentWriteError::Conflict)
    );
    assert_eq!(data.read_document(&Schema), Ok(None));
    assert_eq!(
        data.publish_document(&Schema, upload.generation()),
        Err(DocumentWriteError::Conflict)
    );
    assert_eq!(
        data.append_document_chunk(&Schema, &mut upload, 1, b"cd"),
        Ok(4)
    );
    let digest = Sha256::digest(b"abcd").into();
    assert_eq!(data.seal_document_upload(&Schema, &upload, digest), Ok(()));
    assert_eq!(data.seal_document_upload(&Schema, &upload, digest), Ok(()));
    assert_eq!(data.read_document(&Schema), Ok(None));
    let manifest = data
        .publish_document_with_mutations(
            &Schema,
            upload.generation(),
            &[Mutation::Put((&Key::Attachment).into(), &42)],
        )
        .unwrap();
    assert_eq!(
        data.publish_document_with_mutations(
            &Schema,
            upload.generation(),
            &[Mutation::Put((&Key::Attachment).into(), &123)],
        ),
        Ok(manifest.clone())
    );
    assert_eq!(
        data.get(&Target::all(), (&Key::Attachment).into()),
        Some(42)
    );
    let document = data.read_document(&Schema).unwrap().unwrap();
    assert_eq!(document.manifest, manifest);
    assert_eq!(document.bytes, b"abcd");

    let stale = data
        .begin_document_upload(&Schema, 0, "application/pdf".into())
        .unwrap();
    let replacement = data
        .begin_document_upload(&Schema, 0, "application/pdf".into())
        .unwrap();
    let empty_digest = Sha256::digest([]).into();
    data.seal_document_upload(&Schema, &stale, empty_digest)
        .unwrap();
    data.seal_document_upload(&Schema, &replacement, empty_digest)
        .unwrap();
    let new_manifest = data
        .publish_document(&Schema, replacement.generation())
        .unwrap();
    assert_eq!(
        data.publish_document(&Schema, stale.generation()),
        Err(DocumentWriteError::Conflict)
    );
    assert_eq!(
        data.delete_document(&Schema, manifest.generation),
        Err(DocumentWriteError::Conflict)
    );
    data.cancel_document_upload(&Schema, stale.generation())
        .unwrap();
    assert_eq!(
        data.cancel_document_upload(&Schema, replacement.generation()),
        Err(DocumentWriteError::Conflict)
    );
    assert_eq!(
        data.read_document(&Schema).unwrap().unwrap().manifest,
        new_manifest
    );
    assert_eq!(
        data.delete_document(&Schema, new_manifest.generation),
        Ok(true)
    );
    assert_eq!(data.read_document(&Schema), Ok(None));
    assert_eq!(
        data.delete_document(&Schema, new_manifest.generation),
        Ok(false)
    );

    let owner = |peer, editor, field: &str| DocumentTransferOwner {
        peer,
        endpoint: 467,
        editor: [editor; 16],
        field_path: field.into(),
    };
    let mut transfer = DocumentTransfer::begin(
        &data,
        &Schema,
        owner(1, 2, "$.attachment"),
        DocumentTransferStart {
            byte_len: 2,
            media_type: "application/pdf".into(),
            piece_bytes: 2,
        },
    )
    .unwrap();
    assert_eq!(
        transfer.piece(&data, &Schema, &owner(9, 2, "$.attachment"), 0, b"ab"),
        Err(DocumentWriteError::TransferOwnerMismatch)
    );
    assert_eq!(
        transfer.piece(&data, &Schema, &owner(1, 2, "$.attachment"), 0, b"abc"),
        Err(DocumentWriteError::ValueTooLarge)
    );
    assert_eq!(
        transfer.piece(&data, &Schema, &owner(1, 2, "$.attachment"), 0, b"ab"),
        Ok(2)
    );
    assert_eq!(
        transfer.progress(&owner(1, 2, "$.attachment")),
        Ok(DocumentTransferProgress {
            next_index: 1,
            accepted_bytes: 2
        })
    );
    let reference = transfer
        .seal(
            &data,
            &Schema,
            &owner(1, 2, "$.attachment"),
            Sha256::digest(b"ab").into(),
        )
        .unwrap();
    assert_eq!(reference, transfer.handle());
    assert_eq!(
        transfer.publish(&data, &Schema, &owner(1, 3, "$.attachment"), &[]),
        Err(DocumentWriteError::TransferOwnerMismatch)
    );
    assert_eq!(
        transfer.publish(&data, &Schema, &owner(1, 2, "$.other"), &[]),
        Err(DocumentWriteError::TransferOwnerMismatch)
    );
    assert_eq!(data.read_document(&Schema), Ok(None));
    transfer
        .publish(&data, &Schema, &owner(1, 2, "$.attachment"), &[])
        .unwrap();
    assert_eq!(data.read_document(&Schema).unwrap().unwrap().bytes, b"ab");

    crate::__libertas_release_package();
    unsafe {
        crate::RUNTIME_API = core::ptr::null_mut();
    }
}

#[test]
fn checks_the_record_encoding_including_the_avro_length_prefix() {
    let raw = alloc::vec![0u8; wire::MAX_VALUE_BYTES];
    assert_eq!(bounded_value(raw), Err(DocumentWriteError::ValueTooLarge));
}

#[test]
fn startup_sweeps_an_expired_upload_across_bounded_transactions() {
    let _guard = crate::tests::TEST_LOCK.lock().unwrap();
    let mut runtime = crate::tests::fake_runtime_api();
    runtime.device_read = request;
    runtime.device_send = send;
    runtime.get_random = random;
    runtime.get_utc_time = utc;
    crate::__libertas_init_package(&raw mut runtime as *mut c_void);
    crate::libertas_impl_init_task(7);
    crate::data::__bind_schema::<Key, u32>();
    let data = Data::current();

    let mut abandoned = data
        .begin_document_upload(&Schema, 65, "application/pdf".into())
        .unwrap();
    for index in 0..65 {
        data.append_document_chunk(&Schema, &mut abandoned, index, b"x")
            .unwrap();
    }
    let generation = abandoned.generation();
    let stage_key = Schema.stage_key(&generation);
    assert_eq!(data.read_document(&Schema), Ok(None));
    data.sweep_document_page(&Schema, &mut None).unwrap();
    assert!(data.exists(&Target::all(), (&Schema.chunk_key(&generation, 0)).into()));

    crate::libertas_impl_remove_task(7);
    NOW.set(STAGING_LEASE_US + 2);
    crate::libertas_impl_init_task(7);
    crate::data::__bind_schema::<Key, u32>();
    let data = Data::current();
    let manifest = data
        .store_document(&Schema, 4, "application/pdf".into(), [b"ab", b"cd"])
        .unwrap();
    data.start_document_sweeper(Schema, |error| panic!("{error:?}"))
        .unwrap();
    let id = data.get(&Target::all(), (&stage_key).into()).unwrap();
    assert_eq!(
        Schema.stage_from_value(id).unwrap().status,
        DocumentStageStatus::Retired { next_chunk: 64 }
    );
    assert!(!data.exists(&Target::all(), (&Schema.chunk_key(&generation, 0)).into()));
    assert!(data.exists(&Target::all(), (&Schema.chunk_key(&generation, 64)).into()));

    crate::libertas_impl_remove_task(7);
    crate::libertas_impl_init_task(7);
    crate::data::__bind_schema::<Key, u32>();
    let data = Data::current();
    data.start_document_sweeper(Schema, |error| panic!("{error:?}"))
        .unwrap();
    assert!(!data.exists(&Target::all(), (&stage_key).into()));
    assert!(!data.exists(&Target::all(), (&Schema.chunk_key(&generation, 64)).into()));
    let document = data.read_document(&Schema).unwrap().unwrap();
    assert_eq!(document.manifest, manifest);
    assert_eq!(document.bytes, b"abcd");

    crate::__libertas_release_package();
    unsafe {
        crate::RUNTIME_API = core::ptr::null_mut();
    }
}
