use codec::{Decode, Encode};
use rocksdb::{ColumnFamily, ColumnFamilyDescriptor, Options, DB};
use xmt::{
    error::Error,
    merge::MergeValue,
    traits::{StoreReadOps, StoreWriteOps},
    BranchKey, BranchNode, H256,
};

#[derive(Debug, Clone)]
pub struct RocksDbBackend<V> {
    inner: std::sync::Arc<DB>,
    _v: std::marker::PhantomData<V>,
}

#[derive(Clone, Debug, Encode, Decode)]
enum SerializableMergeValue {
    Value([u8; 32]),
    MergeWithZero {
        base_node: [u8; 32],
        zero_bits: [u8; 32],
        zero_count: u8,
    },
}

impl From<MergeValue> for SerializableMergeValue {
    fn from(v: MergeValue) -> Self {
        match v {
            MergeValue::Value(v) => Self::Value(<[u8; 32]>::from(v)),
            MergeValue::MergeWithZero {
                base_node,
                zero_bits,
                zero_count,
            } => Self::MergeWithZero {
                base_node: <[u8; 32]>::from(base_node),
                zero_bits: <[u8; 32]>::from(zero_bits),
                zero_count,
            },
        }
    }
}

impl Into<MergeValue> for SerializableMergeValue {
    fn into(self) -> MergeValue {
        match self {
            Self::Value(v) => MergeValue::Value(H256::from(v)),
            Self::MergeWithZero {
                base_node,
                zero_bits,
                zero_count,
            } => MergeValue::MergeWithZero {
                base_node: H256::from(base_node),
                zero_bits: H256::from(zero_bits),
                zero_count,
            },
        }
    }
}

impl<V> RocksDbBackend<V> {
    pub fn open(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let avs_cf = ColumnFamilyDescriptor::new("avs", Options::default());
        let seq_cf = ColumnFamilyDescriptor::new("seq", Options::default());
        let tree_cf = ColumnFamilyDescriptor::new("tree", Options::default());
        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        let db = DB::open_cf_descriptors(&db_opts, path, vec![avs_cf, seq_cf, tree_cf])
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(Self {
            inner: std::sync::Arc::new(db),
            _v: std::marker::PhantomData,
        })
    }

    pub(crate) fn data_cf(&self) -> &rocksdb::ColumnFamily {
        self.inner.cf_handle("avs").unwrap()
    }

    pub(crate) fn seq_cf(&self) -> &ColumnFamily {
        self.inner.cf_handle("seq").unwrap()
    }

    pub(crate) fn state_cf(&self) -> &ColumnFamily {
        self.inner.cf_handle("tree").unwrap()
    }

    fn leaf_key(key: &H256) -> Vec<u8> {
        let b = <[u8; 32]>::from(*key);
        (b"leaf", b).encode()
    }

    fn branch_key(key: &BranchKey) -> Vec<u8> {
        let BranchKey { height, node_key } = *key;
        let b = <[u8; 32]>::from(node_key);
        (b"bran", height, b).encode()
    }

    fn encode_branch(branch: BranchNode) -> Vec<u8> {
        let BranchNode { left, right } = branch;
        let (left, right): (SerializableMergeValue, SerializableMergeValue) =
            (left.into(), right.into());
        (left, right).encode()
    }

    fn decode_branch(v: &mut &[u8]) -> Result<BranchNode, Error> {
        let (left, right) = <(SerializableMergeValue, SerializableMergeValue)>::decode(v)
            .map_err(|e| Error::Store(e.to_string()))?;
        Ok(BranchNode {
            left: left.into(),
            right: right.into(),
        })
    }
}

impl<V: TryFrom<Vec<u8>>> StoreReadOps<V> for RocksDbBackend<V> {
    fn get_branch(&self, key: &BranchKey) -> Result<Option<BranchNode>, Error> {
        let v = self
            .inner
            .get_cf(self.state_cf(), &Self::branch_key(key))
            .map_err(|e| Error::Store(e.to_string()))?;
        v.map(|v| Self::decode_branch(&mut &v[..])).transpose()
    }

    fn get_leaf(&self, key: &H256) -> Result<Option<V>, Error> {
        let v = self
            .inner
            .get_cf(self.state_cf(), &Self::leaf_key(key))
            .map_err(|e| Error::Store(e.to_string()))?;
        v.map(|v| V::try_from(v).map_err(|_| Error::Store("invalid value".to_string())))
            .transpose()
    }
}

impl<V: AsRef<[u8]>> StoreWriteOps<V> for RocksDbBackend<V> {
    fn insert_branch(&mut self, key: BranchKey, branch: BranchNode) -> Result<(), Error> {
        self.inner
            .put_cf(
                self.state_cf(),
                &Self::branch_key(&key),
                &Self::encode_branch(branch),
            )
            .map_err(|e| Error::Store(e.to_string()))?;
        Ok(())
    }

    fn insert_leaf(&mut self, key: H256, leaf: V) -> Result<(), Error> {
        self.inner
            .put_cf(self.state_cf(), &Self::leaf_key(&key), leaf.as_ref())
            .map_err(|e| Error::Store(e.to_string()))?;
        Ok(())
    }

    fn remove_branch(&mut self, key: &BranchKey) -> Result<(), Error> {
        self.inner
            .delete_cf(self.state_cf(), &Self::branch_key(key))
            .map_err(|e| Error::Store(e.to_string()))?;
        Ok(())
    }

    fn remove_leaf(&mut self, key: &H256) -> Result<(), Error> {
        self.inner
            .delete_cf(self.state_cf(), &Self::leaf_key(key))
            .map_err(|e| Error::Store(e.to_string()))?;
        Ok(())
    }
}
