use blake2::{Blake2s256, Digest};
use vrs_primitives::NucleusId;

#[derive(Debug)]
pub struct WasmInfo {
    pub id: NucleusId,
    pub version: u32,
    pub code: Vec<u8>,
}

impl WasmInfo {
    pub fn new(id: NucleusId, version: u32, code: Vec<u8>) -> Self {
        Self { id, version, code }
    }

    pub fn digest(&self) -> [u8; 32] {
        let mut hasher = Blake2s256::new();
        hasher.update(self.code.as_slice());
        hasher.finalize().into()
    }
}
