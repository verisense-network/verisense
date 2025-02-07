use crate::runtime::ContextAware;
use codec::Decode;
use wasmtime::{Caller, Engine, FuncType, Val, ValType};

pub(crate) fn stdout_print_signature(engine: &Engine) -> FuncType {
    FuncType::new(engine, [ValType::I32, ValType::I32], [])
}

pub fn stdout_print<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    _result: &mut [Val],
) -> anyhow::Result<()> {
    let ptr = params[0].unwrap_i32();
    let len = params[1].unwrap_i32();
    let bytes = crate::mem::read_bytes_from_memory(&mut caller, ptr, len)
        .expect("can't read bytes from wasm");
    let s = <String as Decode>::decode(&mut bytes.as_slice()).expect("can't decode string");
    log::info!("ℹ️nucleus stdout: {}", s);
    Ok(())
}

pub(crate) fn stderr_print_signature(engine: &Engine) -> FuncType {
    FuncType::new(engine, [ValType::I32, ValType::I32], [])
}

pub fn stderr_print<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    _result: &mut [Val],
) -> anyhow::Result<()> {
    let ptr = params[0].unwrap_i32();
    let len = params[1].unwrap_i32();
    let bytes = crate::mem::read_bytes_from_memory(&mut caller, ptr, len)
        .expect("can't read bytes from wasm");
    let s = <String as Decode>::decode(&mut bytes.as_slice()).expect("can't decode string");
    log::error!("⚠️nucleus stderr: {}", s);
    Ok(())
}

/// the signature of this host function is:
///
/// ```
/// fn get_nucleus_id(return_ptr: *mut u8);
/// ```
pub(crate) fn get_nucleus_id_signature(engine: &Engine) -> FuncType {
    FuncType::new(engine, [ValType::I32], [])
}

/// the signature of this host function is:
///
/// ```
/// fn get_nucleus_id(return_ptr: *mut u8);
/// ```
pub(crate) fn get_nucleus_id<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    _result: &mut [Val],
) -> anyhow::Result<()>
where
    R: ContextAware,
{
    let r_ptr = params[0].unwrap_i32();
    let nucleus_id = caller.data().get_nucleus_id();
    crate::mem::write_to_memory(&mut caller, r_ptr, nucleus_id, None)
        .expect("write to wasm failed");
    Ok(())
}
