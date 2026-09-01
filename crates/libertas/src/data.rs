//! Libertas Rust SDK - Data API
//!
//! Functions for single-record and indexed data operations. Every logical data
//! name is one canonical byte blob containing a package-local LMF1 resource key
//! and ordered, typed Libertas Message Arguments. The blob is both stable
//! storage identity and the complete input for localized human presentation.

use crate::*;
use alloc::string::String;
use alloc::vec::Vec;
use libertas_macros::{LibertasAvroDecode, LibertasAvroEncode};
use notification::*;

/// Direction for indexed data reads.
/// - `Above`: Index >= start.
/// - `Below`: Index <= start.
#[repr(u8)]
pub enum IndexDirection {
    Above,
    Below,
}

/// Indexed data record with index and decoded data.
pub struct IndexedData<T> where T: AvroDecode {
    pub index: i64,
    pub data: T,
}

/// Decoded view of a canonical Libertas data name.
///
/// At the storage boundary, `resource_name` and `arguments` are encoded into
/// one byte blob. That blob is the sole identity of a single record or indexed
/// database/table; it is not a literal or rendered String name. The same tuple
/// is also a FormattedText value: the current task's package selects the
/// `StringResources` catalog, `resource_name` selects an LMF1 template, and the
/// typed arguments supply its data for localized human presentation.
///
/// Data functions accept the tuple and perform the canonical encoding. App code
/// must not pre-render, Base64-wrap, or hand-encode it. A translation or template
/// wording change leaves identity intact; changing the key, argument order,
/// argument type, or argument value creates a different identity.
///
/// Single-record and indexed-database names occupy separate namespaces even
/// when their bytes match. An indexed row's signed index is additional identity
/// and is not part of this tuple.
#[derive(LibertasAvroEncode, LibertasAvroDecode)]
pub struct DataName {
    /// Package-local key of the LMF1 template in `StringResources`.
    pub resource_name: String,

    /// Ordered, typed values used for both byte identity and human rendering.
    pub arguments: Vec<LibertasMessageArgumentDecode>,
}

#[repr(C)]
struct DataSingleInternal {
    name: *const u8,
    name_len: usize,
    value: *const u8,
    value_len: usize,
}

#[repr(C)]
struct DataWriteIndexedReq {
    db: LibertasDataStore,
    index: i64,
    value: *const u8,
    value_len: usize,
}

#[repr(C)]
struct DataDeleteIndexedRecordsReq {
    db: LibertasDataStore,
    index_lo: i64,
    index_hi: i64,
}

#[repr(C)]
struct DataReadIndexedReq {
    db: LibertasDataStore,
    index: i64,
    direction: IndexDirection,
    max_n: usize,
}

#[repr(C)]
struct DataIndexedRaw {
    index: i64,
    value: *const u8,
    value_len: usize,
}

/// Statistics for opened indexed data.
/// Contains handle, record count, and index range.
/// If `count` is 0, then `min_index` and `max_index` are undefined. Otherwise, valid indices are in the range [min_index, max_index].
#[repr(C)]
pub struct IndexedDataStat {
    pub handle: LibertasDataStore,
    pub count: u64,
    pub min_index: i64,
    pub max_index: i64,
}

/// Returns all single-record data names.
/// 
/// # Returns
/// Vector of data names with resource and arguments.
pub fn libertas_data_get_single_names() -> Vec<DataName> {
    let result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_SINGLE, OP_SYSTEM_DATABASE_GET_NAMES, core::ptr::null(), 0);
    let mut names = Vec::new();
    if result.success {
        let data_slice = unsafe { core::slice::from_raw_parts(result.data, result.data_len) };
        let mut index = 0;
        while index < data_slice.len() {
            let name = DataName::avro_decode(data_slice, &mut index).unwrap();
            names.push(name);
        }
    }
    names
}

/// Returns all indexed data names.
/// 
/// # Returns
/// Vector of indexed data names.
pub fn libertas_data_get_indexed_names() -> Vec<DataName> {
    let result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_GET_NAMES, core::ptr::null(), 0);
    let mut names = Vec::new();
    if result.success {
        let data_slice = unsafe { core::slice::from_raw_parts(result.data, result.data_len) };
        let mut index = 0;
        while index < data_slice.len() {
            let name = DataName::avro_decode(data_slice, &mut index).unwrap();
            names.push(name);
        }
    }
    names
}

/// Removes the single record identified by the canonical encoding of a
/// package-local LMF1 resource key and its ordered, typed arguments.
pub fn libertas_data_remove_single(resource_name: &str, arguments: &[LibertasMessageArgument]) {
    let serialized = libertas_formatted_text(resource_name, arguments);
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_SINGLE, OP_SYSTEM_DATABASE_REMOVE_DATA, 0, 0, serialized.as_ptr(), serialized.len());
}

/// Removes the complete indexed database identified and named by a
/// package-local LMF1 resource key and its ordered, typed arguments.
///
/// # Arguments
/// * `resource_name` - Package-local LMF1 template key in `StringResources`.
/// * `arguments` - Ordered, typed values that must exactly match the tuple used
///   when the indexed database was opened.
///
/// Unlike a single record, indexed data is organized by an index value for
/// ordered lookup. This operation removes the complete indexed database named
/// by the resource and arguments, regardless of its records' index values.
///
pub fn libertas_data_remove_indexed(resource_name: &str, arguments: &[LibertasMessageArgument]) {
    let serialized = libertas_formatted_text(resource_name, arguments);
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_REMOVE_DATA, 0, 0, serialized.as_ptr(), serialized.len());
}

/// Removes the complete indexed database identified by an enumerated data
/// name.
///
/// This is the lossless counterpart to [`libertas_data_get_indexed_names`]: it
/// preserves the decoded argument types and values without requiring the App
/// to reconstruct borrowed [`LibertasMessageArgument`] values.
pub fn libertas_data_remove_indexed_name(data_name: &DataName) {
    let mut serialized = Vec::new();
    data_name.avro_encode(&mut serialized);
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_REMOVE_DATA, 0, 0, serialized.as_ptr(), serialized.len());
}

/// Removes indexed records in the given index range.
/// 
/// # Arguments
/// * `db` - Database handle. See `libertas_data_open_indexed` for obtaining the handle.
/// * `index_lo` - Lower index bound (inclusive).
/// * `index_hi` - Upper index bound (inclusive).
pub fn libertas_data_remove_indexed_records(db: LibertasDataStore, index_lo: i64, index_hi: i64) {
    let req: DataDeleteIndexedRecordsReq = DataDeleteIndexedRecordsReq {
        db,
        index_lo,
        index_hi
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_REMOVE_RECORD, 0, 0, &req as *const DataDeleteIndexedRecordsReq as *const u8, core::mem::size_of::<DataDeleteIndexedRecordsReq>());
}

/// Opens indexed data and returns handle with stats.
/// 
/// # Arguments
/// * `resource_name` - Package-local LMF1 template key in `StringResources`.
/// * `arguments` - Ordered, typed values identifying and naming the indexed
///   database.
/// 
/// # Returns
/// IndexedDataStat with handle, count, and index range. If `count` is 0, then `min_index` and `max_index` are undefined.
pub fn libertas_data_open_indexed(resource_name: &str, arguments: &[LibertasMessageArgument]) -> IndexedDataStat {
    let serialized = libertas_formatted_text(resource_name, arguments);
    let result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_OPEN_INDEXED_DATA, serialized.as_ptr(), serialized.len());
    if result.success && result.data_len == core::mem::size_of::<IndexedDataStat>() {
        let stat = unsafe { &*(result.data as *const IndexedDataStat) };
        IndexedDataStat {
            handle: stat.handle,
            count: stat.count,
            min_index: stat.min_index,
            max_index: stat.max_index,
        }
    } else {
        panic!("Failed to open indexed data");
    }
}

/// Writes a single record.
/// 
/// # Arguments
/// * `resource_name` - Package-local LMF1 template key in `StringResources`.
/// * `arguments` - Ordered, typed values identifying and naming the record.
/// * `data` - Encodable data.
pub fn libertas_data_write_single(resource_name: &str, arguments: &[LibertasMessageArgument], data: &dyn AvroEncode) {
    let mut serialized = libertas_formatted_text(resource_name, arguments);
    let name_len = serialized.len();
    data.avro_encode(&mut serialized);
    let total_len = serialized.len();

    let name_ptr = serialized.as_ptr();
    let value_ptr = unsafe { name_ptr.add(name_len) };

    let data_internal = DataSingleInternal {
        name: name_ptr,
        name_len: name_len,
        value: value_ptr,
        value_len: total_len - name_len,
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_SINGLE, OP_SYSTEM_DATABASE_WRITE_DATA, 0, 0, &data_internal as *const DataSingleInternal as *const u8, core::mem::size_of::<DataSingleInternal>());
}

/// Writes indexed data.
/// 
/// # Arguments
/// * `db` - Database handle. See `libertas_data_open_indexed` for obtaining the handle.
/// * `index` - Record index.
/// * `data` - Encodable data.
pub fn libertas_data_write_indexed(db: LibertasDataStore, index: i64, data: &dyn AvroEncode) {
    let mut value = Vec::new();
    data.avro_encode(&mut value);

    let data_internal = DataWriteIndexedReq {
        db,
        index,
        value: value.as_ptr(),
        value_len: value.len(),
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_WRITE_DATA, 0, 0, &data_internal as *const DataWriteIndexedReq as *const u8, core::mem::size_of::<DataWriteIndexedReq>());
}

/// Reads a single record.
/// 
/// # Arguments
/// * `resource_name` - Package-local LMF1 template key in `StringResources`.
/// * `arguments` - Ordered, typed values identifying and naming the record.
/// 
/// # Returns
/// Decoded data or None if not found.
pub fn libertas_data_read_single<T>(resource_name: &str, arguments: &[LibertasMessageArgument]) -> Option<T> where T: AvroDecode {
    let name = libertas_formatted_text(resource_name, arguments);
    let result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_SINGLE, OP_SYSTEM_DATABASE_READ_DATA, name.as_ptr(), name.len());
    return if result.success {
        let data_slice = unsafe { core::slice::from_raw_parts(result.data, result.data_len) };
        let mut index = 0;
        Some(T::avro_decode(data_slice, &mut index).unwrap())
    } else {
        None
    };  
}

/// Reads indexed data range.
/// 
/// # Arguments
/// * `db` - Database handle. See `libertas_data_open_indexed` for obtaining the handle.
/// * `index` - Starting index.
/// * `direction` - Read direction.
/// * `max_n` - Max records to read.
/// * `results` - Output vector for records.
pub fn libertas_data_read_indexed_range<T>(db: LibertasDataStore, index: i64, direction: IndexDirection, max_n: usize, results: &mut Vec<IndexedData<T>>) where T: AvroDecode {
    let req = DataReadIndexedReq {
        db,
        index,
        direction,
        max_n,
    };
    let read_result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_READ_DATA, &req as *const DataReadIndexedReq as *const u8, core::mem::size_of::<DataReadIndexedReq>());
    if read_result.success {
        let data_slice = unsafe { core::slice::from_raw_parts(read_result.data, read_result.data_len) };
        let mut index = 0;
        while index < data_slice.len() {
            let record = unsafe { &*(data_slice.as_ptr().add(index) as *const DataIndexedRaw) };
            index += core::mem::size_of::<DataIndexedRaw>();
            let value_slice = unsafe { core::slice::from_raw_parts(record.value, record.value_len) };
            let mut value_index = 0;
            let data = T::avro_decode(value_slice, &mut value_index).unwrap();
            results.push(IndexedData {
                index: record.index,
                data,
            });
        }
    }
}

/// Reads a single indexed data record.
/// 
/// # Arguments
/// * `db` - Database handle. See `libertas_data_open_indexed` for obtaining the handle.
/// * `index` - Record index.
/// 
/// # Returns
/// Decoded record or None if not found.
pub fn libertas_data_read_indexed<T>(db: LibertasDataStore, index: i64) -> Option<IndexedData<T>> where T: AvroDecode {
    let req = DataReadIndexedReq {
        db,
        index,
        direction: IndexDirection::Above,
        max_n: 1,
    };
    let read_result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_READ_DATA, &req as *const DataReadIndexedReq as *const u8, core::mem::size_of::<DataReadIndexedReq>());
    if read_result.success {
        if read_result.data_len > 0 {
            let record = unsafe { &*(read_result.data as *const DataIndexedRaw) };
            let value_slice = unsafe { core::slice::from_raw_parts(record.value, record.value_len) };
            let mut value_index = 0;
            let data = IndexedData::<T> {
                index: record.index,
                data: T::avro_decode(value_slice, &mut value_index).unwrap(),
            };
            return Some(data);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoded_data_name_matches_formatted_text_bytes() {
        let arguments = [
            LibertasMessageArgument::LiteralText("north greenhouse"),
            LibertasMessageArgument::Object(42),
            LibertasMessageArgument::Boolean(true),
            LibertasMessageArgument::Signed(-9),
            LibertasMessageArgument::Unsigned(9),
            LibertasMessageArgument::Float(1.25),
            LibertasMessageArgument::Double(-2.5),
            LibertasMessageArgument::UnitSigned {
                unit_type: "celsius",
                value: -3,
            },
            LibertasMessageArgument::UnitUnsigned {
                unit_type: "byte",
                value: 1024,
            },
            LibertasMessageArgument::UnitFloat {
                unit_type: "meter-per-second",
                value: 1.5,
            },
            LibertasMessageArgument::UnitDouble {
                unit_type: "millimeter",
                value: 2.75,
            },
            LibertasMessageArgument::ResourceText("READY"),
            LibertasMessageArgument::Plural(3),
        ];
        let expected = libertas_formatted_text("DEVICE_HISTORY", &arguments);

        let decoded = DataName {
            resource_name: String::from("DEVICE_HISTORY"),
            arguments: Vec::from([
                LibertasMessageArgumentDecode::LiteralText(String::from("north greenhouse")),
                LibertasMessageArgumentDecode::Object(42),
                LibertasMessageArgumentDecode::Boolean(true),
                LibertasMessageArgumentDecode::Signed(-9),
                LibertasMessageArgumentDecode::Unsigned(9),
                LibertasMessageArgumentDecode::Float(1.25),
                LibertasMessageArgumentDecode::Double(-2.5),
                LibertasMessageArgumentDecode::UnitSigned {
                    unit_type: String::from("celsius"),
                    value: -3,
                },
                LibertasMessageArgumentDecode::UnitUnsigned {
                    unit_type: String::from("byte"),
                    value: 1024,
                },
                LibertasMessageArgumentDecode::UnitFloat {
                    unit_type: String::from("meter-per-second"),
                    value: 1.5,
                },
                LibertasMessageArgumentDecode::UnitDouble {
                    unit_type: String::from("millimeter"),
                    value: 2.75,
                },
                LibertasMessageArgumentDecode::ResourceText(String::from("READY")),
                LibertasMessageArgumentDecode::Plural(3),
            ]),
        };
        let mut reencoded = Vec::new();
        decoded.avro_encode(&mut reencoded);

        assert_eq!(reencoded, expected);
    }
}
