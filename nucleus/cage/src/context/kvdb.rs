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

// pub fn storage_put(
//     mut caller: Caller<'_, Context>,
//     params: &[Val],
//     results: &mut [Val],
// ) -> anyhow::Result<()> {
//     if caller.data().is_get_method() {
//         results[0] = Val::I32(1); // Error code for not allowed in GET method
//         return Ok(());
//     }
//     let mem = Context::wasm_mem(&mut caller).map_err(|e| anyhow::anyhow!(e))?;
//     let k_ptr = params[0].unwrap_i32() as u32;
//     let k_len = params[1].unwrap_i32() as u32;
//     let v_ptr = params[2].unwrap_i32() as u32;
//     let v_len = params[3].unwrap_i32() as u32;

//     // Boundary check
//     if (k_ptr as u64 + k_len as u64) > mem.data_size(&caller) as u64
//         || (v_ptr as u64 + v_len as u64) > mem.data_size(&caller) as u64
//     {
//         results[0] = Val::I32(2); // Error code for out of bounds
//         return Ok(());
//     }

//     let key = mem.data(&caller)[k_ptr as usize..(k_ptr + k_len) as usize].to_vec();
//     let val = mem.data(&caller)[v_ptr as usize..(v_ptr + v_len) as usize].to_vec();

//     println!(
//         "Storing key={}, val={}",
//         String::from_utf8_lossy(&key),
//         String::from_utf8_lossy(&val)
//     );

//     let db = &caller.data().db;
//     if let Err(e) = db.put_cf(db.cf_handle("avs").unwrap(), &key, &val) {
//         log::error!("Database error: {}", e);
//         results[0] = Val::I32(3); // Error code for database error
//         return Ok(());
//     }

//     results[0] = Val::I32(0); // Success
//     Ok(())
// }
pub fn storage_put_split(
    mut caller: Caller<'_, Context>,
    k_ptr: i32,
    k_len: i32,
    v_ptr: i32,
    v_len: i32,
) -> i32 {
    if caller.data().is_get_method() {
        return 1; // Error code for not allowed in GET method
    }
    let mem = match Context::wasm_mem(&mut caller) {
        Ok(mem) => mem,
        Err(_) => return 4, // Error code for memory access failure
    };

    // Boundary check
    if (k_ptr as u64 + k_len as u64) > mem.data_size(&caller) as u64
        || (v_ptr as u64 + v_len as u64) > mem.data_size(&caller) as u64
    {
        return 2; // Error code for out of bounds
    }

    let key = mem.data(&caller)[k_ptr as usize..(k_ptr + k_len) as usize].to_vec();
    let val = mem.data(&caller)[v_ptr as usize..(v_ptr + v_len) as usize].to_vec();

    println!(
        "Storing split key={}, val={}",
        String::from_utf8_lossy(&key),
        String::from_utf8_lossy(&val)
    );

    let db = &caller.data().db;
    if let Err(e) = db.put_cf(db.cf_handle("avs").unwrap(), &key, &val) {
        log::error!("Database error: {}", e);
        return 3; // Error code for database error
    }

    0 // Success
}

pub fn storage_put_signature(store: &Store<Context>) -> FuncType {
    FuncType::new(
        store.engine(),
        [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    )
}
pub fn storage_get_signature(store: &Store<Context>) -> FuncType {
    FuncType::new(
        store.engine(),
        [ValType::I32, ValType::I32, ValType::I32, ValType::I32], // k_ptr, k_len, v_ptr, v_len_ptr
        [ValType::I32],                                           // status
    )
}
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
pub fn storage_get_split(
    mut caller: Caller<'_, Context>,
    k_ptr: i32,
    k_len: i32,
    v_ptr: i32,
    v_len_ptr: i32,
) -> i32 {
    let memory = match caller.get_export("memory").and_then(|e| e.into_memory()) {
        Some(mem) => mem,
        None => return 3, // Error code for memory export not found
    };

    // Read the key from WebAssembly memory
    let mut key = vec![0u8; k_len as usize];
    if memory.read(&caller, k_ptr as usize, &mut key).is_err() {
        return 4; // Error code for memory read failure
    }
    println!(
        "get split key={}, val={}",
        String::from_utf8_lossy(&key),
        "123"
    );

    let db = &caller.data().db;

    match db.get_cf(db.cf_handle("avs").unwrap(), &key) {
        Ok(value_option) => {
            if let Some(value) = value_option {
                // let value = [65, 66, 67];
                let value_len = value.len() as i32;

                // Write the length of the value
                if memory
                    .write(&mut caller, v_len_ptr as usize, &value_len.to_le_bytes())
                    .is_err()
                {
                    return 5; // Error code for memory write failure
                }

                // Write the value itself
                if memory.write(&mut caller, v_ptr as usize, &value).is_err() {
                    return 5; // Error code for memory write failure
                }

                0 // Success status
            } else {
                1 // Not found status
            }
        }
        Err(_) => 2, // Error status for database error
    }
}

// pub fn storage_delete(context: &Context, key: &[u8]) -> Result<(), StorageError> {
//     context
//         .db
//         .delete_cf(context.db.cf_handle("avs").unwrap(), key)
//         .map_err(|e| StorageError(e.to_string()))?;
//     Ok(())
// }
