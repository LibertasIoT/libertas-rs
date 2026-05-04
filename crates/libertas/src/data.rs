use alloc::vec::Vec;
use alloc::string::String;
use libertas_macros::{LibertasAvroEncode, LibertasAvroDecode};
use crate::*;
use notification::*;

#[repr(u8)]
pub enum IndexDirection {
    Above,
    Below,
}

pub struct IndexedData<T> where T: AvroDecode {
    pub index: i64,
    pub data: T,
}

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

#[repr(C)]
pub struct IndexedDataStat {
    handle: LibertasDataStore,
    count: u64,
    min_index: i64,
    max_index: i64,
}

pub fn libertas_get_data_names() -> Vec<DataName> {
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

pub fn libertas_get_indexed_data_names() -> Vec<DataName> {
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

pub fn libertas_remove_data(resource_name: &str, arguments: &[NotificationArgument]) {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
    let mut serialized = Vec::new();
    data_name.avro_encode(&mut serialized);
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_STADNALONE, OP_SYSTEM_DATABASE_REMOVE_DATA, 0, 0, serialized.as_ptr(), serialized.len());
}

pub fn libertas_remove_indexed_data(resource_name: &str, arguments: &[NotificationArgument]) {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
    let mut serialized = Vec::new();
    data_name.avro_encode(&mut serialized);
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_REMOVE_DATA, 0, 0, serialized.as_ptr(), serialized.len());
}

pub fn libertas_remove_indexed_record(db: LibertasDataStore, index: i64) {
    let req = DataWriteIndexedReq {
        db,
        index,
        value: core::ptr::null(),
        value_len: 0,
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_REMOVE_RECORD, 0, 0, &req as *const DataWriteIndexedReq as *const u8, core::mem::size_of::<DataWriteIndexedReq>());
}

pub fn libertas_open_indexed_data(resource_name: &str, arguments: &[NotificationArgument]) -> IndexedDataStat {
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

pub fn libertas_write_data(resource_name: &str, arguments: &[NotificationArgument], data: &dyn AvroEncode) {
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

pub fn libertas_write_indexed_record(db: LibertasDataStore, index: i64, data: &dyn AvroEncode) {
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

pub fn libertas_read_data<T>(resource_name: &str, arguments: &[NotificationArgument]) -> Option<T> where T: AvroDecode {
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

pub fn libertas_read_indexed_data_range<T>(db: LibertasDataStore, index: i32, direction: IndexDirection, max_n: usize, results: &mut Vec<IndexedData<T>>) where T: AvroDecode {
    let req = DataReadIndexedReq {
        db,
        index: index as i64,
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

pub fn libertas_read_indexed_data<T>(db: LibertasDataStore, index: i32) -> Option<IndexedData<T>> where T: AvroDecode {
    let req = DataReadIndexedReq {
        db,
        index: index as i64,
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