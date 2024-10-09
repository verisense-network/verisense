mod backend;

use self::backend::RocksDbBackend;
use xmt::{blake2b::Blake2bHasher, SparseMerkleTree, H256};

// we only store the hash of each value since we need to scan the keys
// TODO
type State = SparseMerkleTree<Blake2bHasher, Vec<u8>, RocksDbBackend<Vec<u8>>>;

pub struct NucleusState {
    backend: RocksDbBackend<Vec<u8>>,
    // event_id: u64,
    state: State,
}

impl NucleusState {
    pub fn new(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let backend = RocksDbBackend::open(path)?;
        let state = State::new_with_store(backend.clone()).map_err(|e| anyhow::anyhow!(e))?;
        Ok(Self { backend, state })
    }

    pub fn get_root(&self) -> H256 {
        *self.state.root()
    }
}
