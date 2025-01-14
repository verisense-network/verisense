use blake2::{Blake2s256, Digest};
use vrs_primitives::{NodeId, NucleusId};

#[derive(Debug)]
pub struct WasmInfo {
    pub id: NucleusId,
    pub version: u32,
    pub code: Vec<u8>,
}

impl WasmInfo {
    pub fn from_blob(id: NucleusId, version: u32, code: Vec<u8>) -> Self {
        Self { id, version, code }
    }

    pub fn from_local<P: AsRef<std::path::Path>>(
        id: NucleusId,
        version: u32,
        path: P,
    ) -> std::io::Result<Self> {
        let code = std::fs::read(path.as_ref())?;
        Ok(Self { id, version, code })
    }

    pub fn from_peer(id: NucleusId, version: u32, peer: NodeId) -> Result<Self, String> {
        todo!()
    }

    pub fn digest(&self) -> [u8; 32] {
        let mut hasher = Blake2s256::new();
        hasher.update(self.code.as_slice());
        hasher.finalize().into()
    }
}
