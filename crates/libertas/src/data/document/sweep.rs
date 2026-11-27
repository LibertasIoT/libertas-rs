use super::{DocumentStageStatus, DocumentWriteError, DocumentWriteSchema, staging::bounded_value};
use crate::data::{Data, EncodedKey, KeyCodec, KeyRange, Mode, ScanOptions, Target};
use crate::{
    AvroDecode, AvroEncode, libertas_get_sys_ticks, libertas_get_utc_time,
    libertas_timer_new_interval, libertas_timer_update_interval,
};
use alloc::{boxed::Box, vec::Vec};
use core::ops::Bound;

const SWEEP_INTERVAL_US: u64 = 5 * 60 * 1_000_000;
const STAGES_PER_PAGE: u32 = 16;
const CHUNKS_PER_GENERATION: u32 = 64;

/// Identifies the upload entries this task sweeper owns. The prefix must
/// select only upload-state keys managed by the adapter.
pub trait DocumentSweepSchema<K, V>: DocumentWriteSchema<K, V> {
    fn stage_prefix(&self) -> &[u8];
    fn generation_from_stage_key(&self, key: &EncodedKey<K>) -> Option<[u8; 16]>;

    /// The manifest key gives adapters that scope chunk keys by document
    /// enough information to reconstruct them during a task-wide sweep.
    fn sweep_chunk_key(
        &self,
        _manifest_key: &EncodedKey<K>,
        generation: &[u8; 16],
        index: u32,
    ) -> K {
        self.chunk_key(generation, index)
    }
}

impl<K: KeyCodec + 'static, V: AvroEncode + AvroDecode + 'static> Data<K, V> {
    /// Runs the first sweep page during task startup and schedules periodic
    /// bounded pages while the task remains active. Returns the task timer ID.
    pub fn start_document_sweeper(
        &self,
        schema: impl DocumentSweepSchema<K, V> + 'static,
        mut on_error: impl FnMut(DocumentWriteError) + 'static,
    ) -> Result<u32, DocumentWriteError> {
        let mut after = None;
        self.sweep_document_page(&schema, &mut after)?;
        let data = self.duplicate();
        Ok(libertas_timer_new_interval(
            libertas_get_sys_ticks().saturating_add(SWEEP_INTERVAL_US),
            move |timer, tick, _| {
                if let Err(error) = data.sweep_document_page(&schema, &mut after) {
                    on_error(error);
                }
                libertas_timer_update_interval(timer, tick.saturating_add(SWEEP_INTERVAL_US));
            },
            Box::new(()),
        ))
    }

    /// Processes one page of upload entries. `after` is a task-local scan
    /// position and resets at the end of a pass. Durable chunk cursors are
    /// stored with each retired generation, so restarting a pass is safe.
    pub fn sweep_document_page(
        &self,
        schema: &impl DocumentSweepSchema<K, V>,
        after: &mut Option<EncodedKey<K>>,
    ) -> Result<(), DocumentWriteError> {
        let prefix = schema.stage_prefix();
        if prefix.is_empty()
            || after
                .as_ref()
                .is_some_and(|key| !key.as_bytes().starts_with(prefix))
        {
            return Err(DocumentWriteError::InvalidKey);
        }
        let target = Target::prefix(prefix).map_err(|_| DocumentWriteError::InvalidKey)?;
        let lower = after.clone().map_or(Bound::Unbounded, Bound::Excluded);
        let range = KeyRange::new(lower, Bound::Unbounded);
        let page = self.scan_keys(&target, &range, &ScanOptions::new(STAGES_PER_PAGE));
        for key in page.items {
            let result = self.sweep_document_generation(schema, &key);
            *after = Some(key);
            result?;
        }
        if page.next.is_none() {
            *after = None;
        }
        Ok(())
    }

    fn sweep_document_generation(
        &self,
        schema: &impl DocumentSweepSchema<K, V>,
        stage_key: &EncodedKey<K>,
    ) -> Result<(), DocumentWriteError> {
        let stage_target =
            Target::keys(&[stage_key.into()]).map_err(|_| DocumentWriteError::InvalidKey)?;
        let Some(value) = self.get(&stage_target, stage_key.into()) else {
            return Ok(());
        };
        let mut stage = schema
            .stage_from_value(value)
            .ok_or(DocumentWriteError::InvalidStage)?;
        let start = match stage.status {
            DocumentStageStatus::Retired { next_chunk } => next_chunk,
            DocumentStageStatus::Open | DocumentStageStatus::Sealed(_)
                if libertas_get_utc_time().is_some_and(|now| now >= stage.expires_at_utc_us) =>
            {
                0
            }
            _ => return Ok(()),
        };
        if start > stage.chunk_count {
            return Err(DocumentWriteError::InvalidStage);
        }
        let generation = schema
            .generation_from_stage_key(stage_key)
            .ok_or(DocumentWriteError::InvalidKey)?;
        let manifest_key = EncodedKey::from_bytes(stage.manifest_key.clone());
        let end = stage
            .chunk_count
            .min(start.saturating_add(CHUNKS_PER_GENERATION));
        let chunk_keys = (start..end)
            .map(|index| {
                EncodedKey::new(&schema.sweep_chunk_key(&manifest_key, &generation, index))
                    .map_err(|_| DocumentWriteError::InvalidKey)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut keys: Vec<_> = [&manifest_key, stage_key]
            .into_iter()
            .chain(&chunk_keys)
            .collect();
        keys.sort_unstable();
        let keys: Vec<_> = keys.into_iter().map(Into::into).collect();
        let target = Target::keys(&keys).map_err(|_| DocumentWriteError::InvalidKey)?;
        let mut txn = self.begin(&target, Mode::ReadWrite);
        let current = txn
            .get(stage_key.into())
            .and_then(|value| schema.stage_from_value(value));
        if current.as_ref() != Some(&stage) {
            return Ok(());
        }
        if let Some(value) = txn.get((&manifest_key).into()) {
            let manifest = schema
                .manifest_from_value(value)
                .ok_or(DocumentWriteError::InvalidManifest)?;
            if manifest.generation == generation {
                return Err(DocumentWriteError::InvalidStage);
            }
        }
        for key in &chunk_keys {
            txn.delete(key.into());
        }
        if end == stage.chunk_count {
            txn.delete(stage_key.into());
        } else {
            stage.status = DocumentStageStatus::Retired { next_chunk: end };
            let value = bounded_value(schema.stage_value(stage))?;
            txn.put(stage_key.into(), &value);
        }
        txn.commit();
        Ok(())
    }
}
