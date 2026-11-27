//! Staging and verified reads in a task's schema-bound Data context.

use super::{Data, EncodedKey, KeyCodec, Mode, Target};
use crate::{AvroDecode, AvroEncode};
use alloc::{collections::TryReserveError, string::String, vec::Vec};
use sha2::{Digest, Sha256};

mod staging;
mod sweep;
mod transfer;
pub use staging::{
    DocumentStage, DocumentStageStatus, DocumentUpload, DocumentWriteError, DocumentWriteSchema,
};
pub use sweep::DocumentSweepSchema;
pub use transfer::{
    DocumentTransfer, DocumentTransferOwner, DocumentTransferProgress, DocumentTransferStart,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentManifest {
    pub generation: [u8; 16],
    pub byte_len: u64,
    pub sha256: [u8; 32],
    pub media_type: String,
    pub chunk_count: u32,
}

/// Maps a document and its generations to the App's existing key and value unions.
pub trait DocumentReadSchema<K, V> {
    fn manifest_key(&self) -> K;
    fn chunk_key(&self, generation: &[u8; 16], index: u32) -> K;
    fn manifest_from_value(&self, value: V) -> Option<DocumentManifest>;
    fn chunk_from_value(&self, value: V) -> Option<Vec<u8>>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum DocumentReadError<E> {
    InvalidKey,
    InvalidManifest,
    ManifestChanged,
    StageUnavailable,
    InvalidChunk(u32),
    LengthMismatch,
    DigestMismatch,
    Sink(E),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub manifest: DocumentManifest,
    pub bytes: Vec<u8>,
}

impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Data<K, V> {
    /// Collects a published document and returns its bytes after digest verification.
    pub fn read_document(
        &self,
        schema: &impl DocumentReadSchema<K, V>,
    ) -> Result<Option<Document>, DocumentReadError<TryReserveError>> {
        let mut bytes = Vec::new();
        let manifest = self.stream_document(schema, |chunk| {
            bytes.try_reserve(chunk.len())?;
            bytes.extend_from_slice(chunk);
            Ok(())
        })?;
        Ok(manifest.map(|manifest| Document { manifest, bytes }))
    }

    /// Reads one chunk at a time. The sink receives provisional bytes and should
    /// publish its output only after this call returns `Ok(Some(manifest))`.
    /// `Ok(None)` means the initial manifest lookup found no document.
    pub fn stream_document<E>(
        &self,
        schema: &impl DocumentReadSchema<K, V>,
        sink: impl FnMut(&[u8]) -> Result<(), E>,
    ) -> Result<Option<DocumentManifest>, DocumentReadError<E>> {
        let manifest_key = encode_key(&schema.manifest_key())?;
        let manifest_target =
            Target::keys(&[(&manifest_key).into()]).map_err(|_| DocumentReadError::InvalidKey)?;
        let Some(value) = self.get(&manifest_target, (&manifest_key).into()) else {
            return Ok(None);
        };
        let manifest = schema
            .manifest_from_value(value)
            .ok_or(DocumentReadError::InvalidManifest)?;
        self.stream_document_generation(
            schema,
            &manifest_key,
            &manifest,
            |value| schema.manifest_from_value(value).as_ref() == Some(&manifest),
            sink,
        )?;
        Ok(Some(manifest))
    }

    /// Reads one committed piece while verifying that the manifest is still the
    /// exact snapshot supplied by the caller. Use this for transports that
    /// wait for an acknowledgment before obtaining the next indexed piece.
    /// Each returned piece is provisional until the receiver verifies the
    /// complete length and digest against the captured manifest.
    pub fn read_document_piece(
        &self,
        schema: &impl DocumentReadSchema<K, V>,
        manifest: &DocumentManifest,
        index: u32,
    ) -> Result<Vec<u8>, DocumentReadError<core::convert::Infallible>> {
        if index >= manifest.chunk_count {
            return Err(DocumentReadError::InvalidChunk(index));
        }
        let binding_key = encode_key(&schema.manifest_key())?;
        self.read_bound_chunk(schema, &binding_key, &manifest.generation, index, |value| {
            schema.manifest_from_value(value).as_ref() == Some(manifest)
        })
    }

    fn stream_document_generation<E>(
        &self,
        schema: &impl DocumentReadSchema<K, V>,
        binding_key: &EncodedKey<K>,
        manifest: &DocumentManifest,
        matches_generation: impl Fn(V) -> bool,
        mut sink: impl FnMut(&[u8]) -> Result<(), E>,
    ) -> Result<(), DocumentReadError<E>> {
        if (manifest.byte_len == 0) != (manifest.chunk_count == 0)
            || u64::from(manifest.chunk_count) > manifest.byte_len
        {
            return Err(DocumentReadError::InvalidManifest);
        }

        let mut digest = Sha256::new();
        let mut remaining = manifest.byte_len;
        for index in 0..manifest.chunk_count {
            let bytes = self.read_bound_chunk(
                schema,
                binding_key,
                &manifest.generation,
                index,
                &matches_generation,
            )?;
            remaining = remaining
                .checked_sub(bytes.len() as u64)
                .ok_or(DocumentReadError::LengthMismatch)?;
            digest.update(&bytes);
            sink(&bytes).map_err(DocumentReadError::Sink)?;
        }
        if remaining != 0 {
            return Err(DocumentReadError::LengthMismatch);
        }
        if <[u8; 32]>::from(digest.finalize()) != manifest.sha256 {
            return Err(DocumentReadError::DigestMismatch);
        }
        Ok(())
    }

    fn read_bound_chunk<E>(
        &self,
        schema: &impl DocumentReadSchema<K, V>,
        binding_key: &EncodedKey<K>,
        generation: &[u8; 16],
        index: u32,
        matches_generation: impl FnOnce(V) -> bool,
    ) -> Result<Vec<u8>, DocumentReadError<E>> {
        let chunk_key = encode_key(&schema.chunk_key(generation, index))?;
        let mut keys = [binding_key, &chunk_key];
        keys.sort_unstable();
        let target =
            Target::keys(&keys.map(Into::into)).map_err(|_| DocumentReadError::InvalidKey)?;
        let mut txn = self.begin(&target, Mode::ReadOnly);
        if !txn.get(binding_key.into()).is_some_and(matches_generation) {
            return Err(DocumentReadError::ManifestChanged);
        }
        let bytes = txn
            .get((&chunk_key).into())
            .and_then(|value| schema.chunk_from_value(value));
        txn.commit();
        bytes
            .filter(|bytes| !bytes.is_empty())
            .ok_or(DocumentReadError::InvalidChunk(index))
    }
}

fn encode_key<K: KeyCodec, E>(key: &K) -> Result<EncodedKey<K>, DocumentReadError<E>> {
    EncodedKey::new(key).map_err(|_| DocumentReadError::InvalidKey)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        LibertasDevice, ReadResult,
        data::{KeyDecode, wire},
    };
    use alloc::vec;
    use core::{
        cell::{Cell, RefCell},
        convert::Infallible,
        ffi::c_void,
    };

    #[derive(libertas_macros::LibertasKeyCodec, libertas_macros::LibertasKeyDecode)]
    enum Key {
        Manifest,
        Chunk { index: u32 },
    }

    struct Schema;
    impl DocumentReadSchema<Key, Vec<u8>> for Schema {
        fn manifest_key(&self) -> Key {
            Key::Manifest
        }

        fn chunk_key(&self, generation: &[u8; 16], index: u32) -> Key {
            assert_eq!(generation, &[7; 16]);
            Key::Chunk { index }
        }

        fn manifest_from_value(&self, value: Vec<u8>) -> Option<DocumentManifest> {
            match value.as_slice() {
                [0] => Some(DocumentManifest {
                    generation: [7; 16],
                    byte_len: 4,
                    sha256: Sha256::digest(b"abcd").into(),
                    media_type: String::from("application/pdf"),
                    chunk_count: 2,
                }),
                _ => None,
            }
        }

        fn chunk_from_value(&self, value: Vec<u8>) -> Option<Vec<u8>> {
            Some(value)
        }
    }

    #[derive(Clone, Copy)]
    enum Scenario {
        Complete,
        Unpublished,
        Corrupt,
    }

    std::thread_local! {
        static REPLY: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        static SCENARIO: Cell<Scenario> = const { Cell::new(Scenario::Complete) };
    }

    extern "C" fn read_data(
        _: u16,
        _: LibertasDevice,
        operation: u8,
        bytes: *const u8,
        len: usize,
    ) -> ReadResult {
        if operation == 0xe7 {
            return ReadResult {
                success: true,
                data: core::ptr::null(),
                data_len: 0,
            };
        }
        assert_eq!(operation, 0xe8);
        let request: wire::Request =
            wire::decode(unsafe { core::slice::from_raw_parts(bytes, len) }).unwrap();
        assert!(!request.checked);
        let result = match request.command {
            wire::Command::Begin {
                target: wire::Target::Keys(keys),
                mode: Mode::ReadOnly,
            } => {
                assert_eq!(keys.len(), 2);
                assert!(keys[0] < keys[1]);
                wire::Result::Begun(1)
            }
            wire::Command::Execute {
                operation: wire::Operation::Get(key),
                ..
            }
            | wire::Command::ExecuteInTransaction {
                operation: wire::Operation::Get(key),
                ..
            } => {
                let value = match (Key::decode_key(&key).unwrap(), SCENARIO.get()) {
                    (Key::Manifest, Scenario::Unpublished) => None,
                    (Key::Manifest, _) => Some(vec![0u8]),
                    (Key::Chunk { index: 0 }, _) => Some(b"ab".to_vec()),
                    (Key::Chunk { index: 1 }, Scenario::Corrupt) => Some(b"ce".to_vec()),
                    (Key::Chunk { index: 1 }, _) => Some(b"cd".to_vec()),
                    _ => unreachable!(),
                };
                wire::Result::Value(value.map(|value| wire::encode(&value)))
            }
            wire::Command::Commit(1) => wire::Result::Done,
            _ => unreachable!(),
        };
        REPLY.with(|reply| {
            let mut reply = reply.borrow_mut();
            *reply = wire::encode(&wire::Response {
                id: request.id,
                result,
            });
            ReadResult {
                success: true,
                data: reply.as_ptr(),
                data_len: reply.len(),
            }
        })
    }

    #[test]
    fn reads_published_documents_and_refuses_corrupt_bytes() {
        let _guard = crate::tests::TEST_LOCK.lock().unwrap();
        let mut runtime = crate::tests::fake_runtime_api();
        runtime.device_read = read_data;
        crate::__libertas_init_package(&raw mut runtime as *mut c_void);
        crate::libertas_impl_init_task(7);
        crate::data::__bind_schema::<Key, Vec<u8>>();
        let data = Data::current();

        SCENARIO.set(Scenario::Unpublished);
        assert_eq!(data.read_document(&Schema), Ok(None));
        SCENARIO.set(Scenario::Complete);
        assert_eq!(data.read_document(&Schema).unwrap().unwrap().bytes, b"abcd");
        SCENARIO.set(Scenario::Corrupt);
        assert_eq!(
            data.read_document(&Schema),
            Err(DocumentReadError::DigestMismatch)
        );
        assert_eq!(
            data.stream_document(&Schema, |_| Ok::<(), Infallible>(())),
            Err(DocumentReadError::DigestMismatch)
        );

        crate::__libertas_release_package();
        unsafe {
            crate::RUNTIME_API = core::ptr::null_mut();
        }
    }
}
