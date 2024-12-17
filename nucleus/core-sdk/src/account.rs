use codec::{Decode, Encode};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Encode, Decode)]
pub struct AccountId(pub [u8; 32]);

impl AccountId {
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

impl std::ops::Deref for AccountId {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for AccountId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        if s.starts_with("0x") {
            Self::from_hex_str(s)
        } else {
            use sp_core::crypto::Ss58Codec;
            Self::from_ss58check(s).map_err(|_| anyhow::anyhow!("invalid ss58 format"))
        }
    }
}

impl sp_core::crypto::Ss58Codec for AccountId {}

impl std::string::ToString for AccountId {
    fn to_string(&self) -> String {
        use sp_core::crypto::Ss58Codec;
        self.to_ss58check()
    }
}

impl std::fmt::Debug for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use sp_core::crypto::Ss58Codec;
        let s = self.to_ss58check();
        write!(f, "{}", &s)
    }
}

impl AsRef<[u8]> for AccountId {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for AccountId {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl AsRef<[u8; 32]> for AccountId {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsMut<[u8; 32]> for AccountId {
    fn as_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl From<[u8; 32]> for AccountId {
    fn from(x: [u8; 32]) -> Self {
        Self::new(x)
    }
}

impl sp_core::ByteArray for AccountId {
    const LEN: usize = 32;
}

impl<'a> TryFrom<&'a [u8]> for AccountId {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(());
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&value[..]);
        return Ok(AccountId::new(out));
    }
}
