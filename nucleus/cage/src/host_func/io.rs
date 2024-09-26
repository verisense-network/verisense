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
    print!("{}", s);
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
    eprint!("{}", s);
    Ok(())
}
