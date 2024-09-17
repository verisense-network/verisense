use super::*;
use codec::Encode;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use vrs_core_sdk::{error::RuntimeError, CallResult, BUFFER_LEN, NO_MORE_DATA};
use wasmtime::{Caller, FuncType, Val, ValType};

pub(crate) fn init_rocksdb(path: impl AsRef<std::path::Path>) -> anyhow::Result<DB> {
    let avs_cf = ColumnFamilyDescriptor::new("avs", Options::default());
    let seq_cf = ColumnFamilyDescriptor::new("seq", Options::default());
    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    DB::open_cf_descriptors(&db_opts, path, vec![avs_cf, seq_cf]).map_err(|e| anyhow::anyhow!(e))
}

fn db_put(db: &DB, key: &[u8], value: &[u8]) -> Result<(), String> {
    db.put_cf(db.cf_handle("avs").unwrap(), key, value)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn db_get(db: &DB, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
    let value = db
        .get_cf(db.cf_handle("avs").unwrap(), key)
        .map_err(|e| e.to_string())?;
    Ok(value)
}

fn db_del(db: &DB, key: &[u8]) -> Result<(), String> {
    db.delete_cf(db.cf_handle("avs").unwrap(), key)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// the signature of this host function is:
///
/// ```
/// fn storage_put(key_ptr: *const u8, key_len: i32, value_ptr: *const u8, value_len: i32, return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn storage_put_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        engine,
        [
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
        ],
        [ValType::I32],
    )
}

/// the signature of this host function is:
///
/// ```
/// fn storage_put(key_ptr: *const u8, key_len: i32, value_ptr: *const u8, value_len: i32, return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn storage_put(
    mut caller: Caller<'_, Context>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()> {
    result[0] = Val::I32(NO_MORE_DATA);
    let r_ptr = params[4].unwrap_i32();
    if caller.data().is_get_method() {
        let return_value = CallResult::<()>::Err(RuntimeError::WriteIsNotAllowInGetMethod);
        let bytes = return_value.encode();
        assert!(bytes.len() <= BUFFER_LEN);
        Context::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
        return Ok(());
    }
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let v_ptr = params[2].unwrap_i32();
    let v_len = params[3].unwrap_i32();
    let key =
        Context::read_bytes_from_memory(&mut caller, k_ptr, k_len).expect("read from wasm failed");
    let val =
        Context::read_bytes_from_memory(&mut caller, v_ptr, v_len).expect("read from wasm failed");
    let return_value = if let Err(e) = db_put(&caller.data().db, &key, &val) {
        CallResult::<()>::Err(RuntimeError::KvStorageError(e))
    } else {
        CallResult::<()>::Ok(())
    };
    let bytes = return_value.encode();
    assert!(bytes.len() <= BUFFER_LEN);
    Context::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}

/// the signature of this host function is:
///
/// ```
/// fn storage_get(k_ptr: *const u8, k_len: i32, return_ptr: *mut u8, v_offset: i32) -> i32;
/// ```
pub(crate) fn storage_get_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        engine,
        [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    )
}

/// the signature of this host function is:
///
/// ```
/// fn storage_get(k_ptr: *const u8, k_len: i32, return_ptr: *mut u8, v_offset: i32) -> i32;
/// ```
/// the v_offset represents the offset of the value to read
/// first time:
///   Result: 1byte
///     Option: 1byte
///       vec_len(var_len prefix): 2bytes
///         bytes: max 64k-4
/// rest:
///   bytes: max 64k
pub fn storage_get(
    mut caller: Caller<'_, Context>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()> {
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let r_ptr = params[2].unwrap_i32();
    let v_offset = params[3].unwrap_i32();
    let key = Context::read_bytes_from_memory(&mut caller, k_ptr, k_len)
        .expect("can't read bytes from wasm");
    let db = &caller.data().db;
    let r = match db_get(db, &key) {
        Ok(value) => CallResult::<Option<Vec<u8>>>::Ok(value),
        Err(e) => CallResult::<Option<Vec<u8>>>::Err(RuntimeError::KvStorageError(e)),
    };
    let flag = Context::write_to_memory(&mut caller, r_ptr, r, Some(v_offset))?;
    result[0] = flag;
    Ok(())
}

/// the signature of this host function is:
///
/// ```
/// fn storage_del(key_ptr: *const u8, key_len: i32, return_ptr: *mut u8) -> i32;
/// ```
pub(crate) fn storage_del_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        engine,
        [ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    )
}

/// the signature of this host function is:
///
/// ```
/// fn storage_del(key_ptr: *const u8, key_len: i32, return_ptr: *mut u8) -> i32;
/// ```
pub fn storage_delete(
    mut caller: Caller<'_, Context>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()> {
    result[0] = Val::I32(NO_MORE_DATA);
    let r_ptr = params[2].unwrap_i32();
    if caller.data().is_get_method() {
        let return_value = CallResult::<()>::Err(RuntimeError::WriteIsNotAllowInGetMethod);
        let bytes = return_value.encode();
        assert!(bytes.len() <= BUFFER_LEN);
        Context::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
        return Ok(());
    }
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let key = Context::read_bytes_from_memory(&mut caller, k_ptr, k_len)
        .expect("can't read bytes from wasm");
    let return_value = if let Err(e) = db_del(&caller.data().db, &key) {
        CallResult::<()>::Err(RuntimeError::KvStorageError(e))
    } else {
        CallResult::<()>::Ok(())
    };
    let bytes = return_value.encode();
    assert!(bytes.len() <= BUFFER_LEN);
    Context::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}
