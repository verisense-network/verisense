mod backend;

use self::backend::RocksDbBackend;
use codec::{Decode, Encode};
use rocksdb::{DBIterator, IteratorMode};
use tokio::sync::broadcast::Sender;
use xmt::{blake2b::Blake2bHasher, SparseMerkleTree};

// we only store the hash of each value since we need to scan the keys
type State = SparseMerkleTree<Blake2bHasher, B256, RocksDbBackend<B256>>;

pub struct NucleusState {
    state: State,
    broadcast: Sender<(Vec<u8>, Option<Vec<u8>>)>,
}

impl NucleusState {
    pub fn new(
        path: impl AsRef<std::path::Path>,
        broadcast: Sender<(Vec<u8>, Option<Vec<u8>>)>,
    ) -> anyhow::Result<Self> {
        let backend = RocksDbBackend::open(path)?;
        let state = State::new_with_store(backend).map_err(|e| anyhow::anyhow!(e))?;
        Ok(Self { state, broadcast })
    }

    pub fn get_state_root(&self) -> B256 {
        B256::from(<[u8; 32]>::from(*self.state.root()))
    }

    pub fn del_user_data<K: AsRef<[u8]>>(&self, key: K) -> Result<(), String> {
        let backend = self.state.store();
        backend
            .inner
            .delete_cf(backend.data_cf(), key.as_ref())
            .map_err(|e| e.to_string())?;
        let _ = self.broadcast.send((key.as_ref().to_vec(), None));
        Ok(())
    }

    // FIXME: deleting range won't notify
    pub fn del_user_data_range<K: AsRef<[u8]>>(&self, from: K, end: K) -> Result<(), String> {
        let backend = self.state.store();
        backend
            .inner
            .delete_range_cf(backend.data_cf(), from.as_ref(), end.as_ref())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn put_user_data<K, V>(&self, key: K, val: V) -> Result<(), String>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        let backend = self.state.store();
        backend
            .inner
            .put_cf(backend.data_cf(), key.as_ref(), val.as_ref())
            .map_err(|e| e.to_string())?;
        let _ = self
            .broadcast
            .send((key.as_ref().to_vec(), Some(val.as_ref().to_vec())));
        Ok(())
    }

    pub fn get_user_data<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, String> {
        let backend = self.state.store();
        backend
            .inner
            .get_cf(backend.data_cf(), key.as_ref())
            .map_err(|e| e.to_string())
    }

    pub fn apply_on_user_data<F, R>(&self, mode: IteratorMode, mut f: F) -> R
    where
        F: FnMut(DBIterator) -> R,
    {
        let backend = self.state.store();
        let iter = backend.inner.iterator_cf(backend.data_cf(), mode.into());
        f(iter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Encode, Decode, Default)]
pub struct B256(pub [u8; 32]);

impl B256 {
    pub const fn zero() -> Self {
        Self([0; 32])
    }

    pub fn new(x: [u8; 32]) -> Self {
        Self(x)
    }

    pub fn from_hex_str(s: &str) -> anyhow::Result<Self> {
        let hex = s.trim_start_matches("0x");
        if hex.len() == 64 {
            let mut bytes = [0u8; 32];
            hex::decode_to_slice(hex, &mut bytes)
                .map_err(|_| anyhow::anyhow!("invalid hex string"))
                .map(|_| Self::from(bytes))
        } else {
            Err(anyhow::anyhow!("invalid hex string"))
        }
    }
}

impl TryFrom<Vec<u8>> for B256 {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(());
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&value[..]);
        return Ok(B256::new(out));
    }
}

impl std::ops::Deref for B256 {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for B256 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Self::from_hex_str(s)
    }
}

impl std::string::ToString for B256 {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl std::fmt::Debug for B256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.to_string();
        write!(f, "0x{}", &s)
    }
}

impl AsRef<[u8]> for B256 {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for B256 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl AsRef<[u8; 32]> for B256 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsMut<[u8; 32]> for B256 {
    fn as_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl From<[u8; 32]> for B256 {
    fn from(x: [u8; 32]) -> Self {
        Self::new(x)
    }
}

impl<'a> TryFrom<&'a [u8]> for B256 {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(());
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&value[..]);
        return Ok(B256::new(out));
    }
}
