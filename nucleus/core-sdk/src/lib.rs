// pub use vrs_core_macros::*
pub use codec;
pub use vrs_primitives::{AccountId, Balance, BlockNumber, Hash, Nonce, Signature};
mod constant;
pub use constant::MAX_GET_RETURN_SIZE;
#[allow(improper_ctypes)]
pub mod storage {
    use std::any;

    use crate::MAX_GET_RETURN_SIZE;

    #[link(wasm_import_module = "env")]
    extern "C" {
        fn storage_put(
            key_ptr: *const u8,
            key_len: usize,
            value_ptr: *const u8,
            value_len: usize,
        ) -> i32;
    }

    #[link(wasm_import_module = "env")]
    extern "C" {
        fn storage_get_len(key_ptr: *const u8, key_len: u32, value_len_ptr: *mut u32) -> u32;
        fn storage_get_data(
            k_ptr: *const u8,
            k_len: u32,
            v_ptr: *mut u8,
            v_len_ptr: *mut u32,
        ) -> i32;
        fn storage_get(k_ptr: *const u8, k_len: u32, v_ptr: *mut u8, v_len_ptr: *mut u32) -> i32;

    }

    pub fn put(key: &[u8], value: &[u8]) -> anyhow::Result<()> {
        let result = unsafe { storage_put(key.as_ptr(), key.len(), value.as_ptr(), value.len()) };
        match result {
            0 => Ok(()),
            1 => Err(anyhow::anyhow!("Operation not allowed in GET method")),
            2 => Err(anyhow::anyhow!("Memory access out of bounds")),
            3 => Err(anyhow::anyhow!("Database error")),
            _ => Err(anyhow::anyhow!("Unknown error")),
        }
    }
    pub fn get(key: &[u8]) -> anyhow::Result<Vec<u8>> {
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
        if value_len >= MAX_GET_RETURN_SIZE as u32 {
            return Err(anyhow::anyhow!("Value length exceeds maximum allowed size"));
        }
        match status {
            0 => Ok(value[..value_len as usize].to_vec()),
            1 => Err(anyhow::anyhow!("Key not found")),
            2 => Err(anyhow::anyhow!("Database error")),
            _ => Err(anyhow::anyhow!("Unknown error")),
        }
    }
    pub fn get_dynamic(key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut value_len: u32 = 0;
        let status =
            unsafe { storage_get_len(key.as_ptr(), key.len() as u32, &mut value_len as *mut u32) };

        match status {
            0 => {
                let mut value = vec![0u8; value_len as usize];

                let status = unsafe {
                    storage_get_data(
                        key.as_ptr(),
                        key.len() as u32,
                        value.as_mut_ptr(),
                        &mut value_len as *mut u32,
                    )
                };
                if value.len() != value_len as usize {
                    return Err(anyhow::anyhow!("Value length mismatch"));
                }
                match status {
                    0 => Ok(value),
                    1 => Err(anyhow::anyhow!("Key not found")),
                    2 => Err(anyhow::anyhow!("Database error")),
                    _ => Err(anyhow::anyhow!("Unknown error")),
                }
            }
            1 => Err(anyhow::anyhow!("Key not found")),
            2 => Err(anyhow::anyhow!("Database error")),
            _ => Err(anyhow::anyhow!("Unknown error")),
        }
    }
}
