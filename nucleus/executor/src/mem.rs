use vrs_core_sdk::{BUFFER_LEN, NO_MORE_DATA};
use wasmtime::{Caller, Extern, Memory, Trap, Val};

pub(crate) fn wasm_mem<R>(caller: &mut Caller<'_, R>) -> Memory {
    match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => panic!("Invalid WASM: no memory exported"),
    }
}

pub(crate) fn read_bytes_from_memory<R>(
    caller: &mut Caller<'_, R>,
    ptr: i32,
    len: i32,
) -> Result<Vec<u8>, Trap> {
    let mem = self::wasm_mem(caller);
    if (ptr as u64 + len as u64) > mem.data_size(&caller) as u64 {
        return Err(Trap::MemoryOutOfBounds);
    }
    let data = mem.data(&caller)[ptr as usize..(ptr + len) as usize].to_vec();
    Ok(data)
}

pub(crate) fn write_bytes_to_memory<R>(
    caller: &mut Caller<'_, R>,
    ptr: i32,
    data: &[u8],
) -> Result<(), Trap> {
    let mem = self::wasm_mem(caller);
    if (ptr as u64 + data.len() as u64) > mem.data_size(&caller) as u64 {
        return Err(Trap::MemoryOutOfBounds);
    }
    mem.write(caller, ptr as usize, data)
        .map_err(|_| Trap::MemoryOutOfBounds)
}

pub(crate) fn write_to_memory<T: codec::Encode, R>(
    caller: &mut Caller<'_, R>,
    ptr: i32,
    data: T,
    offset: Option<i32>,
) -> Result<Val, Trap> {
    let bytes = data.encode();
    if bytes.len() > BUFFER_LEN {
        assert!(offset.is_some());
        let offset = offset.unwrap() as usize;
        if offset >= bytes.len() {
            return Err(Trap::MemoryOutOfBounds);
        } else if offset + BUFFER_LEN >= bytes.len() {
            self::write_bytes_to_memory(caller, ptr, &bytes[offset..])?;
            Ok(Val::I32(NO_MORE_DATA))
        } else {
            let bytes = &bytes[offset..offset + BUFFER_LEN];
            self::write_bytes_to_memory(caller, ptr, &bytes[offset..=offset + BUFFER_LEN])?;
            Ok(Val::I32(1))
        }
    } else {
        self::write_bytes_to_memory(caller, ptr, &bytes)?;
        Ok(Val::I32(NO_MORE_DATA))
    }
}
