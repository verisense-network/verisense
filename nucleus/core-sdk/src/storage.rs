use crate::constant::StorageError;
use crate::MAX_GET_RETURN_SIZE;

#[link(wasm_import_module = "env")]
extern "C" {
    fn storage_put(
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    ) -> i32;

    fn storage_get_len(key_ptr: *const u8, key_len: u32, value_len_ptr: *mut u32) -> i32;

    fn storage_get(k_ptr: *const u8, k_len: u32, v_ptr: *mut u8, v_len_ptr: *mut u32) -> i32;
}

pub fn put(key: &[u8], value: &[u8]) -> anyhow::Result<()> {
    let status = unsafe { storage_put(key.as_ptr(), key.len(), value.as_ptr(), value.len()) };
    if status != 0 {
        Err(anyhow::anyhow!(StorageError::from(status).to_string()))
    } else {
        Ok(())
    }
}

// TODO
pub fn get_static(key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
    let mut value = vec![0u8; MAX_GET_RETURN_SIZE];
    let mut value_len: u32 = 0;
    let status = unsafe {
        storage_get(
            key.as_ptr(),
            key.len() as u32,
            value.as_mut_ptr(),
            &mut value_len as *mut u32,
        )
    };
    if status == 4 {
        return Ok(None);
    }
    if value_len >= MAX_GET_RETURN_SIZE as u32 {
        return Err(anyhow::anyhow!("Value length exceeds maximum allowed size"));
    }
    if status != 0 {
        Err(anyhow::anyhow!(StorageError::from(status).to_string()))
    } else {
        Ok(Some(value[..value_len as usize].to_vec()))
    }
}

pub fn get(key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
    let mut value_len: u32 = 0;
    let status =
        unsafe { storage_get_len(key.as_ptr(), key.len() as u32, &mut value_len as *mut u32) };
    if status == 4 {
        return Ok(None);
    }

    match status {
        0 => {
            let mut value = vec![0u8; value_len as usize];

            let status = unsafe {
                storage_get(
                    key.as_ptr(),
                    key.len() as u32,
                    value.as_mut_ptr(),
                    &mut value_len as *mut u32,
                )
            };
            if status == 4 {
                return Ok(None);
            }
            if value.len() != value_len as usize {
                return Err(anyhow::anyhow!("Value length mismatch"));
            }
            match status {
                0 => Ok(Some(value)),
                _ => Err(anyhow::anyhow!(StorageError::from(status).to_string())),
            }
        }
        _ => Err(anyhow::anyhow!(StorageError::from(status).to_string())),
    }
}
