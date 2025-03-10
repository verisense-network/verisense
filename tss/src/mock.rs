use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use ed25519_dalek::{SigningKey as Ed25519SigningKey, VerifyingKey as Ed25519VerifyingKey};
use k256::{
    ecdsa::{signature::Signer, SigningKey as SecpSigningKey, VerifyingKey as SecpVerifyingKey},
    elliptic_curve::sec1::EncodedPoint,
};
use rand::{rngs::OsRng, TryRngCore};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Ed25519Pair {
    pub(crate) public: Ed25519VerifyingKey,
    pub(crate) secret: Ed25519SigningKey,
}

#[derive(Clone)]
pub(crate) struct Secp256k1Pair {
    pub(crate) public: SecpVerifyingKey,
    pub(crate) secret: SecpSigningKey,
}
#[derive(Serialize, Deserialize)]
struct Secp256k1PairHelper {
    // 32-byte secret scalar
    secret: [u8; 32],
    // SEC1-encoded public key (usually 33 bytes for compressed)
    public: Vec<u8>,
}

impl Serialize for Secp256k1Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let secret_bytes: [u8; 32] = self.secret.to_bytes().into();
        let public_point = self.public.to_encoded_point(true);
        let helper = Secp256k1PairHelper {
            secret: secret_bytes,
            public: public_point.as_bytes().to_vec(),
        };
        helper.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Secp256k1Pair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let helper = Secp256k1PairHelper::deserialize(deserializer)?;

        // Reconstruct the secret key from the 32-byte scalar
        let secret = SecpSigningKey::from_bytes(&helper.secret.into())
            .map_err(|e| D::Error::custom(format!("SecpSigningKey error: {}", e)))?;

        // Reconstruct the verifying key from the SEC1-encoded public bytes
        let encoded_point = EncodedPoint::<k256::Secp256k1>::from_bytes(&helper.public)
            .map_err(|e| D::Error::custom(format!("EncodedPoint error: {}", e)))?;
        let public = SecpVerifyingKey::from_encoded_point(&encoded_point)
            .map_err(|e| D::Error::custom(format!("VerifyingKey error: {}", e)))?;

        Ok(Secp256k1Pair { secret, public })
    }
}

#[derive(Serialize, Deserialize)]
pub struct MockKeystore {
    #[serde(skip)]
    pub(crate) path: PathBuf,
    pub(crate) ed25519_keystore: HashMap<Option<Vec<u8>>, Ed25519Pair>,
    pub(crate) secp256k1_keystore: HashMap<Option<Vec<u8>>, Secp256k1Pair>,
}

impl MockKeystore {
    pub fn new(base_path: &PathBuf) -> Self {
        let path = base_path.join(".mock_keystore");
        if path.exists() {
            let file = File::open(path.clone()).unwrap();
            let reader = BufReader::new(file);
            let mut keystore: MockKeystore = bincode::deserialize_from(reader).unwrap();
            keystore.path = path.clone();
            keystore
        } else {
            let keystore = MockKeystore {
                path: path.clone(),
                ed25519_keystore: HashMap::new(),
                secp256k1_keystore: HashMap::new(),
            };
            let file = File::create(path).unwrap();
            bincode::serialize_into(file, &keystore).unwrap();
            keystore
        }
    }

    pub fn write(&self) {
        let mut file = File::create(self.path.clone()).unwrap();
        bincode::serialize_into(&mut file, &self).unwrap();
    }

    pub fn get_secp_public_key(&mut self, seed: &Option<Vec<u8>>) -> Vec<u8> {
        match self.secp256k1_keystore.get(seed) {
            Some(pair) => pair.public.to_encoded_point(true).as_bytes().to_vec(),
            None => {
                let pair = Secp256k1Pair::new();
                self.secp256k1_keystore.insert(seed.clone(), pair.clone());
                self.write();
                pair.public.to_encoded_point(true).as_bytes().to_vec()
            }
        }
    }
    pub fn get_ed25519_public_key(&mut self, seed: &Option<Vec<u8>>) -> Vec<u8> {
        let pair = self.ed25519_keystore.get(&seed);
        match pair {
            Some(pair) => pair.public.to_bytes().to_vec(),
            None => {
                let pair = Ed25519Pair::new();
                self.ed25519_keystore.insert(seed.clone(), pair.clone());
                self.write();
                pair.public.to_bytes().to_vec()
            }
        }
    }

    pub fn secp_sign_recoverable(
        &mut self,
        seed: &Option<Vec<u8>>,
        message: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let _ = self.get_secp_public_key(seed);
        let secret = self.secp256k1_keystore.get_mut(seed).unwrap();
        let (signature, recovery_id) = secret
            .secret
            .sign_recoverable(&message)
            .map_err(|e| e.to_string())?;
        self.write();
        let mut signature_bytes = signature.to_bytes().to_vec();
        signature_bytes.push(recovery_id.to_byte() & 1);
        Ok(signature_bytes)
    }
    pub fn secp_sign_prehash_recoverable(
        &mut self,
        seed: &Option<Vec<u8>>,
        message: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let _ = self.get_secp_public_key(seed);
        let secret = self.secp256k1_keystore.get_mut(seed).unwrap();
        let (signature, recovery_id) = secret
            .secret
            .sign_prehash_recoverable(&message)
            .map_err(|e| e.to_string())?;
        self.write();
        let mut signature_bytes = signature.to_bytes().to_vec();
        signature_bytes.push(recovery_id.to_byte() & 1);
        Ok(signature_bytes)
    }
    pub fn ed25519_sign(&mut self, seed: &Option<Vec<u8>>, message: Vec<u8>) -> Vec<u8> {
        let _pair = self.get_ed25519_public_key(seed);
        let secret = self.ed25519_keystore.get_mut(&seed).unwrap();
        let r = secret.sign(message);
        self.write();
        r
    }
}

impl Secp256k1Pair {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let mut secret = [0u8; 32];
        csprng.try_fill_bytes(&mut secret).unwrap();
        let secret = SecpSigningKey::from_bytes(&secret.into()).unwrap();
        let public = secret.verifying_key();
        Self {
            secret: secret.to_owned(),
            public: public.to_owned(),
        }
    }
    #[cfg(test)]
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let secret = SecpSigningKey::from_bytes(&bytes.clone().into()).unwrap();
        let public = secret.verifying_key();
        Self {
            secret: secret.to_owned(),
            public: public.to_owned(),
        }
    }
}

impl Ed25519Pair {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let mut secret = [0u8; 32];
        csprng.try_fill_bytes(&mut secret).unwrap();
        let secret = Ed25519SigningKey::from_bytes(&secret.into());
        let public = Ed25519VerifyingKey::from(&secret);
        Self { secret, public }
    }
    pub fn sign(&mut self, message: Vec<u8>) -> Vec<u8> {
        self.secret.sign(&message).to_bytes().to_vec()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use hex::ToHex;
    use sha3::{Digest, Keccak256};
    use std::path::Path;
    #[test]
    fn test_secp_mock_keystore() {
        if Path::new(".test").exists() {
            std::fs::remove_dir_all(".test").unwrap();
        }
        std::fs::create_dir(".test").unwrap();
        let message = vec![
            0xec, 0x09, 0x85, 0x04, 0xa8, 0x17, 0xc8, 0x00, 0x82, 0x52, 0x08, 0x94, 0x35, 0x35,
            0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35, 0x35,
            0x35, 0x35, 0x35, 0x35, 0x88, 0x0d, 0xe0, 0xb6, 0xb3, 0xa7, 0x64, 0x00, 0x00, 0x80,
            0x01, 0x80, 0x80,
        ];
        let secret_raw = [
            0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46,
            0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46,
            0x46, 0x46, 0x46, 0x46,
        ];
        let mut hasher = Keccak256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        assert_eq!(
            hash.encode_hex::<String>(),
            "daf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53"
        );
        let mut keystore = MockKeystore::new(&PathBuf::from(".test"));

        let public_key1 = keystore.get_secp_public_key(&None);
        let seed = Some(b"test".to_vec());
        let signature1 = keystore.secp_sign_recoverable(&seed, hash.to_vec());

        let public_key2 = keystore.get_secp_public_key(&None);
        let signature2 = keystore.secp_sign_recoverable(&seed, hash.to_vec());
        assert_eq!(public_key1, public_key2);
        assert_eq!(signature1, signature2);

        let secp_pair = Secp256k1Pair::from_bytes(&secret_raw);
        assert_eq!(
            secp_pair
                .public
                .to_encoded_point(false)
                .as_bytes()
                .encode_hex::<String>(),
            "044bc2a31265153f07e70e0bab08724e6b85e217f8cd628ceb62974247bb493382ce28cab79ad7119ee1ad3ebcdb98a16805211530ecc6cfefa1b88e6dff99232a"
            );
        let signature = secp_pair.secret.sign_prehash_recoverable(&hash.as_slice());
        let signature = signature.unwrap().0.to_bytes();
        //28ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa63627667cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83
        assert_eq!(
            signatußre.encode_hex::<String>(),
            "28ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa63627667cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83"
        );
        std::fs::remove_dir_all(".test").unwrap();
    }
}
