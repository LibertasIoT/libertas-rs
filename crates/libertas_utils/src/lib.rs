//! Low-level, allocation-conscious utilities for Libertas Rust applications.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::{mem::MaybeUninit, slice};

/// Number of bytes stored inline by [`InlineByteBuffer`].
pub const STACK_BUF_SIZE: usize = 1000;

/// Byte buffer with [`STACK_BUF_SIZE`] bytes of uninitialized inline storage.
///
/// A locally constructed buffer uses its inline storage, normally placing the
/// bytes on the stack without a heap allocation or byte initialization. If more
/// capacity is required, the initialized bytes are moved into a [`Vec<u8>`] and
/// all subsequent storage remains on the heap.
///
/// Only the initialized prefix is exposed through [`Self::as_slice`] and
/// [`Self::as_mut_slice`]. Use [`Self::spare_capacity_mut`] and
/// [`Self::set_len`] when a writer needs direct access to uninitialized space.
pub struct InlineByteBuffer {
    storage: InlineByteBufferStorage,
}

// The size difference is intentional: boxing the inline variant would defeat
// the buffer's allocation-free fast path.
#[allow(clippy::large_enum_variant)]
enum InlineByteBufferStorage {
    Inline {
        data: [MaybeUninit<u8>; STACK_BUF_SIZE],
        len: usize,
    },
    Heap(Vec<u8>),
}

impl InlineByteBuffer {
    /// Creates an empty buffer backed by uninitialized inline storage.
    ///
    /// This does not allocate on the heap or initialize the inline bytes.
    pub fn new() -> Self {
        Self {
            storage: InlineByteBufferStorage::Inline {
                data: [const { MaybeUninit::uninit() }; STACK_BUF_SIZE],
                len: 0,
            },
        }
    }

    /// Creates an empty buffer with at least `capacity` bytes available.
    ///
    /// Capacities up to [`STACK_BUF_SIZE`] use inline storage. Larger
    /// capacities allocate a [`Vec<u8>`] immediately.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buffer = Self::new();
        buffer.reserve(capacity);
        buffer
    }

    /// Returns the number of initialized bytes in the buffer.
    pub fn len(&self) -> usize {
        match &self.storage {
            InlineByteBufferStorage::Inline { len, .. } => *len,
            InlineByteBufferStorage::Heap(data) => data.len(),
        }
    }

    /// Returns `true` when the buffer contains no initialized bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of bytes available without another allocation.
    pub fn capacity(&self) -> usize {
        match &self.storage {
            InlineByteBufferStorage::Inline { .. } => STACK_BUF_SIZE,
            InlineByteBufferStorage::Heap(data) => data.capacity(),
        }
    }

    /// Returns the initialized bytes.
    pub fn as_slice(&self) -> &[u8] {
        match &self.storage {
            InlineByteBufferStorage::Inline { data, len } => {
                // SAFETY: `len` only includes bytes initialized through the
                // safe mutation methods or covered by `set_len`'s contract.
                unsafe { slice::from_raw_parts(data.as_ptr().cast::<u8>(), *len) }
            }
            InlineByteBufferStorage::Heap(data) => data.as_slice(),
        }
    }

    /// Returns the initialized bytes mutably.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match &mut self.storage {
            InlineByteBufferStorage::Inline { data, len } => {
                // SAFETY: See `as_slice`; the initialized prefix is valid for
                // mutable access for the duration of this borrow.
                unsafe { slice::from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), *len) }
            }
            InlineByteBufferStorage::Heap(data) => data.as_mut_slice(),
        }
    }

    /// Returns the uninitialized capacity after the initialized bytes.
    ///
    /// Callers may write to this slice and then use [`Self::set_len`] to include
    /// the newly initialized bytes in the buffer.
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        match &mut self.storage {
            InlineByteBufferStorage::Inline { data, len } => &mut data[*len..],
            InlineByteBufferStorage::Heap(data) => data.spare_capacity_mut(),
        }
    }

    /// Ensures capacity for at least `additional` more bytes.
    ///
    /// The buffer remains inline when the requested total fits in
    /// [`STACK_BUF_SIZE`]. Otherwise, its initialized bytes are copied to a
    /// [`Vec<u8>`].
    pub fn reserve(&mut self, additional: usize) {
        let required = self
            .len()
            .checked_add(additional)
            .expect("InlineByteBuffer capacity overflow");
        if required <= self.capacity() {
            return;
        }

        match &mut self.storage {
            InlineByteBufferStorage::Inline { data, len } => {
                let heap_capacity = required.max(STACK_BUF_SIZE.saturating_mul(2));
                let mut heap = Vec::with_capacity(heap_capacity);
                // SAFETY: The prefix ending at `len` is initialized by the
                // buffer invariant.
                let initialized =
                    unsafe { slice::from_raw_parts(data.as_ptr().cast::<u8>(), *len) };
                heap.extend_from_slice(initialized);
                self.storage = InlineByteBufferStorage::Heap(heap);
            }
            InlineByteBufferStorage::Heap(data) => data.reserve(additional),
        }
    }

    /// Appends one initialized byte.
    pub fn push(&mut self, value: u8) {
        self.reserve(1);
        match &mut self.storage {
            InlineByteBufferStorage::Inline { data, len } => {
                data[*len].write(value);
                *len += 1;
            }
            InlineByteBufferStorage::Heap(data) => data.push(value),
        }
    }

    /// Appends initialized bytes from `bytes`.
    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.reserve(bytes.len());
        match &mut self.storage {
            InlineByteBufferStorage::Inline { data, len } => {
                for (slot, value) in data[*len..].iter_mut().zip(bytes) {
                    slot.write(*value);
                }
                *len += bytes.len();
            }
            InlineByteBufferStorage::Heap(data) => data.extend_from_slice(bytes),
        }
    }

    /// Removes all initialized bytes without changing the storage mode.
    pub fn clear(&mut self) {
        match &mut self.storage {
            InlineByteBufferStorage::Inline { len, .. } => *len = 0,
            InlineByteBufferStorage::Heap(data) => data.clear(),
        }
    }

    /// Sets the number of initialized bytes.
    ///
    /// # Safety
    ///
    /// - `new_len` must not exceed [`Self::capacity`].
    /// - Every byte between the previous length and `new_len` must already be
    ///   initialized, typically through [`Self::spare_capacity_mut`].
    pub unsafe fn set_len(&mut self, new_len: usize) {
        assert!(
            new_len <= self.capacity(),
            "InlineByteBuffer length exceeds capacity"
        );
        match &mut self.storage {
            InlineByteBufferStorage::Inline { len, .. } => *len = new_len,
            InlineByteBufferStorage::Heap(data) => {
                // SAFETY: The caller upholds the same initialization and
                // capacity requirements as `Vec::set_len`.
                unsafe { data.set_len(new_len) };
            }
        }
    }
}

impl Default for InlineByteBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for InlineByteBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for InlineByteBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_empty_with_inline_capacity() {
        let buffer = InlineByteBuffer::new();

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), STACK_BUF_SIZE);
        assert!(matches!(
            buffer.storage,
            InlineByteBufferStorage::Inline { .. }
        ));
    }

    #[test]
    fn appends_without_spilling_while_data_fits_inline() {
        let mut buffer = InlineByteBuffer::new();

        buffer.extend_from_slice(&[1, 2, 3]);
        buffer.push(4);
        buffer.as_mut_slice()[1] = 9;

        assert_eq!(buffer.as_slice(), &[1, 9, 3, 4]);
        assert!(matches!(
            buffer.storage,
            InlineByteBufferStorage::Inline { .. }
        ));
    }

    #[test]
    fn spills_to_vec_and_preserves_initialized_bytes() {
        let mut buffer = InlineByteBuffer::new();
        let inline_bytes = [0x5a; STACK_BUF_SIZE];

        buffer.extend_from_slice(&inline_bytes);
        buffer.push(0xa5);

        assert_eq!(buffer.len(), STACK_BUF_SIZE + 1);
        assert_eq!(&buffer.as_slice()[..STACK_BUF_SIZE], &inline_bytes);
        assert_eq!(buffer.as_slice()[STACK_BUF_SIZE], 0xa5);
        assert!(matches!(buffer.storage, InlineByteBufferStorage::Heap(_)));
    }

    #[test]
    fn supports_writing_into_uninitialized_capacity() {
        let mut buffer = InlineByteBuffer::new();
        let spare = buffer.spare_capacity_mut();
        spare[0].write(7);
        spare[1].write(8);
        spare[2].write(9);

        // SAFETY: The first three spare bytes were initialized above.
        unsafe { buffer.set_len(3) };

        assert_eq!(buffer.as_slice(), &[7, 8, 9]);
    }

    #[test]
    fn large_requested_capacity_starts_on_heap() {
        let buffer = InlineByteBuffer::with_capacity(STACK_BUF_SIZE + 1);

        assert!(buffer.is_empty());
        assert!(buffer.capacity() > STACK_BUF_SIZE);
        assert!(matches!(buffer.storage, InlineByteBufferStorage::Heap(_)));
    }
}
