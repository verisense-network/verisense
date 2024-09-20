use crate::{error::RuntimeError, CallResult};
use codec::Decode;

#[link(wasm_import_module = "env")]
extern "C" {
    fn storage_put(
        key_ptr: *const u8,
        key_len: i32,
        value_ptr: *const u8,
        value_len: i32,
        return_ptr: *mut u8,
    ) -> i32;

    fn storage_del(key_ptr: *const u8, key_len: i32, return_ptr: *mut u8) -> i32;

    fn storage_get(k_ptr: *const u8, k_len: i32, return_ptr: *mut u8, v_offset: i32) -> i32;
}

pub fn put(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> CallResult<()> {
    let key = key.as_ref();
    let value = value.as_ref();
    assert!(key.len() <= i32::MAX as usize);
    assert!(value.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let status = unsafe {
        storage_put(
            key.as_ptr(),
            key.len() as i32,
            value.as_ptr(),
            value.len() as i32,
            buf.as_mut_ptr(),
        )
    };
    assert!(status == crate::NO_MORE_DATA);
    CallResult::<()>::decode(&mut &buf[..]).map_err(|_| RuntimeError::DecodeReturnValueError)?
}

pub fn del(key: impl AsRef<[u8]>) -> CallResult<()> {
    let key = key.as_ref();
    assert!(key.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let status = unsafe { storage_del(key.as_ptr(), key.len() as i32, buf.as_mut_ptr()) };
    assert!(status == crate::NO_MORE_DATA);
    CallResult::<()>::decode(&mut &buf[..]).map_err(|_| RuntimeError::DecodeReturnValueError)?
}

pub fn get(key: impl AsRef<[u8]>) -> CallResult<Option<Vec<u8>>> {
    let key = key.as_ref();
    assert!(key.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let mut val = vec![];
    loop {
        let status = unsafe {
            storage_get(
                key.as_ptr(),
                key.len() as i32,
                buf.as_mut_ptr(),
                val.len() as i32,
            )
        };
        val.extend_from_slice(&buf);
        if status == crate::NO_MORE_DATA {
            break;
        }
    }
    CallResult::<Option<Vec<u8>>>::decode(&mut &val[..])
        .map_err(|_| RuntimeError::DecodeReturnValueError)?
}
