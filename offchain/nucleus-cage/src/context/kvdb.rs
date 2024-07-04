use super::*;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};

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
    k_ptr: i32,
    k_len: i32,
    v_ptr: i32,
    v_len: i32,
) -> anyhow::Result<()> {
    let mem = Context::wasm_mem(&mut caller).map_err(|e| anyhow::anyhow!(e))?;
    let mut key = vec![0u8; k_len as u32 as usize];
    mem.read(&caller, k_ptr as u32 as usize, &mut key)
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut val = vec![0u8; v_len as u32 as usize];
    mem.read(&caller, v_ptr as u32 as usize, &mut val)
        .map_err(|e| anyhow::anyhow!(e))?;
    let db = &caller.data().db;
    db.put_cf(db.cf_handle("avs").unwrap(), &key, &val)
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
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
