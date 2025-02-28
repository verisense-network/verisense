use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use ed25519_dalek::{ed25519::signature::SignerMut, SecretKey, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Pair {
    pub(crate) public: VerifyingKey,
    pub(crate) secret: SigningKey,
}
#[derive(Serialize, Deserialize)]
pub struct MockKeystore {
    #[serde(skip)]
    pub(crate) path: PathBuf,
    pub(crate) ed25519_keystore: HashMap<Option<Vec<u8>>, Pair>,
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
            };
            let file = File::create(path).unwrap();
            bincode::serialize_into(file, &keystore).unwrap();
            keystore
        }
    }
    pub fn write(&self) {
        // open a file to write
        let mut file = File::create(self.path.clone()).unwrap();
        bincode::serialize_into(&mut file, &self).unwrap();
    }
    pub fn get_public_key(&mut self, seed: &Option<Vec<u8>>) -> Vec<u8> {
        let pair = self.ed25519_keystore.get(&seed);
        match pair {
            Some(pair) => pair.public.to_bytes().to_vec(),
            None => {
                let pair = Pair::new();
                self.ed25519_keystore.insert(seed.clone(), pair.clone());
                self.write();
                pair.public.to_bytes().to_vec()
            }
        }
    }
    pub fn sign(&mut self, seed: &Option<Vec<u8>>, message: Vec<u8>) -> Vec<u8> {
        let _pair = self.get_public_key(&seed);
        let secret = self.ed25519_keystore.get_mut(&seed).unwrap();
        let r = secret.sign(message);
        self.write();
        r
    }
}
use rand::{rngs::OsRng, TryRngCore};
impl Pair {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let mut secret = SecretKey::default();
        csprng.try_fill_bytes(&mut secret).unwrap();
        let secret = SigningKey::from_bytes(&secret);
        let public = VerifyingKey::from(&secret);
        Self { secret, public }
    }
    pub fn sign(&mut self, message: Vec<u8>) -> Vec<u8> {
        self.secret.sign(&message).to_bytes().to_vec()
    }
}
#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    #[test]
    fn test_mock_keystore() {
        if Path::new(".test_mock_keystore").exists() {
            std::fs::remove_dir_all(".test").unwrap();
        }
        // create .test directory
        std::fs::create_dir(".test").unwrap();
        let mut keystore = MockKeystore::new(&PathBuf::from(".test"));

        let public_key1 = keystore.get_public_key(&None);
        let seed = Some(b"test".to_vec());
        let signature1 = keystore.sign(&seed, vec![1, 2, 3]);
        let public_key2 = keystore.get_public_key(&None);
        let seed = Some(b"test".to_vec());
        let signature2 = keystore.sign(&seed, vec![1, 2, 3]);
        assert_eq!(public_key1, public_key2);
        assert_eq!(signature1, signature2);
        std::fs::remove_dir_all(".test").unwrap();
    }
}
