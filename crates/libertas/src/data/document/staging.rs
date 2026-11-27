use super::{DocumentManifest, DocumentReadError, DocumentReadSchema};
use crate::data::{Data, EncodedKey, KeyCodec, KeyRef, Mode, Mutation, Target, Transaction, wire};
use crate::{AvroDecode, AvroEncode, ByteBuffer, libertas_get_random, libertas_get_utc_time};
use alloc::{string::String, vec::Vec};
use sha2::{Digest, Sha256};

const STAGING_LEASE_US: u64 = 15 * 60 * 1_000_000;
// Covers the version, request ID, checked flag, and command and mode tags.
const BEGIN_ENVELOPE_BYTES: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentStageStatus {
    Open,
    Sealed([u8; 32]),
    Published,
    Retired { next_chunk: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentStage {
    pub manifest_key: Vec<u8>,
    pub expected_generation: Option<[u8; 16]>,
    pub byte_len: u64,
    pub media_type: String,
    pub accepted_bytes: u64,
    pub chunk_count: u32,
    pub expires_at_utc_us: u64,
    pub status: DocumentStageStatus,
}

/// Maps the SDK's staging state and new records into the App's Data unions.
pub trait DocumentWriteSchema<K, V>: DocumentReadSchema<K, V> {
    fn stage_key(&self, generation: &[u8; 16]) -> K;
    fn stage_from_value(&self, value: V) -> Option<DocumentStage>;
    fn stage_value(&self, stage: DocumentStage) -> V;
    fn chunk_value(&self, bytes: &[u8]) -> V;
    fn manifest_value(&self, manifest: DocumentManifest) -> V;
}

/// An upload's running digest, retained by the App for its active transfer.
pub struct DocumentUpload {
    generation: [u8; 16],
    digest: Sha256,
    chunk_count: u32,
    accepted_bytes: u64,
}

impl DocumentUpload {
    pub fn generation(&self) -> [u8; 16] {
        self.generation
    }

    pub(super) fn progress(&self) -> (u32, u64) {
        (self.chunk_count, self.accepted_bytes)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DocumentWriteError {
    InvalidKey,
    InvalidManifest,
    InvalidStage,
    ClockUnavailable,
    Expired,
    Conflict,
    InvalidChunk,
    ChunkOrder,
    ValueTooLarge,
    LengthMismatch,
    DigestMismatch,
    TransferOwnerMismatch,
}

impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Data<K, V> {
    /// Stores a document from an iterator of bounded pieces, publishing only
    /// after the iterator ends and the received length matches `byte_len`.
    pub fn store_document<B: AsRef<[u8]>>(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        byte_len: u64,
        media_type: String,
        pieces: impl IntoIterator<Item = B>,
    ) -> Result<DocumentManifest, DocumentWriteError> {
        let mut upload = self.begin_document_upload(schema, byte_len, media_type)?;
        let result = (|| {
            for piece in pieces {
                let index = upload.chunk_count;
                self.append_document_chunk(schema, &mut upload, index, piece.as_ref())?;
            }
            let sha256 = upload.digest.clone().finalize().into();
            self.seal_document_upload(schema, &upload, sha256)?;
            self.publish_document(schema, upload.generation)
        })();
        if result.is_err() {
            let _ = self.cancel_document_upload(schema, upload.generation);
        }
        result
    }

    /// Records the current manifest and creates an unpublished upload generation.
    pub fn begin_document_upload(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        byte_len: u64,
        media_type: String,
    ) -> Result<DocumentUpload, DocumentWriteError> {
        let mut generation = [0; 16];
        for part in generation.chunks_exact_mut(8) {
            part.copy_from_slice(&libertas_get_random(8).to_be_bytes());
        }
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&generation))?;
        let target = exact_target([&manifest_key, &stage_key])?;
        let mut txn = self.begin(&target, Mode::ReadWrite);
        if txn.get((&stage_key).into()).is_some() {
            return Err(DocumentWriteError::Conflict);
        }
        let expected_generation = txn
            .get((&manifest_key).into())
            .map(|value| {
                schema
                    .manifest_from_value(value)
                    .ok_or(DocumentWriteError::InvalidManifest)
            })
            .transpose()?
            .map(|manifest| manifest.generation);
        let stage = DocumentStage {
            manifest_key: manifest_key.as_bytes().to_vec(),
            expected_generation,
            byte_len,
            media_type,
            accepted_bytes: 0,
            chunk_count: 0,
            expires_at_utc_us: utc_now()?.saturating_add(STAGING_LEASE_US),
            status: DocumentStageStatus::Open,
        };
        let value = bounded_value(schema.stage_value(stage))?;
        txn.put((&stage_key).into(), &value);
        txn.commit();
        Ok(DocumentUpload {
            generation,
            digest: Sha256::new(),
            chunk_count: 0,
            accepted_bytes: 0,
        })
    }

    /// Restores the running digest and progress of an authorized durable upload.
    /// The calling App must first validate the generation against its durable
    /// owner record. Recovery reads one committed chunk at a time.
    pub fn resume_document_upload(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
    ) -> Result<DocumentUpload, DocumentWriteError> {
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&generation))?;
        let target = exact_target([&stage_key])?;
        let stage = self
            .get(&target, (&stage_key).into())
            .ok_or(DocumentWriteError::InvalidStage)
            .and_then(|value| decode_stage(schema, value, &manifest_key))?;
        require_live(&stage, utc_now()?)?;
        if !matches!(
            stage.status,
            DocumentStageStatus::Open | DocumentStageStatus::Sealed(_)
        ) {
            return Err(DocumentWriteError::Conflict);
        }
        if stage.accepted_bytes > stage.byte_len
            || u64::from(stage.chunk_count) > stage.accepted_bytes
        {
            return Err(DocumentWriteError::InvalidStage);
        }
        let same_stage = |value| {
            decode_stage(schema, value, &manifest_key).is_ok_and(|current| current == stage)
        };
        let mut digest = Sha256::new();
        let mut remaining = stage.accepted_bytes;
        for index in 0..stage.chunk_count {
            let bytes = self
                .read_bound_chunk::<core::convert::Infallible>(
                    schema,
                    &stage_key,
                    &generation,
                    index,
                    &same_stage,
                )
                .map_err(|error| match error {
                    DocumentReadError::InvalidKey => DocumentWriteError::InvalidKey,
                    DocumentReadError::InvalidChunk(_) => DocumentWriteError::InvalidChunk,
                    _ => DocumentWriteError::Conflict,
                })?;
            remaining = remaining
                .checked_sub(bytes.len() as u64)
                .ok_or(DocumentWriteError::LengthMismatch)?;
            digest.update(bytes);
        }
        if remaining != 0 {
            return Err(DocumentWriteError::LengthMismatch);
        }
        if let DocumentStageStatus::Sealed(expected) = stage.status
            && (stage.accepted_bytes != stage.byte_len
                || <[u8; 32]>::from(digest.clone().finalize()) != expected)
        {
            return Err(DocumentWriteError::DigestMismatch);
        }
        if !self
            .get(&target, (&stage_key).into())
            .is_some_and(same_stage)
        {
            return Err(DocumentWriteError::Conflict);
        }
        require_live(&stage, utc_now()?)?;
        Ok(DocumentUpload {
            generation,
            digest,
            chunk_count: stage.chunk_count,
            accepted_bytes: stage.accepted_bytes,
        })
    }

    /// Commits one piece and its progress together. A retry must match the stored bytes.
    pub fn append_document_chunk(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        upload: &mut DocumentUpload,
        index: u32,
        bytes: &[u8],
    ) -> Result<u64, DocumentWriteError> {
        if bytes.is_empty() || bytes.len() > wire::MAX_VALUE_BYTES {
            return Err(DocumentWriteError::InvalidChunk);
        }
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&upload.generation))?;
        let chunk_key = encode_key(&schema.chunk_key(&upload.generation, index))?;
        let target = exact_target([&manifest_key, &stage_key, &chunk_key])?;
        let mut txn = self.begin(&target, Mode::ReadWrite);
        let mut stage = read_stage(&mut txn, schema, &stage_key, &manifest_key)?;
        let now = utc_now()?;
        require_live(&stage, now)?;
        if stage.status != DocumentStageStatus::Open
            || stage.chunk_count != upload.chunk_count
            || stage.accepted_bytes != upload.accepted_bytes
        {
            return Err(DocumentWriteError::Conflict);
        }
        if index < stage.chunk_count {
            let saved = txn
                .get((&chunk_key).into())
                .and_then(|value| schema.chunk_from_value(value));
            return if saved.as_deref() == Some(bytes) {
                txn.commit();
                Ok(stage.accepted_bytes)
            } else {
                Err(DocumentWriteError::Conflict)
            };
        }
        if index != stage.chunk_count {
            return Err(DocumentWriteError::ChunkOrder);
        }
        if txn.exists((&chunk_key).into()) {
            return Err(DocumentWriteError::Conflict);
        }
        stage.chunk_count = index.checked_add(1).ok_or(DocumentWriteError::ChunkOrder)?;
        stage.accepted_bytes = stage
            .accepted_bytes
            .checked_add(bytes.len() as u64)
            .filter(|len| *len <= stage.byte_len)
            .ok_or(DocumentWriteError::LengthMismatch)?;
        stage.expires_at_utc_us = now.saturating_add(STAGING_LEASE_US);
        let accepted_bytes = stage.accepted_bytes;
        let chunk = bounded_value(schema.chunk_value(bytes))?;
        let stage = bounded_value(schema.stage_value(stage))?;
        txn.put((&chunk_key).into(), &chunk);
        txn.put((&stage_key).into(), &stage);
        txn.commit();
        upload.digest.update(bytes);
        upload.chunk_count = index + 1;
        upload.accepted_bytes = accepted_bytes;
        Ok(accepted_bytes)
    }

    /// Verifies the transfer digest and freezes the generation for publication.
    pub fn seal_document_upload(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        upload: &DocumentUpload,
        sha256: [u8; 32],
    ) -> Result<(), DocumentWriteError> {
        if <[u8; 32]>::from(upload.digest.clone().finalize()) != sha256 {
            return Err(DocumentWriteError::DigestMismatch);
        }
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&upload.generation))?;
        let target = exact_target([&stage_key])?;
        let mut txn = self.begin(&target, Mode::ReadWrite);
        let mut stage = read_stage(&mut txn, schema, &stage_key, &manifest_key)?;
        require_live(&stage, utc_now()?)?;
        if stage.accepted_bytes != stage.byte_len
            || stage.accepted_bytes != upload.accepted_bytes
            || stage.chunk_count != upload.chunk_count
        {
            return Err(DocumentWriteError::LengthMismatch);
        }
        match stage.status {
            DocumentStageStatus::Open => {
                stage.status = DocumentStageStatus::Sealed(sha256);
                let value = bounded_value(schema.stage_value(stage))?;
                txn.put((&stage_key).into(), &value);
            }
            DocumentStageStatus::Sealed(previous) if previous == sha256 => {}
            _ => return Err(DocumentWriteError::Conflict),
        }
        txn.commit();
        Ok(())
    }

    /// Streams a live sealed upload for App validation before publication.
    /// A successful read verifies the length and digest. Until it returns,
    /// any bytes seen by the sink are provisional.
    pub fn stream_sealed_document<E>(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
        sink: impl FnMut(&[u8]) -> Result<(), E>,
    ) -> Result<DocumentManifest, DocumentReadError<E>> {
        let manifest_key = super::encode_key(&schema.manifest_key())?;
        let stage_key = super::encode_key(&schema.stage_key(&generation))?;
        let target = exact_target([&stage_key]).map_err(|_| DocumentReadError::InvalidKey)?;
        let decode_live = |value| {
            let stage = decode_stage(schema, value, &manifest_key).ok()?;
            require_live(&stage, utc_now().ok()?).ok()?;
            let DocumentStageStatus::Sealed(sha256) = stage.status else {
                return None;
            };
            if stage.accepted_bytes != stage.byte_len {
                return None;
            }
            Some(DocumentManifest {
                generation,
                byte_len: stage.byte_len,
                sha256,
                media_type: stage.media_type,
                chunk_count: stage.chunk_count,
            })
        };
        let manifest = self
            .get(&target, (&stage_key).into())
            .and_then(&decode_live)
            .ok_or(DocumentReadError::StageUnavailable)?;
        self.stream_document_generation(
            schema,
            &stage_key,
            &manifest,
            |value| decode_live(value).as_ref() == Some(&manifest),
            sink,
        )?;
        if !self
            .get(&target, (&stage_key).into())
            .and_then(decode_live)
            .is_some_and(|current| current == manifest)
        {
            return Err(DocumentReadError::StageUnavailable);
        }
        Ok(manifest)
    }

    /// Publishes a sealed generation and retires the prior generation atomically.
    pub fn publish_document(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
    ) -> Result<DocumentManifest, DocumentWriteError> {
        self.publish_document_with_mutations(schema, generation, &[])
    }

    /// Publishes the document and applies the App's attachment mutations in
    /// the same transaction. Each mutation must target a distinct App key.
    pub fn publish_document_with_mutations(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
        mutations: &[Mutation<'_, K, V>],
    ) -> Result<DocumentManifest, DocumentWriteError> {
        self.publish_document_in_target(schema, generation, mutations, None, |_| true)
    }

    /// Publishes with the App's mutations if the guarded key still satisfies
    /// `check` inside the manifest transaction. A rejected guard leaves the
    /// sealed upload available to cancel or retry. A retry of an already
    /// committed publication returns its manifest without applying mutations.
    pub fn publish_document_if(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
        guard_key: KeyRef<'_, K>,
        check: impl FnOnce(Option<V>) -> bool,
        mutations: &[Mutation<'_, K, V>],
    ) -> Result<DocumentManifest, DocumentWriteError> {
        let guard_key = guard_key
            .bytes()
            .map(EncodedKey::from_bytes)
            .map_err(|_| DocumentWriteError::InvalidKey)?;
        self.publish_document_in_target(schema, generation, mutations, Some(guard_key), check)
    }

    fn publish_document_in_target(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
        mutations: &[Mutation<'_, K, V>],
        guard_key: Option<EncodedKey<K>>,
        check: impl FnOnce(Option<V>) -> bool,
    ) -> Result<DocumentManifest, DocumentWriteError> {
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&generation))?;
        let stage_target = exact_target([&stage_key])?;
        let expected = self
            .get(&stage_target, (&stage_key).into())
            .and_then(|value| schema.stage_from_value(value))
            .filter(|stage| stage.manifest_key == manifest_key.as_bytes())
            .ok_or(DocumentWriteError::InvalidStage)?
            .expected_generation;
        let previous_key = expected
            .map(|previous| encode_key(&schema.stage_key(&previous)))
            .transpose()?;
        if mutations.len() > wire::MAX_ITEMS - 2 - usize::from(previous_key.is_some()) {
            return Err(DocumentWriteError::ValueTooLarge);
        }
        let app_keys = mutations
            .iter()
            .map(|mutation| {
                let key = match mutation {
                    Mutation::Put(key, _) | Mutation::Delete(key) => key,
                };
                key.bytes()
                    .map(EncodedKey::from_bytes)
                    .map_err(|_| DocumentWriteError::InvalidKey)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut keys: Vec<_> = [&manifest_key, &stage_key]
            .into_iter()
            .chain(&previous_key)
            .chain(&app_keys)
            .collect();
        if let Some(key) = &guard_key
            && !keys.contains(&key)
        {
            keys.push(key);
        }
        if keys.len() > wire::MAX_ITEMS {
            return Err(DocumentWriteError::ValueTooLarge);
        }
        keys.sort_unstable();
        let keys: Vec<_> = keys.into_iter().map(Into::into).collect();
        let target = Target::keys(&keys).map_err(|_| DocumentWriteError::InvalidKey)?;
        if encoded_len(&target.raw) > wire::MAX_REQUEST_BYTES - BEGIN_ENVELOPE_BYTES {
            return Err(DocumentWriteError::ValueTooLarge);
        }
        let mut txn = self.begin(&target, Mode::ReadWrite);
        let mut stage = read_stage(&mut txn, schema, &stage_key, &manifest_key)?;
        let current = txn
            .get((&manifest_key).into())
            .map(|value| {
                schema
                    .manifest_from_value(value)
                    .ok_or(DocumentWriteError::InvalidManifest)
            })
            .transpose()?;
        if stage.status == DocumentStageStatus::Published {
            return current
                .filter(|manifest| manifest.generation == generation)
                .ok_or(DocumentWriteError::Conflict);
        }
        let DocumentStageStatus::Sealed(sha256) = stage.status else {
            return Err(DocumentWriteError::Conflict);
        };
        require_live(&stage, utc_now()?)?;
        if stage.expected_generation != expected
            || current.as_ref().map(|manifest| manifest.generation) != expected
        {
            return Err(DocumentWriteError::Conflict);
        }
        if let Some(key) = &guard_key
            && !check(txn.get(key.into()))
        {
            return Err(DocumentWriteError::Conflict);
        }
        let manifest = DocumentManifest {
            generation,
            byte_len: stage.byte_len,
            sha256,
            media_type: stage.media_type.clone(),
            chunk_count: stage.chunk_count,
        };
        stage.status = DocumentStageStatus::Published;
        let manifest_value = schema.manifest_value(manifest.clone());
        let stage_value = schema.stage_value(stage);
        let mut mutation_bytes = mutation_size(&manifest_key, Some(&manifest_value))?
            .saturating_add(mutation_size(&stage_key, Some(&stage_value))?);
        let mut previous_value = None;
        if let Some(previous_key) = &previous_key {
            let previous = read_stage(&mut txn, schema, previous_key, &manifest_key)?;
            if previous.status != DocumentStageStatus::Published {
                return Err(DocumentWriteError::InvalidStage);
            }
            let value = retired_value(schema, previous)?;
            mutation_bytes =
                mutation_bytes.saturating_add(mutation_size(previous_key, Some(&value))?);
            previous_value = Some(value);
        }
        for (key, mutation) in app_keys.iter().zip(mutations) {
            let value = match mutation {
                Mutation::Put(_, value) => Some(*value),
                Mutation::Delete(_) => None,
            };
            mutation_bytes = mutation_bytes.saturating_add(mutation_size(key, value)?);
            if mutation_bytes > wire::MAX_REQUEST_BYTES {
                return Err(DocumentWriteError::ValueTooLarge);
            }
        }
        txn.put((&manifest_key).into(), &manifest_value);
        txn.put((&stage_key).into(), &stage_value);
        if let (Some(key), Some(value)) = (&previous_key, &previous_value) {
            txn.put(key.into(), value);
        }
        for (key, mutation) in app_keys.iter().zip(mutations) {
            match mutation {
                Mutation::Put(_, value) => txn.put(key.into(), *value),
                Mutation::Delete(_) => txn.delete(key.into()),
            }
        }
        txn.commit();
        Ok(manifest)
    }

    /// Extends an active upload's lease while an editor waits to save.
    pub fn renew_document_upload(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
    ) -> Result<(), DocumentWriteError> {
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&generation))?;
        let target = exact_target([&stage_key])?;
        let mut txn = self.begin(&target, Mode::ReadWrite);
        let mut stage = read_stage(&mut txn, schema, &stage_key, &manifest_key)?;
        let now = utc_now()?;
        require_live(&stage, now)?;
        if !matches!(
            stage.status,
            DocumentStageStatus::Open | DocumentStageStatus::Sealed(_)
        ) {
            return Err(DocumentWriteError::Conflict);
        }
        stage.expires_at_utc_us = now.saturating_add(STAGING_LEASE_US);
        let value = bounded_value(schema.stage_value(stage))?;
        txn.put((&stage_key).into(), &value);
        txn.commit();
        Ok(())
    }

    /// Retires an open or sealed upload. A published document requires deletion.
    pub fn cancel_document_upload(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        generation: [u8; 16],
    ) -> Result<(), DocumentWriteError> {
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&generation))?;
        let target = exact_target([&manifest_key, &stage_key])?;
        let mut txn = self.begin(&target, Mode::ReadWrite);
        let Some(value) = txn.get((&stage_key).into()) else {
            return Ok(());
        };
        let stage = decode_stage(schema, value, &manifest_key)?;
        match stage.status {
            DocumentStageStatus::Open | DocumentStageStatus::Sealed(_) => {
                let retired = retired_value(schema, stage)?;
                txn.put((&stage_key).into(), &retired);
            }
            DocumentStageStatus::Retired { .. } => {}
            DocumentStageStatus::Published => return Err(DocumentWriteError::Conflict),
        }
        txn.commit();
        Ok(())
    }

    /// Deletes the expected generation's manifest and retires its chunks.
    /// An absent manifest returns `false`; a replacement returns `Conflict`.
    pub fn delete_document(
        &self,
        schema: &impl DocumentWriteSchema<K, V>,
        expected: [u8; 16],
    ) -> Result<bool, DocumentWriteError> {
        let manifest_key = encode_key(&schema.manifest_key())?;
        let stage_key = encode_key(&schema.stage_key(&expected))?;
        let target = exact_target([&manifest_key, &stage_key])?;
        let mut txn = self.begin(&target, Mode::ReadWrite);
        let Some(value) = txn.get((&manifest_key).into()) else {
            return Ok(false);
        };
        let manifest = schema
            .manifest_from_value(value)
            .ok_or(DocumentWriteError::InvalidManifest)?;
        if manifest.generation != expected {
            return Err(DocumentWriteError::Conflict);
        }
        let stage = read_stage(&mut txn, schema, &stage_key, &manifest_key)?;
        if stage.status != DocumentStageStatus::Published {
            return Err(DocumentWriteError::InvalidStage);
        }
        let retired = retired_value(schema, stage)?;
        txn.put((&stage_key).into(), &retired);
        txn.delete((&manifest_key).into());
        txn.commit();
        Ok(true)
    }
}

fn read_stage<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
    txn: &mut Transaction<K, V>,
    schema: &impl DocumentWriteSchema<K, V>,
    stage_key: &EncodedKey<K>,
    manifest_key: &EncodedKey<K>,
) -> Result<DocumentStage, DocumentWriteError> {
    let value = txn
        .get(stage_key.into())
        .ok_or(DocumentWriteError::InvalidStage)?;
    decode_stage(schema, value, manifest_key)
}

fn decode_stage<K, V>(
    schema: &impl DocumentWriteSchema<K, V>,
    value: V,
    manifest_key: &EncodedKey<K>,
) -> Result<DocumentStage, DocumentWriteError> {
    schema
        .stage_from_value(value)
        .filter(|stage| stage.manifest_key == manifest_key.as_bytes())
        .ok_or(DocumentWriteError::InvalidStage)
}

fn retired_value<K, V: AvroEncode>(
    schema: &impl DocumentWriteSchema<K, V>,
    mut stage: DocumentStage,
) -> Result<V, DocumentWriteError> {
    stage.status = DocumentStageStatus::Retired { next_chunk: 0 };
    bounded_value(schema.stage_value(stage))
}

fn utc_now() -> Result<u64, DocumentWriteError> {
    libertas_get_utc_time().ok_or(DocumentWriteError::ClockUnavailable)
}

fn require_live(stage: &DocumentStage, now: u64) -> Result<(), DocumentWriteError> {
    if now >= stage.expires_at_utc_us {
        return Err(DocumentWriteError::Expired);
    }
    Ok(())
}

fn encode_key<K: KeyCodec>(key: &K) -> Result<EncodedKey<K>, DocumentWriteError> {
    EncodedKey::new(key).map_err(|_| DocumentWriteError::InvalidKey)
}

fn exact_target<K: KeyCodec, const N: usize>(
    mut keys: [&EncodedKey<K>; N],
) -> Result<Target<K>, DocumentWriteError> {
    keys.sort_unstable();
    Target::keys(&keys.map(Into::into)).map_err(|_| DocumentWriteError::InvalidKey)
}

struct EncodedLen(usize);
impl ByteBuffer for EncodedLen {
    fn push(&mut self, _: u8) {
        self.0 = self.0.saturating_add(1);
    }

    fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.0 = self.0.saturating_add(bytes.len());
    }
}

pub(super) fn bounded_value<V: AvroEncode>(value: V) -> Result<V, DocumentWriteError> {
    value_size(&value)?;
    Ok(value)
}

fn value_size<V: AvroEncode>(value: &V) -> Result<usize, DocumentWriteError> {
    let len = encoded_len(value);
    if len > wire::MAX_VALUE_BYTES {
        return Err(DocumentWriteError::ValueTooLarge);
    }
    Ok(len)
}

fn encoded_len<V: AvroEncode>(value: &V) -> usize {
    let mut len = EncodedLen(0);
    value.avro_encode(&mut len);
    len.0
}

fn mutation_size<K, V: AvroEncode>(
    key: &EncodedKey<K>,
    value: Option<&V>,
) -> Result<usize, DocumentWriteError> {
    let value_len = value.map(value_size).transpose()?.unwrap_or(0);
    Ok(key.as_bytes().len() + value_len)
}

#[cfg(test)]
mod tests;
