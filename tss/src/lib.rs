use core::fmt;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sp_core::crypto::KeyTypeId;
use sp_core::ByteArray;
use sp_keystore::Keystore;
mod runtime;
mod testing;
pub use runtime::*;
pub use tss_sdk::*;
mod mock;

use sp_core::crypto::AccountId32;
use sp_core::crypto::Pair as CryptoPair;
use sp_core::sr25519::{Pair, Public, Signature};
use sp_keystore::KeystorePtr;
use tss_sdk::crypto::{
    ValidatorIdentity, ValidatorIdentityIdentity, ValidatorIdentityKeypair,
    ValidatorIdentityPublicKey,
};
#[derive(Clone)]
pub struct TssKeystore {
    keystore: KeystorePtr,
    key_type: KeyTypeId,
    public: Public,
}
impl Debug for TssKeystore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "public: {:?}, key_type: {:?}",
            self.public, self.key_type
        )
    }
}
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TssPublicKey(pub Public);
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TssIdentity(pub AccountId32);

#[derive(Debug, Clone)]
pub struct Sr25519Identity;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VrsTssValidatorIdentity; // SHA256 hash of public key

impl ValidatorIdentity for VrsTssValidatorIdentity {
    type Keypair = TssKeystore;
    type PublicKey = TssPublicKey;
    type Identity = TssIdentity;
}
#[derive(Debug)]
pub enum Sr25519SignError {
    SignError(String),
}
impl fmt::Display for Sr25519SignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sr25519SignError")
    }
}
impl Error for Sr25519SignError {}
impl TssKeystore {
    pub fn new(keystore: KeystorePtr, key_type: KeyTypeId) -> anyhow::Result<Self> {
        let public = keystore
            .sr25519_public_keys(key_type)
            .first()
            .copied()
            .ok_or(anyhow::anyhow!(
                "No {:?} key found, please insert one",
                key_type
            ))?;
        Ok(Self {
            keystore,
            public,
            key_type,
        })
    }
}
impl ValidatorIdentityKeypair for TssKeystore {
    type PublicKey = TssPublicKey;
    type SignError = Sr25519SignError;

    fn to_public_key(&self) -> TssPublicKey {
        TssPublicKey(self.public)
    }

    fn sign<T: AsRef<[u8]>>(&self, message: T) -> Result<Vec<u8>, Self::SignError> {
        let signature = self
            .keystore
            .sr25519_sign(self.key_type, &self.public, message.as_ref())
            .map_err(|e| Sr25519SignError::SignError(e.to_string()))?
            .ok_or(Sr25519SignError::SignError(
                "No signature found".to_string(),
            ))?;
        Ok(signature.0.to_vec())
    }

    fn derive_key(&self, salt: &[u8]) -> Vec<u8> {
        // use sha256 of salt to sign the keystore
        let mut hash = sha2::Sha256::new();
        hash.update("derive_key_from_salt");
        hash.update(salt);
        let hash = hash.finalize();
        // use sr25519_vrf_pre_output to derive the key

        let input = sp_core::sr25519::vrf::VrfInput::new(
            b"derive_key_from_salt",
            &[
                (b"salt", &hash),
                (b"public", &self.public.to_raw_vec()),
                (b"key_type", &self.key_type.0),
            ],
        );
        let result = self
            .keystore
            .sr25519_vrf_pre_output(self.key_type, &self.public, &input)
            .unwrap();
        result.unwrap().0.to_bytes().to_vec()
    }

    fn random_generate_keypair() -> Self {
        use crate::testing::MemoryKeystore;
        use std::sync::Arc;
        let keystore = Arc::new(MemoryKeystore::new());
        let key_type = KeyTypeId::from(0x74657374); // "test" in hex
        let public = keystore.sr25519_generate_new(key_type, None).unwrap();
        Self {
            keystore,
            public,
            key_type,
        }
    }
}

#[derive(Debug)]
pub enum IdentityDecodeError {
    InvalidLength,
    InvalidAccountId32(String),
}

impl fmt::Display for IdentityDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Invalid identity length"),
            Self::InvalidAccountId32(e) => write!(f, "Invalid account id32: {}", e),
        }
    }
}

impl Error for IdentityDecodeError {}

impl ValidatorIdentityIdentity for TssIdentity {
    type PublicKey = TssPublicKey;
    type DecodeError = IdentityDecodeError;

    fn from_public_key(public_key: TssPublicKey) -> Self {
        TssIdentity(AccountId32::from(public_key.0))
    }

    fn to_fmt_string(&self) -> String {
        self.0.to_string()
    }

    fn from_fmt_str(s: &str) -> Result<Self, Self::DecodeError> {
        Ok(TssIdentity(AccountId32::from_str(s).map_err(|_| {
            Self::DecodeError::InvalidAccountId32(s.to_string())
        })?))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_raw_vec()
    }

    fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, Self::DecodeError> {
        Ok(TssIdentity(
            AccountId32::from_slice(bytes.as_ref()).map_err(|_| {
                Self::DecodeError::InvalidAccountId32(format!("{:?}", bytes.as_ref()))
            })?,
        ))
    }
}

impl ValidatorIdentityPublicKey for TssPublicKey {
    type Identity = TssIdentity;
    type Keypair = TssKeystore;
    type DecodeError = IdentityDecodeError;

    fn to_identity(&self) -> Self::Identity {
        TssIdentity(AccountId32::from(self.0))
    }

    fn from_keypair(keypair: &TssKeystore) -> Self {
        TssPublicKey(keypair.public)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_raw_vec()
    }

    fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, Self::DecodeError> {
        Ok(TssPublicKey(Public::from_slice(bytes.as_ref()).map_err(
            |_| Self::DecodeError::InvalidAccountId32(format!("{:?}", bytes.as_ref())),
        )?))
    }

    fn verify<T: AsRef<[u8]>, S: AsRef<[u8]>>(&self, message: T, signature: S) -> bool {
        if let Ok(sig) = Signature::from_slice(signature.as_ref()) {
            <Pair as CryptoPair>::verify(&sig, message.as_ref(), &self.0)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        test_single::<VrsTssValidatorIdentity>();
    }
    fn test_single<T: ValidatorIdentity>() {
        let keypair = T::Keypair::random_generate_keypair();
        // test to_public_key
        let public_key = keypair.clone().to_public_key();
        let public_key1 = T::PublicKey::from_keypair(&keypair);
        assert_eq!(public_key, public_key1);
        // test derive_key
        let salt = b"test";
        let derived_key = keypair.derive_key(salt);
        let derived_key1 = keypair.clone().derive_key(salt);
        assert_eq!(derived_key, derived_key1);
        let another_salt = b"test2";
        let derived_key2 = keypair.derive_key(another_salt);
        assert_ne!(derived_key, derived_key2);

        // test sign
        let message = b"test";
        let signature = keypair.sign(message).unwrap();
        assert!(public_key.verify(message, &signature));
        assert!(!public_key.verify("test2", &signature));
        let new_keypair = T::Keypair::random_generate_keypair();
        let signature2 = new_keypair.sign("test").unwrap();
        assert!(!public_key.verify("test", &signature2));

        // test public key
        let public_key2 = T::PublicKey::from_keypair(&new_keypair);
        assert_ne!(public_key, public_key2);
        // test from_bytes and to_bytes
        let bytes = public_key.to_bytes();
        let public_key3 = T::PublicKey::from_bytes(&bytes).unwrap();
        assert_eq!(public_key, public_key3);
        assert_ne!(public_key3, public_key2);

        //test to_identity
        let identity = public_key.to_identity();
        let identity2 = public_key2.to_identity();
        let identity3 = public_key3.to_identity();
        assert_ne!(identity, identity2);
        assert_eq!(identity, identity3);
        assert_ne!(identity2, identity3);

        //test from_public_key
        let identity_from = T::Identity::from_public_key(public_key);
        assert_eq!(identity, identity_from);
        let identity_from2 = T::Identity::from_public_key(public_key2);
        assert_ne!(identity, identity_from2);
        let identity_from3 = T::Identity::from_public_key(public_key3);
        assert_eq!(identity, identity_from3);

        //test to_fmt_string
        let fmt_str = identity.to_fmt_string();
        let identity4 = T::Identity::from_fmt_str(&fmt_str).unwrap();
        assert_eq!(identity, identity4);

        let bytes = identity.to_bytes();
        let identity5 = T::Identity::from_bytes(&bytes).unwrap();
        assert_eq!(identity, identity5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_id32() {
        let id = TssIdentity(AccountId32::from([1u8; 32]));
        let bytes = id.to_bytes();
        assert_eq!(bytes.len(), 32);
        let decoded = TssIdentity::from_bytes(bytes).unwrap();
        assert_eq!(id, decoded);

        let str_repr = id.to_fmt_string();
        let decoded = TssIdentity::from_fmt_str(&str_repr).unwrap();
        assert_eq!(id, decoded);
    }

    #[test]
    fn test_public_key() {
        let keystore = TssKeystore::random_generate_keypair();
        let public = keystore.to_public_key();

        // Test to/from bytes
        let bytes = public.to_bytes();
        let decoded = TssPublicKey::from_bytes(&bytes).unwrap();
        assert_eq!(public, decoded);

        // Test verify
        let message = b"test message";
        let signature = keystore.sign(message).unwrap();
        assert!(<TssPublicKey as ValidatorIdentityPublicKey>::verify(
            &public,
            message,
            signature.clone()
        ));

        // Test invalid signature
        let mut bad_sig = signature.to_vec();
        bad_sig[0] ^= 1;
        assert!(!public.verify(message, bad_sig));
    }

    #[test]
    fn test_identity_from_public_key() {
        let keystore = TssKeystore::random_generate_keypair();
        let public = keystore.to_public_key();

        let id = public.to_identity();
        assert_eq!(id.to_bytes().len(), 32);
    }

    #[test]
    fn test_signing_key() {
        let keystore = TssKeystore::random_generate_keypair();
        // Test to_public_key
        let public_key = keystore.to_public_key();
        assert_eq!(public_key, TssPublicKey(keystore.public));

        // Test sign
        let message = b"test message";
        let signature = keystore.sign(message).unwrap();
        assert_eq!(signature.len(), 64);

        // Test derive_key
        let salt = b"test salt";
        let derived_key = keystore.derive_key(salt);
        assert!(!derived_key.is_empty());

        // Test random_generate_keypair
        let random_key = TssKeystore::random_generate_keypair();
        assert_ne!(random_key.public, keystore.public);
    }

    #[test]
    fn test_identity_decode_error() {
        // Test InvalidLength error
        let bytes = vec![0u8; 16];
        let result = TssIdentity::from_bytes(bytes);
        assert!(matches!(
            result,
            Err(IdentityDecodeError::InvalidAccountId32(_))
        ));

        // Test HexError
        let invalid_hex = "invalid hex string";
        let result = TssIdentity::from_fmt_str(invalid_hex);
        assert!(matches!(
            result,
            Err(IdentityDecodeError::InvalidAccountId32(_))
        ));

        // Test error formatting
        let invalid_length_error = IdentityDecodeError::InvalidLength;
        assert_eq!(invalid_length_error.to_string(), "Invalid identity length");

        let hex_error = IdentityDecodeError::InvalidAccountId32(invalid_hex.to_string());
        assert!(hex_error.to_string().contains("Invalid account id32"));
    }
}
