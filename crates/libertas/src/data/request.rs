//! Borrowed SDK requests, serialized before crossing the host/dispatcher boundary.
//! The owned wire types remain the canonical decoder and retained representation.
use super::*;
use crate::InlineByteBuffer;
use libertas_macros::variant_index;

pub(crate) enum Operation<'a, K, V> {
    Get(KeyRef<'a, K>),
    GetMany(&'a [KeyRef<'a, K>]),
    Exists(KeyRef<'a, K>),
    Scan(&'a KeyRange<K>, &'a ScanOptions<K>, bool),
    Put { key: KeyRef<'a, K>, value: &'a V },
    Delete(KeyRef<'a, K>),
    WriteBatch(&'a [Mutation<'a, K, V>]),
}

pub(crate) enum Command<'a, K, V> {
    Begin {
        target: &'a Target<K>,
        mode: Mode,
    },
    Execute {
        target: &'a Target<K>,
        operation: Operation<'a, K, V>,
    },
    ExecuteInTransaction {
        transaction: u64,
        operation: Operation<'a, K, V>,
    },
    Commit(u64),
    Rollback(u64),
}

impl<K: KeyCodec, V: AvroEncode> Command<'_, K, V> {
    pub(crate) fn encode(self, output: &mut InlineByteBuffer) -> Result<(), DataError> {
        match self {
            Self::Begin { target, mode } => {
                tag(variant_index!(wire::Command::Begin), output);
                target.raw.avro_encode(output);
                mode.avro_encode(output);
            }
            Self::Execute { target, operation } => {
                tag(variant_index!(wire::Command::Execute), output);
                target.raw.avro_encode(output);
                operation.encode(output)?;
            }
            Self::ExecuteInTransaction {
                transaction,
                operation,
            } => {
                tag(variant_index!(wire::Command::ExecuteInTransaction), output);
                transaction.avro_encode(output);
                operation.encode(output)?;
            }
            Self::Commit(id) => {
                tag(variant_index!(wire::Command::Commit), output);
                id.avro_encode(output);
            }
            Self::Rollback(id) => {
                tag(variant_index!(wire::Command::Rollback), output);
                id.avro_encode(output);
            }
        }
        Ok(())
    }
}

impl<K: KeyCodec, V: AvroEncode> Operation<'_, K, V> {
    fn encode(self, output: &mut InlineByteBuffer) -> Result<(), DataError> {
        match self {
            Self::Get(key) => {
                tag(variant_index!(wire::Operation::Get), output);
                key.encode_bytes(output)?;
            }
            Self::GetMany(keys) => {
                tag(variant_index!(wire::Operation::GetMany), output);
                (keys.len() as i64).avro_encode(output);
                if !keys.is_empty() {
                    for key in keys {
                        key.encode_bytes(output)?;
                    }
                    0i64.avro_encode(output);
                }
            }
            Self::Exists(key) => {
                tag(variant_index!(wire::Operation::Exists), output);
                key.encode_bytes(output)?;
            }
            Self::Scan(range, options, keys_only) => {
                tag(variant_index!(wire::Operation::Scan), output);
                range.raw.avro_encode(output);
                options.direction.avro_encode(output);
                options.limit.avro_encode(output);
                match &options.continuation {
                    None => 0i64.avro_encode(output),
                    Some(continuation) => {
                        1i64.avro_encode(output);
                        continuation.bytes.avro_encode(output);
                    }
                }
                keys_only.avro_encode(output);
            }
            Self::Put { key, value } => {
                tag(variant_index!(wire::Operation::Put), output);
                key.encode_bytes(output)?;
                value_bytes(value, output);
            }
            Self::Delete(key) => {
                tag(variant_index!(wire::Operation::Delete), output);
                key.encode_bytes(output)?;
            }
            Self::WriteBatch(batch) => {
                tag(variant_index!(wire::Operation::WriteBatch), output);
                (batch.len() as i64).avro_encode(output);
                if !batch.is_empty() {
                    // Serialize one item at a time: no Vec of encoded keys,
                    // values or mutations, and no inline buffer per retained item.
                    for mutation in batch {
                        match mutation {
                            Mutation::Put(key, value) => {
                                tag(variant_index!(wire::Mutation::Put), output);
                                key.encode_bytes(output)?;
                                value_bytes(*value, output);
                            }
                            Mutation::Delete(key) => {
                                tag(variant_index!(wire::Mutation::Delete), output);
                                key.encode_bytes(output)?;
                            }
                        }
                    }
                    0i64.avro_encode(output);
                }
            }
        }
        Ok(())
    }
}

impl<K: KeyCodec> KeyRef<'_, K> {
    fn encode_bytes(&self, output: &mut InlineByteBuffer) -> Result<(), DataError> {
        match self {
            Self::Typed(key) => {
                (key.encoded_len() as i64).avro_encode(output);
                key.encode_key_to_buffer(output)
                    .map_err(|_| DataError::InvalidRequest)
            }
            Self::Encoded(key) => {
                key.bytes.avro_encode(output);
                Ok(())
            }
        }
    }
}

fn tag(index: usize, output: &mut InlineByteBuffer) {
    (index as i64).avro_encode(output);
}

fn value_bytes(value: &impl AvroEncode, output: &mut InlineByteBuffer) {
    // Avro Bytes requires a length prefix. Encode the value once into a local
    // inline buffer, then append its length and bytes to the complete frame.
    let mut value_buffer = InlineByteBuffer::new();
    value.avro_encode(&mut value_buffer);
    (value_buffer.len() as i64).avro_encode(output);
    output.extend_from_slice(value_buffer.as_slice());
}

pub(crate) fn frame<K: KeyCodec, V: AvroEncode>(
    id: u64,
    checked: bool,
    command: Command<'_, K, V>,
) -> Result<InlineByteBuffer, DataError> {
    let mut bytes = InlineByteBuffer::new();
    wire::VERSION.avro_encode(&mut bytes);
    id.avro_encode(&mut bytes);
    checked.avro_encode(&mut bytes);
    command.encode(&mut bytes)?;
    Ok(bytes)
}

#[cfg(feature = "tokio")]
pub(crate) fn work<K: KeyCodec, V: AvroEncode>(
    ticket: u64,
    checked: bool,
    command: Command<'_, K, V>,
) -> Result<InlineByteBuffer, DataError> {
    let mut bytes = InlineByteBuffer::new();
    ticket.avro_encode(&mut bytes);
    checked.avro_encode(&mut bytes);
    command.encode(&mut bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_allocations::{allocations, suspend};
    use alloc::{string::String, vec};
    use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode, LibertasKeyCodec};
    use std::sync::Mutex;

    #[derive(LibertasKeyCodec)]
    enum Key {
        Item(u64),
        Name(#[libertas_size(max = 4)] String),
    }
    #[derive(LibertasAvroEncode, LibertasAvroDecode)]
    enum Value {
        Number(u32),
        Bytes(Vec<u8>),
    }

    #[test]
    fn borrowed_requests_match_every_owned_command_and_operation() {
        let key = Key::Item(42);
        let encoded = EncodedKey::new(&key).unwrap();
        let targets = [Target::all(), Target::keys(&[(&key).into()]).unwrap()];
        let keys = [(&key).into(), (&encoded).into()];
        let value = Value::Number(42);
        let batch = [
            Mutation::Put((&key).into(), &value),
            Mutation::Delete((&encoded).into()),
        ];
        let range = KeyRange::new(
            Bound::Included(encoded.clone()),
            Bound::Excluded(encoded.clone()),
        );
        let options = ScanOptions {
            direction: Direction::Descending,
            limit: 7,
            continuation: Some(Continuation {
                bytes: vec![1, 2, 3],
                marker: PhantomData,
            }),
        };
        for target in &targets {
            let operations = vec![
                (
                    Operation::Get((&key).into()),
                    wire::Operation::Get(encoded.bytes.clone()),
                ),
                (
                    Operation::Exists((&encoded).into()),
                    wire::Operation::Exists(encoded.bytes.clone()),
                ),
                (
                    Operation::GetMany(&keys),
                    wire::Operation::GetMany(vec![encoded.bytes.clone(); 2]),
                ),
                (Operation::GetMany(&[]), wire::Operation::GetMany(vec![])),
                (
                    Operation::Scan(&range, &options, true),
                    wire::Operation::Scan(wire::Scan {
                        range: range.raw.clone(),
                        direction: options.direction,
                        limit: 7,
                        continuation: Some(vec![1, 2, 3]),
                        keys_only: true,
                    }),
                ),
                (
                    Operation::Put {
                        key: (&key).into(),
                        value: &value,
                    },
                    wire::Operation::Put {
                        key: encoded.bytes.clone(),
                        value: wire::encode(&value),
                    },
                ),
                (
                    Operation::Delete((&key).into()),
                    wire::Operation::Delete(encoded.bytes.clone()),
                ),
                (
                    Operation::WriteBatch(&batch),
                    wire::Operation::WriteBatch(vec![
                        wire::Mutation::Put {
                            key: encoded.bytes.clone(),
                            value: wire::encode(&value),
                        },
                        wire::Mutation::Delete {
                            key: encoded.bytes.clone(),
                        },
                    ]),
                ),
                (
                    Operation::WriteBatch(&[]),
                    wire::Operation::WriteBatch(vec![]),
                ),
            ];
            for (operation, owned) in operations {
                check(
                    Command::Execute { target, operation },
                    wire::Command::Execute {
                        target: target.raw.clone(),
                        operation: owned,
                    },
                );
            }
            for mode in [Mode::ReadOnly, Mode::ReadWrite] {
                check(
                    Command::Begin { target, mode },
                    wire::Command::Begin {
                        target: target.raw.clone(),
                        mode,
                    },
                );
            }
        }
        check(
            Command::ExecuteInTransaction {
                transaction: 99,
                operation: Operation::Get((&key).into()),
            },
            wire::Command::ExecuteInTransaction {
                transaction: 99,
                operation: wire::Operation::Get(encoded.bytes),
            },
        );
        check(Command::Commit(99), wire::Command::Commit(99));
        check(Command::Rollback(99), wire::Command::Rollback(99));

        fn check(command: Command<'_, Key, Value>, owned: wire::Command) {
            let expected = wire::Request {
                version: wire::VERSION,
                id: u64::MAX,
                checked: true,
                command: owned,
            };
            let (bytes, count) = allocations(|| frame(u64::MAX, true, command).unwrap());
            assert_eq!(count, 0);
            assert_eq!(bytes.as_slice(), wire::encode(&expected));
            assert_eq!(wire::decode_request(bytes.as_slice()).unwrap(), expected);
        }
    }

    static REQUESTS: Mutex<Vec<wire::Request>> = Mutex::new(Vec::new());
    static REPLY: Mutex<Vec<u8>> = Mutex::new(Vec::new());
    static SENDS: Mutex<Vec<(u8, Vec<u8>)>> = Mutex::new(Vec::new());
    extern "C" fn read(
        _: u16,
        _: crate::LibertasDevice,
        opcode: u8,
        data: *const u8,
        len: usize,
    ) -> crate::ReadResult {
        let _host = suspend();
        if opcode == 0xe7 {
            return crate::ReadResult {
                success: true,
                data: core::ptr::null(),
                data_len: 0,
            };
        }
        assert_eq!(opcode, 0xe8);
        let request =
            wire::decode_request(unsafe { core::slice::from_raw_parts(data, len) }).unwrap();
        let result = match &request.command {
            wire::Command::Begin { .. } => wire::Result::Begun(99),
            wire::Command::Execute { operation, .. }
            | wire::Command::ExecuteInTransaction { operation, .. } => match operation {
                wire::Operation::Get(_) => wire::Result::Value(None),
                wire::Operation::Exists(_) => wire::Result::Exists(false),
                wire::Operation::Scan(_) => wire::Result::Keys {
                    items: vec![],
                    continuation: None,
                },
                _ => wire::Result::Done,
            },
            _ => wire::Result::Done,
        };
        let mut reply = REPLY.lock().unwrap();
        reply.clear();
        wire::Response {
            id: request.id,
            result,
        }
        .avro_encode(&mut *reply);
        REQUESTS.lock().unwrap().push(request);
        crate::ReadResult {
            success: true,
            data: reply.as_ptr(),
            data_len: reply.len(),
        }
    }
    extern "C" fn send(
        _: u16,
        _: crate::LibertasDevice,
        _: u32,
        opcode: u8,
        data: *const u8,
        len: usize,
        _: crate::LibertasPeer,
    ) {
        let _host = suspend();
        // Notification input is a native pointer record. Only endpoint payloads
        // are Avro bytes at this ABI location; leave notification pointers alone.
        let payload = if opcode == crate::OP_SYSTEM_MESSAGE {
            vec![]
        } else {
            unsafe { core::slice::from_raw_parts(data, len) }.to_vec()
        };
        SENDS.lock().unwrap().push((opcode, payload));
    }

    #[test]
    fn public_storage_endpoint_and_notification_apis_use_inline_buffers() {
        let _guard = crate::tests::TEST_LOCK.lock().unwrap();
        let mut api = crate::tests::fake_runtime_api();
        api.device_read = read;
        api.device_send = send;
        crate::__libertas_init_package((&raw mut api).cast());
        crate::libertas_impl_init_task(7);
        __bind_schema::<Key, Value>();
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                crate::__libertas_release_package();
                unsafe {
                    crate::RUNTIME_API = core::ptr::null_mut();
                }
            }
        }
        let _reset = Reset;
        REQUESTS.lock().unwrap().clear();
        SENDS.lock().unwrap().clear();
        let data = Data::<Key, Value>::current();
        let key = Key::Item(42);
        let value = Value::Number(42);
        let target = Target::keys(&[(&key).into()]).unwrap();
        let batch = [
            Mutation::Put((&key).into(), &value),
            Mutation::Delete((&key).into()),
        ];
        let ((), count) = allocations(|| {
            assert!(data.get(&target, (&key).into()).is_none());
            assert!(!data.exists(&target, (&key).into()));
            data.put(&target, (&key).into(), &value);
            data.delete(&target, (&key).into());
            data.write_batch(&target, &batch);
            assert!(
                data.scan_keys(&target, &KeyRange::all(), &ScanOptions::new(2))
                    .items
                    .is_empty()
            );
            let mut transaction = data.begin(&target, Mode::ReadWrite);
            transaction.put((&key).into(), &value);
            transaction.write_batch(&batch);
            transaction.commit();
            data.begin(&target, Mode::ReadOnly).rollback();
            crate::libertas_endpoint_request(8, &value);
            crate::libertas_endpoint_subscribe_request(8, &value);
            crate::libertas_endpoint_response(8, &value, 1, 9);
            crate::libertas_endpoint_edit_request(8, &value, 1, 9);
            crate::libertas_endpoint_report(8, &value, Some(9));
            crate::libertas_notification_send(
                &[1],
                crate::NotificationImportance::Info,
                Some(1),
                "example",
                &[],
            );
        });
        assert_eq!(
            count, 0,
            "small App-facing calls must not allocate encoding buffers"
        );
        let requests = REQUESTS.lock().unwrap();
        assert_eq!(requests.len(), 12);
        assert!(requests.iter().all(|r| !r.checked));
        drop(requests);
        let sends = SENDS.lock().unwrap();
        assert_eq!(sends.len(), 6);
        for (_, payload) in &sends[..5] {
            assert_eq!(
                payload,
                &[crate::LibertasEndpointStatus::Success as u8, 0, 84]
            );
        }
        drop(sends);

        let large = Value::Bytes(vec![0xa5; crate::STACK_BUF_SIZE + 10]);
        let ((), count) = allocations(|| data.put(&target, (&key).into(), &large));
        assert!(count > 0, "large values must spill");
        let requests = REQUESTS.lock().unwrap();
        if let wire::Command::Execute {
            operation: wire::Operation::Put { value, .. },
            ..
        } = &requests.last().unwrap().command
        {
            assert_eq!(value, &wire::encode(&large));
        } else {
            panic!("expected put");
        }
        let before = requests.len();
        drop(requests);
        let invalid = Key::Name(String::from("too long"));
        let bad_batch = [
            Mutation::Put((&key).into(), &value),
            Mutation::Delete((&invalid).into()),
        ];
        assert_eq!(
            data.try_write_batch(&target, &bad_batch),
            Err(DataError::InvalidRequest)
        );
        assert_eq!(
            REQUESTS.lock().unwrap().len(),
            before,
            "partial batches cannot reach the host"
        );
    }
}
