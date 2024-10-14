use crate::{
    mem,
    runtime::{ComponentProvider, ContextAware},
    state::NucleusState,
    Runtime,
};
use codec::Encode;
use rocksdb::{Direction as IterDirection, IteratorMode};
use vrs_core_sdk::{error::RuntimeError, storage::Direction, CallResult, BUFFER_LEN, NO_MORE_DATA};
use wasmtime::{Caller, Engine, FuncType, Val, ValType};

impl ComponentProvider<NucleusState> for Runtime {
    fn get_component(&self) -> std::sync::Arc<NucleusState> {
        self.state.clone()
    }
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
pub(crate) fn storage_put<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ContextAware + ComponentProvider<NucleusState>,
{
    result[0] = Val::I32(NO_MORE_DATA);
    let r_ptr = params[4].unwrap_i32();
    if caller.data().read_only() {
        let return_value = CallResult::<()>::Err(RuntimeError::WriteIsNotAllowInGetMethod);
        let bytes = return_value.encode();
        assert!(bytes.len() <= BUFFER_LEN);
        mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
        return Ok(());
    }
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let v_ptr = params[2].unwrap_i32();
    let v_len = params[3].unwrap_i32();
    let key =
        mem::read_bytes_from_memory(&mut caller, k_ptr, k_len).expect("read from wasm failed");
    let val =
        mem::read_bytes_from_memory(&mut caller, v_ptr, v_len).expect("read from wasm failed");
    let db = caller.data().get_component();
    let return_value = if let Err(e) = db.put_user_data(&key, &val) {
        CallResult::<()>::Err(RuntimeError::KvStorageError(e.to_string()))
    } else {
        CallResult::<()>::Ok(())
    };
    let bytes = return_value.encode();
    assert!(bytes.len() <= BUFFER_LEN);
    mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
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
pub fn storage_get<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ComponentProvider<NucleusState>,
{
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let r_ptr = params[2].unwrap_i32();
    let v_offset = params[3].unwrap_i32();
    let key =
        mem::read_bytes_from_memory(&mut caller, k_ptr, k_len).expect("can't read bytes from wasm");
    let db = caller.data().get_component();
    let r = match db.get_user_data(&key) {
        Ok(value) => CallResult::<Option<Vec<u8>>>::Ok(value),
        Err(e) => CallResult::<Option<Vec<u8>>>::Err(RuntimeError::KvStorageError(e.to_string())),
    };
    let flag = mem::write_to_memory(&mut caller, r_ptr, r, Some(v_offset))?;
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
pub fn storage_del<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ComponentProvider<NucleusState> + ContextAware,
{
    result[0] = Val::I32(NO_MORE_DATA);
    let r_ptr = params[2].unwrap_i32();
    if caller.data().read_only() {
        let return_value = CallResult::<()>::Err(RuntimeError::WriteIsNotAllowInGetMethod);
        let bytes = return_value.encode();
        assert!(bytes.len() <= BUFFER_LEN);
        mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
        return Ok(());
    }
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let key =
        mem::read_bytes_from_memory(&mut caller, k_ptr, k_len).expect("can't read bytes from wasm");
    let db = caller.data().get_component();
    let return_value = if let Err(e) = db.del_user_data(&key) {
        CallResult::<()>::Err(RuntimeError::KvStorageError(e.to_string()))
    } else {
        CallResult::<()>::Ok(())
    };
    let bytes = return_value.encode();
    assert!(bytes.len() <= BUFFER_LEN);
    mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}

pub(crate) fn storage_get_prefix_signature(engine: &Engine) -> FuncType {
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
/// fn storage_get_prefix(key_ptr: *const u8, key_len: i32, direction: i32, return_ptr: *mut u8, v_offset: i32) -> i32;
/// ```
pub fn storage_get_prefix<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ComponentProvider<NucleusState>,
{
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let direction: Direction = params[2].unwrap_i32().into();
    let r_ptr = params[3].unwrap_i32();
    let v_offset = params[4].unwrap_i32();
    let key =
        mem::read_bytes_from_memory(&mut caller, k_ptr, k_len).expect("can't read bytes from wasm");
    let db = caller.data().get_component();
    let direction = match direction {
        Direction::Forward => IterDirection::Forward,
        Direction::Reverse => IterDirection::Reverse,
    };
    db.apply_on_user_data(IteratorMode::From(&key, direction), |mut iter| {
        let next = iter
            .next()
            .transpose()
            .map_err(|e| RuntimeError::KvStorageError(e.to_string()))
            .map(|kv| kv.map(|(k, v)| (k.to_vec(), v.to_vec())));
        let flag = mem::write_to_memory(&mut caller, r_ptr, next, Some(v_offset))?;
        result[0] = flag;
        Ok(())
    })
}

/// the signature of this host function is:
///
/// ```
/// fn storage_get_range(
///     k_ptr: *const u8,
///     k_len: i32,
///     direction: i32,
///     limit: i32,
///     return_ptr: *mut u8,
///     v_offset: i32,
/// ) -> i32;
/// ```
pub(crate) fn storage_get_range_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        engine,
        [
            ValType::I32,
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
/// fn storage_get_range(
///     k_ptr: *const u8,
///     k_len: i32,
///     direction: i32,
///     limit: i32,
///     return_ptr: *mut u8,
///     v_offset: i32,
/// ) -> i32;
/// ```
pub fn storage_get_range<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ComponentProvider<NucleusState>,
{
    let k_ptr = params[0].unwrap_i32();
    let k_len = params[1].unwrap_i32();
    let direction: Direction = params[2].unwrap_i32().into();
    let mut limit = params[3].unwrap_i32() as usize;
    let r_ptr = params[4].unwrap_i32();
    let v_offset = params[5].unwrap_i32();
    let key =
        mem::read_bytes_from_memory(&mut caller, k_ptr, k_len).expect("can't read bytes from wasm");
    let db = caller.data().get_component();
    let direction = match direction {
        Direction::Forward => IterDirection::Forward,
        Direction::Reverse => IterDirection::Reverse,
    };
    db.apply_on_user_data(IteratorMode::From(&key, direction), |mut iter| {
        let mut collected = vec![];
        while let Some(kv) = iter.next() {
            if limit == 0 {
                break;
            }
            match kv {
                Err(e) => {
                    let flag = mem::write_to_memory(
                        &mut caller,
                        r_ptr,
                        CallResult::<Vec<(Vec<u8>, Vec<u8>)>>::Err(RuntimeError::KvStorageError(
                            e.to_string(),
                        )),
                        Some(v_offset),
                    )?;
                    result[0] = flag;
                    return Ok(());
                }
                Ok((k, v)) => collected.push((k.to_vec(), v.to_vec())),
            }
            limit -= 1;
        }
        let flag = mem::write_to_memory(
            &mut caller,
            r_ptr,
            CallResult::<Vec<(Vec<u8>, Vec<u8>)>>::Ok(collected),
            Some(v_offset),
        )?;
        result[0] = flag;
        Ok(())
    })
}

/// the signature of this host function is:
///
/// ```
/// fn storage_del_range(
///    s0_ptr: *const u8,
///    s0_len: i32,
///    s1_ptr: *const u8,
///    s1_len: i32,
///    return_ptr: *mut u8,
/// ) -> i32;
/// ```
pub(crate) fn storage_del_range_signature(engine: &Engine) -> FuncType {
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
/// fn storage_del_range(
///    s0_ptr: *const u8,
///    s0_len: i32,
///    s1_ptr: *const u8,
///    s1_len: i32,
///    return_ptr: *mut u8,
/// ) -> i32;
/// ```
pub fn storage_del_range<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ComponentProvider<NucleusState> + ContextAware,
{
    result[0] = Val::I32(NO_MORE_DATA);
    let r_ptr = params[4].unwrap_i32();
    if caller.data().read_only() {
        let return_value = CallResult::<()>::Err(RuntimeError::WriteIsNotAllowInGetMethod);
        let bytes = return_value.encode();
        assert!(bytes.len() <= BUFFER_LEN);
        mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
        return Ok(());
    }
    let start_key_ptr = params[0].unwrap_i32();
    let start_key_len = params[1].unwrap_i32();
    let end_key_ptr = params[2].unwrap_i32();
    let end_key_len = params[3].unwrap_i32();
    let start_key = mem::read_bytes_from_memory(&mut caller, start_key_ptr, start_key_len)
        .expect("can't read bytes from wasm");
    let end_key = mem::read_bytes_from_memory(&mut caller, end_key_ptr, end_key_len)
        .expect("can't read bytes from wasm");
    let db = caller.data().get_component();
    let return_value = if let Err(e) = db.del_user_data_range(&start_key, &end_key) {
        CallResult::<()>::Err(RuntimeError::KvStorageError(e.to_string()))
    } else {
        CallResult::<()>::Ok(())
    };
    let bytes = return_value.encode();
    assert!(bytes.len() <= BUFFER_LEN);
    mem::write_bytes_to_memory(&mut caller, r_ptr, &bytes).expect("write to wasm failed");
    Ok(())
}
