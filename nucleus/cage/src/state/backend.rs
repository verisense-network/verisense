use serde::{Deserialize, Serialize};
use xmt::{
    error::Error,
    merge::MergeValue,
    traits::{StoreReadOps, StoreWriteOps},
    BranchKey, BranchNode, H256,
};

#[derive(Debug)]
pub struct RocksDbBackend {}
