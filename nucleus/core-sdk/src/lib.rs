pub use vrs_core_macros::*;
pub use vrs_primitives::{AccountId, Balance, BlockNumber, Hash, Nonce, Signature};

use std::cell::Cell;
#[allow(improper_ctypes)]
pub mod storage {
    #[link(wasm_import_module = "env")]
    extern "C" {
        fn storage_put(
            key_ptr: *const u8,
            key_len: usize,
            value_ptr: *const u8,
            value_len: usize,
        ) -> i32;
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
}
