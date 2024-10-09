mod backend;

use self::backend::RocksDbBackend;
use rocksdb::DB;
use std::sync::Arc;
use xmt::{blake2b::Blake2bHasher, SparseMerkleTree};

pub use xmt::H256;

type State = SparseMerkleTree<Blake2bHasher, H256, RocksDbBackend>;

#[derive(Debug)]
pub struct NucleusState {
    pub backend: Arc<DB>,
    pub event_id: u64,
    pub root: H256,
}
