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

/// A struct containing statistics about a piece of indexed data in the indexed database, including the handle for the indexed data, the number of records, and the range of index values. This struct is returned when opening indexed data using `libertas_open_indexed_data`, and it provides important information about the indexed data that can be used for subsequent read or write operations. The `handle` field is a `LibertasDataStore` value that serves as a reference to the opened indexed data and is used in other functions to read or write records associated with that indexed data. The `count` field indicates how many records are currently stored in the indexed database for that piece of indexed data. The `min_index` and `max_index` fields indicate the range of index values for the records stored in the indexed database for that piece of indexed data, which can be useful for understanding the distribution of index values and for performing range queries on the indexed data. 
#[repr(C)]
pub struct IndexedDataStat {
    /// The handle for the opened indexed data, which is used for subsequent read or write operations on that indexed data. This handle is obtained when opening indexed data using `libertas_open_indexed_data` and is required for functions that read or write records associated with that indexed data, such as `libertas_read_indexed_data`, `libertas_read_indexed_data_range`, and `libertas_write_indexed_record`. The handle serves as a reference to the specific piece of indexed data in the indexed database that you have opened, allowing you to perform operations on it without needing to specify the resource name and arguments again.
    handle: LibertasDataStore,
    /// The number of records currently stored in the indexed database for the opened indexed data. This count indicates how many records are associated with the specific piece of indexed data that you have opened using `libertas_open_indexed_data`. The count can be useful for understanding how much data is stored for that indexed data and for making decisions about how to read or write records based on the number of existing records. For example, if the count is zero, it means there are no records currently stored for that indexed data, and you may want to write some records before attempting to read. Conversely, if the count is very high, you may want to read a range of records instead of trying to read them all at once.
    count: u64,
    /// The minimum index value for the records stored in the indexed database for the opened indexed data. This value indicates the lowest index value among the records that are currently stored for that piece of indexed data. The minimum index can be useful for understanding the range of index values and for performing range queries on the indexed data. For example, if you want to read records starting from a certain index value, knowing the minimum index can help you determine where to start reading from. Note that if the `count` of the indexed data is zero, this `min_index` value should be ignored, as there are no records in the indexed database for that indexed data.
    min_index: i64,
    /// The maximum index value for the records stored in the indexed database for the opened indexed data. This value indicates the highest index value among the records that are currently stored for that piece of indexed data. The maximum index can be useful for understanding the range of index values and for performing range queries on the indexed data. For example, if you want to read records up to a certain index value, knowing the maximum index can help you determine where to stop reading. Note that if the `count` of the indexed data is zero, this `max_index` value should be ignored, as there are no records in the indexed database for that indexed data.
    max_index: i64,
}

/// Gets a list of all data names in the standalone database. Each name includes the resource name and arguments. 
/// This can be used to discover what data is available in the database and to find the arguments needed to read specific data.
/// Note that indexed data names are not included in this list, use `libertas_get_indexed_data_names` to get those.
/// The returned data names are decoded from Avro format and returned as a vector of `DataName` structs.
/// 
/// # Returns
/// A vector of `DataName` structs, each containing the resource name and arguments for a piece of data in the standalone database.
/// 
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

/// Gets a list of all indexed data names in the indexed database. Each name includes the resource name and arguments.
/// This can be used to discover what indexed data is available in the database and to find the arguments needed to read specific indexed data.
/// Note that standalone data names are not included in this list, use `libertas_get_data_names` to get those.
/// The returned data names are decoded from Avro format and returned as a vector of `DataName` structs.
/// 
/// # Returns
/// A vector of `DataName` structs, each containing the resource name and arguments for a piece of indexed data in the indexed database.
///
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

/// Removes a piece of data from the standalone database. The data to remove is identified by the resource name and arguments.
/// The resource name and arguments are encoded in Avro format and sent to the device to perform the removal. After this function is called, the specified data will no longer be available in the standalone
/// 
/// # Arguments
/// * `resource_name` - The resource name of the data to remove. This is a string that identifies the type of data, such as "player_position" or "game_score".
/// * `arguments` - The arguments that further specify the data to remove. The arguments are an array of `NotificationArgument` structs, which can include various types of data such as integers, strings, or booleans. The specific arguments needed to identify the data will depend on how the data was originally written to the database.
/// 
pub fn libertas_remove_data(resource_name: &str, arguments: &[NotificationArgument]) {
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
pub fn libertas_remove_indexed_data(resource_name: &str, arguments: &[NotificationArgument]) {
    let data_name = DataNameInternal {
        resource_name,
        arguments,
    };
    let mut serialized = Vec::new();
    data_name.avro_encode(&mut serialized);
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_REMOVE_DATA, 0, 0, serialized.as_ptr(), serialized.len());
}

/// Removes a range of indexed records from the indexed database. The records to remove are identified by the database handle and index range.
/// The database handle is obtained when opening indexed data using `libertas_open_indexed_data`, and the index range specifies which records to remove based on their index values. After this function is called, all indexed records in the specified database that have index values within the specified range will be removed from the indexed database.
/// 
/// # Arguments
/// * `db` - The handle of the indexed database from which to remove records. This handle is obtained by calling `libertas_open_indexed_data` with the appropriate resource name and arguments that identify the indexed data.
/// * `index_lo` - The lower bound of the index range for the records to remove. All records with index values greater than or equal to this value will be considered for removal.
/// * `index_hi` - The upper bound of the index range for the records to remove. All records with index values less than or equal to this value will be considered for removal.
/// Note that the index range is inclusive, meaning that records with index values equal to `index_lo` or `index_hi` will also be removed. If you want to remove all records with index values greater than a certain value, you can set `index_hi` to a very large number. Conversely, if you want to remove all records with index values less than a certain value, you can set `index_lo` to a very small number. Be cautious when using this function, as it can potentially remove a large number of records if the index range is not specified carefully.
///
pub fn libertas_remove_indexed_records(db: LibertasDataStore, index_lo: i64, index_hi: i64) {
    let req: DataDeleteIndexedRecordsReq = DataDeleteIndexedRecordsReq {
        db,
        index_lo,
        index_hi
    };
    __libertas_device_send_raw(PROTOCOL_LIBERTAS, DEVICE_SYSTEM_DATABASE_INDEXED, OP_SYSTEM_DATABASE_REMOVE_RECORD, 0, 0, &req as *const DataDeleteIndexedRecordsReq as *const u8, core::mem::size_of::<DataDeleteIndexedRecordsReq>());
}

/// Opens a handle to a piece of indexed data in the indexed database, identified by the resource name and arguments. This handle can then be used to read or write indexed records associated with that data.
/// The resource name and arguments are encoded in Avro format and sent to the device to perform the opening of the indexed data. If the operation is successful, a handle to the indexed data is returned, along with statistics about the indexed data such as the number of records and the range of index values. This function does not actually read any indexed records, it only opens a handle to the indexed data so that you can perform subsequent read or write operations on it. If you want to read indexed records, you can use the `libertas_read_indexed_data` or `libertas_read_indexed_data_range` functions with the handle obtained from this function. If you want to write indexed records, you can use the `libertas_write_indexed_record` function with the handle obtained from this function.
/// 
/// # Arguments
/// * `resource_name` - The resource name of the indexed data to open. This is a string that identifies the type of indexed data, such as "player_score_history" or "enemy_spawn_events".
/// * `arguments` - The arguments that further specify the indexed data to open. The arguments are an array of `NotificationArgument` structs, which can include various types of data such as integers, strings, or booleans. The specific arguments needed to identify the indexed data will depend on how the indexed data was originally written to the database.
/// 
/// # Returns
/// An `IndexedDataStat` struct containing the handle for the opened indexed data and statistics about the indexed data such as the number of records and the range of index values. The handle can be used for subsequent read or write operations on the indexed data. If the operation fails, this function will panic with an error message indicating that the indexed data could not be opened. Be sure to handle this function with care, as it will panic on failure, so you may want to ensure that the resource name and arguments you provide are correct and that the indexed data you are trying to open actually exists in the indexed database before calling this function.
/// 
/// Note that if the `count` of the returned `IndexedDataStat` is zero, the `min_index` and `max_index` shall be ignored.
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

/// Writes a piece of data to the standalone database, identified by the resource name and arguments. The data is encoded in Avro format and sent to the device to perform the write operation. After this function is called, the specified data will be available in the standalone database and can be read using `libertas_read_data` with the same resource name and arguments.
/// 
/// # Arguments
/// * `resource_name` - The resource name of the data to write. This is a string that identifies the type of data, such as "player_position" or "game_score".
/// * `arguments` - The arguments that further specify the data to write. The arguments are an array of `NotificationArgument` structs, which can include various types of data such as integers, strings, or booleans. The specific arguments needed to identify the data will depend on how you want to organize your data in the database. For example, if you are writing player position data, you might use the player ID as an argument to distinguish between different players' positions.
/// * `data` - The actual data to write, which can be any type that implements the `AvroEncode` trait. This data will be encoded in Avro format and stored in the database under the specified resource name and arguments. When you want to read this data back from the database, you will need to use the same resource name and arguments to identify it, and you will need to decode it using the appropriate type that implements `AvroDecode`.
/// Note that if you write data with the same resource name and arguments as existing data in the standalone database using this function, it will overwrite the existing data with the new data you are writing. The standalone database does not support multiple records with the same resource name and arguments, so each unique combination of resource name and arguments can only have one piece of data associated with it. If you want to store multiple records with the same resource name and arguments, you should consider using indexed data in the indexed database instead, which allows for multiple records distinguished by their index values.
///
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

/// Writes a piece of indexed data to the indexed database, identified by the database handle and index value. The data is encoded in Avro format and sent to the device to perform the write operation. After this function is called, the specified indexed data will be available in the indexed database under the given index value and can be read using `libertas_read_indexed_data` with the same database handle and index value.
/// 
/// # Arguments
/// * `db` - The handle of the indexed database to which to write the record. This handle is obtained by calling `libertas_open_indexed_data` with the appropriate resource name and arguments that identify the indexed data.
/// * `index` - The index value for the record to write. This is a 64-bit integer that serves as the index for the record within the indexed database. The specific meaning of the index value is up to you and how you want to organize your indexed data. For example, if you are writing player score history data, you might use a timestamp as the index value to keep track of when each score was recorded. When you read indexed data from the database, you can query for records based on their index values, so it's important to choose index values that make sense for how you want to access your data.
/// * `data` - The actual data to write, which can be any type that implements the `AvroEncode` trait. This data will be encoded in Avro format and stored in the indexed database under the specified index value. When you want to read this indexed data back from the database, you will need to use the same database handle and index value to identify it, and you will need to decode it using the appropriate type that implements `AvroDecode`.
/// Note that the indexed database allows for multiple records with the same resource name and arguments, distinguished by their index values. This means that you can write multiple records to the indexed database with the same resource name and arguments but different index values, and they will all be stored separately. When you read indexed data, you can query for specific index values or ranges of index values to retrieve the records you are interested in. This makes the indexed database a good choice for storing timer series data or any data where you want to keep a history of changes over time, as you can use the index values to track when each record was written and query for records within specific time ranges or other index-based criteria.
///
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

/// Reads a piece of data from the standalone database, identified by the resource name and arguments. The resource name and arguments are encoded in Avro format and sent to the device to perform the read operation. If the specified data is found in the standalone database, it is returned decoded into the specified type `T` that implements `AvroDecode`. If the specified data is not found, this function returns `None`.
/// 
/// # Arguments
/// * `resource_name` - The resource name of the data to read. This is a string that identifies the type of data, such as "player_position" or "game_score".
/// * `arguments` - The arguments that further specify the data to read. The arguments are an array of `NotificationArgument` structs, which can include various types of data such as integers, strings, or booleans. The specific arguments needed to identify the data will depend on how the data was originally written to the database. You need to provide the same resource name and arguments that were used when the data was written to the database in order to successfully read it back.
/// * `T` - The type into which to decode the data. This can be any type that implements the `AvroDecode` trait. When you read data from the database, it is stored in Avro format, so you need to specify the type that matches the structure of the data you are trying to read and that implements `AvroDecode` so that the function can decode the Avro data into that type before returning it. If the data in the database does not match the structure of the specified type `T`, the decoding will fail and this function will return `None`.
/// Note that the standalone database does not support multiple records with the same resource name and arguments, so this function will return at most one piece of data for a given combination of resource name and arguments. If you need to store and read multiple records with the same resource name and arguments, you should consider using indexed data in the indexed database instead, which allows for multiple records distinguished by their index values. When using this function, be sure to provide the correct resource name and arguments that match the data you want to read, and ensure that the type `T` you specify for decoding matches the structure of the data in the database, otherwise you may not be able to successfully read the data back from the database.
///
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

/// Reads a range of indexed data records from the indexed database, identified by the database handle and index range. The records are returned as a vector of `IndexedData<T>` structs, where each struct contains the index value and the decoded data of type `T` that implements `AvroDecode`. The index range is specified by the `index` and `direction` parameters, which determine where to start reading from and in which direction to read. The `max_n` parameter specifies the maximum number of records to read. If the read operation is successful, the specified range of indexed data records will be returned in the `results` vector. If the read operation fails, the `results` vector will be left unchanged and the function will return without modifying it.
/// 
/// # Arguments
/// * `db` - The handle of the indexed database from which to read records. This handle is obtained by calling `libertas_open_indexed_data` with the appropriate resource name and arguments that identify the indexed data.
/// * `index` - The index value that serves as the starting point for reading records. The specific meaning of the index value is up to you and how you want to organize your indexed data. For example, if you are reading player score history data, you might use a timestamp as the index value to specify where in the score history you want to start reading from. The interpretation of the index value will depend on how you are using it when writing indexed records to the database.
/// * `direction` - The direction in which to read records from the starting index. This is an `IndexDirection` enum that can be either `Above` or `Below`. If `Above` is specified, the function will read records with index values greater than or equal to the specified starting index. If `Below` is specified, the function will read records with index values less than or equal to the specified starting index. The direction parameter allows you to control whether you want to read forward or backward from the starting index, which can be useful for different use cases. For example, if you want to read the most recent records up to a certain point, you might use `Below` to read backward from a high index value. Conversely, if you want to read upcoming records starting from a certain point, you might use `Above` to read forward from a low index value.
/// * `max_n` - The maximum number of records to read. This is a usize value that limits the number of indexed data records that will be read and returned in the `results` vector. If there are more records available in the specified direction beyond the starting index, only up to `max_n` records will be read and returned. This parameter allows you to control how much data is read and can help prevent reading too many records at once, which could be inefficient or overwhelming depending on the size of the indexed data. If you set `max_n` to a large number, the function will attempt to read as many records as possible in the specified direction until it reaches the end of the indexed data or the maximum number of records is read.
/// * `results` - A mutable reference to a vector where the read indexed data records will be stored. Each record will be an `IndexedData<T>` struct containing the index value and the decoded data of type `T`. If the read operation is successful, the specified range of indexed data records will be appended to this vector. If the read operation fails, this vector will be left unchanged. You should provide an empty vector when calling this function, and after the function returns, it will contain the indexed data records that were read from the database. You can then iterate over this vector to access the individual records and their data.
/// Note that the indexed data records returned by this function are decoded from Avro format into the specified type `T`, so you need to ensure that the type `T` you specify for decoding matches the structure of the data in the indexed database, otherwise the decoding will fail and you may not get the expected results. When using this function, be sure to provide the correct database handle, starting index, direction, and maximum number of records to read based on how your indexed data is organized and how you want to access it. This function is a powerful way to read a range of indexed data records based on their index values, which can be very useful for time series data or any data where you want to read records in a specific order based on their index values.
/// 
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

/// Reads a piece of indexed data from the indexed database, identified by the database handle and index value. If a record with the specified index value is found in the indexed database, it is returned as an `IndexedData<T>` struct containing the index value and the decoded data of type `T` that implements `AvroDecode`. If no record with the specified index value is found, this function returns `None`.
/// 
/// # Arguments
/// * `db` - The handle of the indexed database from which to read the record. This handle is obtained by calling `libertas_open_indexed_data` with the appropriate resource name and arguments that identify the indexed data.
/// * `index` - The index value of the record to read. This is a 64-bit integer that serves as the index for the record within the indexed database. The specific meaning of the index value is up to you and how you want to organize your indexed data. For example, if you are reading player score history data, you might use a timestamp as the index value to specify which score record you want to read. When you read indexed data from the database, you can query for specific index values or ranges of index values to retrieve the records you are interested in. This function allows you to read a single record based on its index value, so you need to provide the exact index value of the record you want to read. If there is a record with that index value in the indexed database, it will be returned. If there is no record with that index value, this function will return `None`.
/// * `T` - The type into which to decode the data. This can be any type that implements the `AvroDecode` trait. When you read indexed data from the database, it is stored in Avro format, so you need to specify the type that matches the structure of the data you are trying to read and that implements `AvroDecode` so that the function can decode the Avro data into that type before returning it. If the data in the database does not match the structure of the specified type `T`, the decoding will fail and this function will return `None`.
/// Note that the indexed database allows for multiple records with the same resource name and arguments, distinguished by their index values. This function reads a single record based on its index value, so you need to provide the correct database handle and index value that match the record you want to read. If you want to read multiple records based on a starting index and direction, you can use the `libertas_read_indexed_data_range` function instead, which allows you to read a range of indexed data records based on their index values. When using this function, be sure to provide the correct database handle and index value based on how your indexed data is organized and how you want to access it. This function is a convenient way to read a single piece of indexed data based on its index value, which can be very useful for time series data or any data where you want to read records based on their index values.
/// 
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