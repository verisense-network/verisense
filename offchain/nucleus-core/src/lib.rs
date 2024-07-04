pub use vrs_primitives::{AccountId, Balance, BlockNumber, Hash, Nonce, Signature};

#[allow(improper_ctypes)]
pub mod storage {

    extern "C" {
        fn storage_put(key: &[u8], value: &[u8]);
    }

    pub fn put(key: &[u8], value: &[u8]) -> anyhow::Result<()> {
        unsafe {
            storage_put(key, value);
        }
        Ok(())
    }
}
