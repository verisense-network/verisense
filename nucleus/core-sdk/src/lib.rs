pub use vrs_core_macros::*;
pub use vrs_primitives::{AccountId, Balance, BlockNumber, Hash, Nonce, Signature};

#[allow(improper_ctypes)]
pub mod storage {

    extern "C" {
        fn storage_put(key: &[u8], value: &[u8]) -> anyhow::Result<()>;
        // fn storage_get(key: &[u8]) -> anyhow::Result<Vec<u8>>;
        // fn storage_del();
        // fn storage_range();
    }

    pub fn put(key: &[u8], value: &[u8]) -> anyhow::Result<()> {
        unsafe {
            storage_put(key, value)?;
        }
        Ok(())
    }
}
