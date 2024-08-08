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

pub fn storage_put(
    mut caller: Caller<'_, Context>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()> {
    let mem = Context::wasm_mem(&mut caller).map_err(|e| anyhow::anyhow!(e))?;
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let v_ptr = params[2].unwrap_i32();
    let v_len = params[3].unwrap_i32();
    let mut key = vec![0u8; k_len as u32 as usize];
    mem.read(&caller, k_ptr as u32 as usize, &mut key)
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut val = vec![0u8; v_len as u32 as usize];
    mem.read(&caller, v_ptr as u32 as usize, &mut val)
        .map_err(|e| anyhow::anyhow!(e))?;
    let db = &caller.data().db;
    db.put_cf(db.cf_handle("avs").unwrap(), &key, &val)
        .map_err(|e| anyhow::anyhow!(e))?;
    result[0] = Val::I32(0);
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
