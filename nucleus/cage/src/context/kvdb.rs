use super::*;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use wasmtime::{Caller, FuncType, Val, ValType};

pub(crate) fn init_rocksdb(path: impl AsRef<std::path::Path>) -> anyhow::Result<DB> {
    let avs_cf = ColumnFamilyDescriptor::new("avs", Options::default());
    let seq_cf = ColumnFamilyDescriptor::new("seq", Options::default());
    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    DB::open_cf_descriptors(&db_opts, path, vec![avs_cf, seq_cf]).map_err(|e| anyhow::anyhow!(e))
}

pub fn storage_put_db(db: &DB, key: &[u8], value: &[u8]) -> Result<(), String> {
    db.put_cf(db.cf_handle("avs").unwrap(), key, value)
        .map_err(|e| e.to_string())?;
    // println!(
    //     "put key={}, val={}",
    //     String::from_utf8_lossy(&key),
    //     String::from_utf8_lossy(&value)
    // );
    Ok(())
}
pub fn storage_get_db(db: &DB, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
    let value = db
        .get_cf(db.cf_handle("avs").unwrap(), key)
        .map_err(|e| e.to_string())?;
    // println!("get key={}, val={:?}", String::from_utf8_lossy(&key), value);
    Ok(value)
}

// pub fn storage_put_signature(store: &Store<Context>) -> FuncType {
//     FuncType::new(
//         store.engine(),
//         [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
//         [ValType::I32],
//     )
// }
// pub fn storage_get_signature(store: &Store<Context>) -> FuncType {
//     FuncType::new(
//         store.engine(),
//         [ValType::I32, ValType::I32, ValType::I32, ValType::I32], // k_ptr, k_len, v_ptr, v_len_ptr
//         [ValType::I32],                                           // status
//     )
// }
// pub fn storage_get(
//     mut caller: Caller<'_, Context>,
//     params: &[Val],
//     results: &mut [Val],
// ) -> anyhow::Result<()> {
//     println!("aaa");
//     let memory = caller
//         .get_export("memory")
//         .and_then(|e| e.into_memory())
//         .ok_or_else(|| anyhow::anyhow!("Failed to find memory export"))?;

//     let k_ptr = params[0].unwrap_i32() as u32;
//     let k_len = params[1].unwrap_i32() as u32;
//     let v_ptr = params[2].unwrap_i32() as u32;
//     let v_len_ptr = params[3].unwrap_i32() as u32;

//     // Read the key from WebAssembly memory
//     let mut key = vec![0u8; k_len as usize];
//     println!("get key={}, val={}", String::from_utf8_lossy(&key), "123");

//     memory.read(&caller, k_ptr as usize, &mut key)?;

//     let db = &caller.data().db;

//     if let Ok(value_option) = db.get_cf(db.cf_handle("avs").unwrap(), &key) {
//         if let Some(value) = value_option {
//             let value_len = value.len() as i32;

//             // Write the length of the value
//             memory.write(&mut caller, v_len_ptr as usize, &value_len.to_le_bytes())?;

//             // Write the value itself
//             memory.write(&mut caller, v_ptr as usize, &value)?;

//             results[0] = Val::I32(0); // Success status
//         } else {
//             // Key not found
//             results[0] = Val::I32(1); // Not found status
//         }
//     } else {
//         // An error occurred
//         results[0] = Val::I32(2); // Error status
//     }

//     Ok(())
// }
// pub fn storage_get_split(
//     mut caller: Caller<'_, Context>,
//     k_ptr: i32,
//     k_len: i32,
//     v_ptr: i32,
//     v_len_ptr: i32,
// ) -> i32 {
//     let memory = match caller.get_export("memory").and_then(|e| e.into_memory()) {
//         Some(mem) => mem,
//         None => return 3, // Error code for memory export not found
//     };

//     // Read the key from WebAssembly memory
//     let mut key = vec![0u8; k_len as usize];
//     if memory.read(&caller, k_ptr as usize, &mut key).is_err() {
//         return 4; // Error code for memory read failure
//     }
//     println!(
//         "get split key={}, val={}",
//         String::from_utf8_lossy(&key),
//         "123"
//     );

//     let db = &caller.data().db;

//     match db.get_cf(db.cf_handle("avs").unwrap(), &key) {
//         Ok(value_option) => {
//             if let Some(value) = value_option {
//                 // let value = [65, 66, 67];
//                 let value_len = value.len() as i32;

//                 // Write the length of the value
//                 if memory
//                     .write(&mut caller, v_len_ptr as usize, &value_len.to_le_bytes())
//                     .is_err()
//                 {
//                     return 5; // Error code for memory write failure
//                 }

//                 // Write the value itself
//                 if memory.write(&mut caller, v_ptr as usize, &value).is_err() {
//                     return 5; // Error code for memory write failure
//                 }

//                 0 // Success status
//             } else {
//                 1 // Not found status
//             }
//         }
//         Err(_) => 2, // Error status for database error
//     }
// }
// pub fn storage_get_split_len(
//     mut caller: Caller<'_, Context>,
//     k_ptr: i32,
//     k_len: i32,
//     v_len_ptr: i32,
// ) -> Result<Vec<u8>, i32> {
//     let memory = match caller.get_export("memory").and_then(|e| e.into_memory()) {
//         Some(mem) => mem,
//         None => return Err(3), // Error code for memory export not found
//     };

//     // Read the key from WebAssembly memory
//     let mut key = vec![0u8; k_len as usize];
//     if memory.read(&caller, k_ptr as usize, &mut key).is_err() {
//         return Err(4); // Error code for memory read failure
//     }
//     let db = &caller.data().db;

//     match db.get_cf(db.cf_handle("avs").unwrap(), &key) {
//         Ok(Some(value)) => {
//             let value_len = value.len() as i32;

//             // Write the length of the value
//             if memory
//                 .write(&mut caller, v_len_ptr as usize, &value_len.to_le_bytes())
//                 .is_err()
//             {
//                 return Err(5); // Error code for memory write failure
//             }

//             Ok(value)
//         }
//         Ok(None) => Err(1), // Not found status
//         Err(_) => Err(2),   // Error status for database error
//     }
// }
// pub fn storage_delete(context: &Context, key: &[u8]) -> Result<(), StorageError> {
//     context
//         .db
//         .delete_cf(context.db.cf_handle("avs").unwrap(), key)
//         .map_err(|e| StorageError(e.to_string()))?;
//     Ok(())
// }
