use super::{
    DocumentManifest, DocumentReadError, DocumentUpload, DocumentWriteError, DocumentWriteSchema,
};
use crate::data::{Data, KeyCodec, KeyRef, Mutation, wire};
use crate::{AvroDecode, AvroEncode, LibertasEndpoint, LibertasPeer};
use alloc::string::String;

/// The App derives this identity from the authenticated Endpoint callback and
/// the live editor field. Peer and Endpoint come from the server callback.
#[derive(Debug, PartialEq, Eq)]
pub struct DocumentTransferOwner {
    pub peer: LibertasPeer,
    pub endpoint: LibertasEndpoint,
    pub editor: [u8; 16],
    pub field_path: String,
}

pub struct DocumentTransferStart {
    pub byte_len: u64,
    pub media_type: String,
    /// Negotiated raw payload bound, accounting for the stored value and
    /// complete wire envelope overhead.
    pub piece_bytes: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DocumentTransferProgress {
    pub next_index: u32,
    pub accepted_bytes: u64,
}

/// The App keeps this handle in its live editor state and uses the normal
/// Endpoint permission and editor lifecycle checks for each request.
pub struct DocumentTransfer {
    owner: DocumentTransferOwner,
    upload: DocumentUpload,
    piece_bytes: usize,
}

impl DocumentTransfer {
    pub fn begin<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: DocumentTransferOwner,
        start: DocumentTransferStart,
    ) -> Result<Self, DocumentWriteError> {
        if start.piece_bytes == 0 || start.piece_bytes >= wire::MAX_VALUE_BYTES {
            return Err(DocumentWriteError::ValueTooLarge);
        }
        let upload = data.begin_document_upload(schema, start.byte_len, start.media_type)?;
        Ok(Self {
            owner,
            upload,
            piece_bytes: start.piece_bytes,
        })
    }

    /// Rebinds a durable upload after the App verifies its persisted owner
    /// and returns the existing handle and committed progress.
    pub fn resume<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: DocumentTransferOwner,
        generation: [u8; 16],
        piece_bytes: usize,
    ) -> Result<Self, DocumentWriteError> {
        if piece_bytes == 0 || piece_bytes >= wire::MAX_VALUE_BYTES {
            return Err(DocumentWriteError::ValueTooLarge);
        }
        let upload = data.resume_document_upload(schema, generation)?;
        Ok(Self {
            owner,
            upload,
            piece_bytes,
        })
    }

    pub fn handle(&self) -> [u8; 16] {
        self.upload.generation()
    }

    pub fn piece_bytes(&self) -> usize {
        self.piece_bytes
    }

    pub fn progress(
        &self,
        owner: &DocumentTransferOwner,
    ) -> Result<DocumentTransferProgress, DocumentWriteError> {
        self.check_owner(owner)?;
        let (next_index, accepted_bytes) = self.upload.progress();
        Ok(DocumentTransferProgress {
            next_index,
            accepted_bytes,
        })
    }

    /// Returns the durable byte count for this transfer's acknowledgement.
    pub fn piece<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        &mut self,
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: &DocumentTransferOwner,
        index: u32,
        bytes: &[u8],
    ) -> Result<u64, DocumentWriteError> {
        self.check_owner(owner)?;
        if bytes.len() > self.piece_bytes {
            return Err(DocumentWriteError::ValueTooLarge);
        }
        data.append_document_chunk(schema, &mut self.upload, index, bytes)
    }

    pub fn seal<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        &self,
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: &DocumentTransferOwner,
        sha256: [u8; 32],
    ) -> Result<[u8; 16], DocumentWriteError> {
        self.check_owner(owner)?;
        data.seal_document_upload(schema, &self.upload, sha256)?;
        Ok(self.handle())
    }

    /// Reads the authenticated editor's sealed bytes for validation before Save.
    pub fn stream_sealed<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static, E>(
        &self,
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: &DocumentTransferOwner,
        sink: impl FnMut(&[u8]) -> Result<(), E>,
    ) -> Result<DocumentManifest, DocumentReadError<E>> {
        self.check_owner(owner)
            .map_err(|_| DocumentReadError::StageUnavailable)?;
        data.stream_sealed_document(schema, self.handle(), sink)
    }

    pub fn renew<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        &self,
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: &DocumentTransferOwner,
    ) -> Result<(), DocumentWriteError> {
        self.check_owner(owner)?;
        data.renew_document_upload(schema, self.handle())
    }

    /// Called after the ordinary Save validation has accepted this field.
    pub fn publish<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        &self,
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: &DocumentTransferOwner,
        mutations: &[Mutation<'_, K, V>],
    ) -> Result<DocumentManifest, DocumentWriteError> {
        self.check_owner(owner)?;
        data.publish_document_with_mutations(schema, self.handle(), mutations)
    }

    /// Checks the authenticated editor and an App-owned record revision in
    /// the same transaction as the manifest and attachment mutations.
    pub fn publish_if<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        &self,
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: &DocumentTransferOwner,
        guard_key: KeyRef<'_, K>,
        check: impl FnOnce(Option<V>) -> bool,
        mutations: &[Mutation<'_, K, V>],
    ) -> Result<DocumentManifest, DocumentWriteError> {
        self.check_owner(owner)?;
        data.publish_document_if(schema, self.handle(), guard_key, check, mutations)
    }

    pub fn cancel<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static>(
        &self,
        data: &Data<K, V>,
        schema: &impl DocumentWriteSchema<K, V>,
        owner: &DocumentTransferOwner,
    ) -> Result<(), DocumentWriteError> {
        self.check_owner(owner)?;
        data.cancel_document_upload(schema, self.handle())
    }

    fn check_owner(&self, owner: &DocumentTransferOwner) -> Result<(), DocumentWriteError> {
        if &self.owner != owner {
            return Err(DocumentWriteError::TransferOwnerMismatch);
        }
        Ok(())
    }
}
