/// Libertas Rust SDK - Data API
/// 
/// Functions for standalone and indexed database operations with Avro encoding.

use alloc::vec::Vec;
use alloc::string::String;
use libertas_macros::{LibertasAvroEncode, LibertasAvroDecode};
use crate::*;
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

/// Data name with resource and arguments.
/// Used for identifying standalone and indexed data in the database.
/// printf style localized template shall be supplied in APp documents to render the resource name into natural language with arguments.
/// 
#[derive(LibertasAvroDecode)]
pub struct DataName {
    pub resource_name: String,
    pub arguments: Vec<NotificationArgumentDecode>,
}

#[derive(LibertasAvroEncode)]
struct DataNameInternal<'a> {
    resource_name: &'a str,
    arguments: &'a [NotificationArgument<'a>],
}

#[repr(C)]
struct DataInternal {
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

/// Returns all standalone data names.
/// 
/// # Returns
/// Vector of data names with resource and arguments.
pub fn libertas_data_get_names() -> Vec<DataName> {
    let result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_STADNALONE, OP_SYSTEM_DATABASE_GET_NAMES, core::ptr::null(), 0);
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

/// Removes standalone data by resource name and arguments.
pub fn libertas_data_remove(resource_name: &str, arguments: &[NotificationArgument]) {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
    let mut serialized = Vec::new();
    data_name.avro_encode(&mut serialized);
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_STADNALONE, OP_SYSTEM_DATABASE_REMOVE_DATA, 0, 0, serialized.as_ptr(), serialized.len());
}

/// Removes a piece of indexed data from the indexed database. The data to remove is identified by the resource name and arguments.
/// The resource name and arguments are encoded in Avro format and sent to the device to perform the removal. After this function is called, the specified indexed data will no longer be available in the indexed database.
/// 
/// # Arguments
/// * `resource_name` - The resource name of the indexed data to remove. This is a string that identifies the type of indexed data, such as "player_score_history" or "enemy_spawn_events".
/// * `arguments` - The arguments that further specify the indexed data to remove. The arguments are an array of `NotificationArgument` structs, which can include various types of data such as integers, strings, or booleans. The specific arguments needed to identify the indexed data will depend on how the indexed data was originally written to the database.
/// Note that indexed data is different from standalone data in that it is organized by an index value, which allows for efficient querying of data based on that index. When you remove indexed data using this function, you are removing all records that match the specified resource name and arguments, regardless of their index values.
///
pub fn libertas_data_remove_indexed(resource_name: &str, arguments: &[NotificationArgument]) {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
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
/// * `resource_name` - Resource identifier.
/// * `arguments` - Resource arguments.
/// 
/// # Returns
/// IndexedDataStat with handle, count, and index range. If `count` is 0, then `min_index` and `max_index` are undefined.
pub fn libertas_data_open_indexed(resource_name: &str, arguments: &[NotificationArgument]) -> IndexedDataStat {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
    let mut serialized = Vec::new();
    data_name.avro_encode(&mut serialized);
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

/// Writes data to standalone database.
/// 
/// # Arguments
/// * `resource_name` - Resource identifier.
/// * `arguments` - Resource arguments.
/// * `data` - Encodable data.
pub fn libertas_data_write(resource_name: &str, arguments: &[NotificationArgument], data: &dyn AvroEncode) {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
    let mut serialized = Vec::new();
    data_name.avro_encode(&mut serialized);
    let name_len = serialized.len();
    data.avro_encode(&mut serialized);
    let total_len = serialized.len();

    let name_ptr = serialized.as_ptr();
    let value_ptr = unsafe { name_ptr.add(name_len) };

    let data_internal = DataInternal {
        name: name_ptr,
        name_len: name_len,
        value: value_ptr,
        value_len: total_len - name_len,
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_STADNALONE, OP_SYSTEM_DATABASE_WRITE_DATA, 0, 0, &data_internal as *const DataInternal as *const u8, core::mem::size_of::<DataInternal>());
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

/// Reads standalone data.
/// 
/// # Arguments
/// * `resource_name` - Resource identifier.
/// * `arguments` - Resource arguments.
/// 
/// # Returns
/// Decoded data or None if not found.
pub fn libertas_data_read<T>(resource_name: &str, arguments: &[NotificationArgument]) -> Option<T> where T: AvroDecode {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
    let mut name = Vec::new();
    data_name.avro_encode(&mut name);
    let result = __libertas_device_read_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_STADNALONE, OP_SYSTEM_DATABASE_READ_DATA, name.as_ptr(), name.len());
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