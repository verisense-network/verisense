use std::cell::Cell;

use super::*;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use wasmtime::{Caller, FuncType, Val, ValType};

pub(crate) fn init_rocksdb() -> anyhow::Result<DB> {
    // TODO
    let path = "/tmp";
    let avs_cf = ColumnFamilyDescriptor::new("avs", Options::default());
    let seq_cf = ColumnFamilyDescriptor::new("seq", Options::default());
    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    DB::open_cf_descriptors(&db_opts, path, vec![avs_cf, seq_cf]).map_err(|e| anyhow::anyhow!(e))
}
pub fn storage_put(
    mut caller: Caller<'_, Context>,
    params: &[Val],
    results: &mut [Val],
) -> anyhow::Result<()> {
    if caller.data().is_get_method() {
        println!("storage_put called in GET method");
        results[0] = Val::I32(1); // Error code for method not allowed
        return Ok(());
    }

    let mem = Context::wasm_mem(&mut caller).map_err(|e| anyhow::anyhow!(e))?;
    let k_ptr = params[0].unwrap_i32() as u32;
    let k_len = params[1].unwrap_i32() as u32;
    let v_ptr = params[2].unwrap_i32() as u32;
    let v_len = params[3].unwrap_i32() as u32;

    // Boundary check
    if (k_ptr as u64 + k_len as u64) > mem.data_size(&caller) as u64
        || (v_ptr as u64 + v_len as u64) > mem.data_size(&caller) as u64
    {
        results[0] = Val::I32(2); // Error code for out of bounds
        return Ok(());
    }

    let key = mem.data(&caller)[k_ptr as usize..(k_ptr + k_len) as usize].to_vec();
    let val = mem.data(&caller)[v_ptr as usize..(v_ptr + v_len) as usize].to_vec();

    log::debug!(
        "Storing key={}, val={}",
        String::from_utf8_lossy(&key),
        String::from_utf8_lossy(&val)
    );

    let db = &caller.data().db;
    if let Err(e) = db.put_cf(db.cf_handle("avs").unwrap(), &key, &val) {
        log::error!("Database error: {}", e);
        results[0] = Val::I32(3); // Error code for database error
        return Ok(());
    }

    results[0] = Val::I32(0); // Success
    Ok(())
}

pub fn storage_put_signature(store: &Store<Context>) -> FuncType {
    FuncType::new(
        store.engine(),
        [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    )
}

// pub fn storage_get(context: &Context, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
//     let value = context
//         .db
//         .get_cf(context.db.cf_handle("avs").unwrap(), key)
//         .map_err(|e| StorageError(e.to_string()))?;
//     Ok(value)
// }

// pub fn storage_delete(context: &Context, key: &[u8]) -> Result<(), StorageError> {
//     context
//         .db
//         .delete_cf(context.db.cf_handle("avs").unwrap(), key)
//         .map_err(|e| StorageError(e.to_string()))?;
//     Ok(())
// }
