use std::path::PathBuf;

use std::sync::Mutex;
use tokio::time::Duration;
use tss_sdk::crypto::CryptoType;
use tss_sdk::node::Node;

pub enum NodeRuntime {
    Node {
        tss_node: Node<crate::VrsTssValidatorIdentity>,
        runtime: tokio::runtime::Runtime,
        timeout: Option<Duration>,
    },
    Empty {
        mock_keystore: Mutex<crate::mock::MockKeystore>,
    },
}
pub static EMPTY_NODE_ERROR: &str =
    "NodeRuntime is empty. it is probably the number of nodes is not enough";

impl NodeRuntime {
    pub fn new_empty(base_path: &PathBuf) -> Self {
        NodeRuntime::Empty {
            mock_keystore: Mutex::new(crate::mock::MockKeystore::new(base_path)),
        }
    }
    pub fn new(node: Node<crate::VrsTssValidatorIdentity>, timeout: Option<Duration>) -> Self {
        Self::Node {
            tss_node: node,
            runtime: tokio::runtime::Runtime::new().unwrap(),
            timeout,
        }
    }
    pub fn get_public_key(
        &self,
        crypto_type: CryptoType,
        tweak_data: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        match self {
            NodeRuntime::Node {
                tss_node,
                runtime,
                timeout,
            } => {
                let auto_dkg = runtime
                    .block_on(tss_node.auto_dkg_async(timeout.clone()))
                    .map_err(|e| e.to_string())?;
                let pkid = auto_dkg
                    .get_pkid_by_crypto_type(crypto_type)
                    .map_err(|e| e.to_string())?;
                let public_key = runtime
                    .block_on(tss_node.pk_async(pkid, Some(tweak_data), timeout.clone()))
                    .map_err(|e| e.to_string())?;
                Ok(public_key.group_public_key_tweak)
            }
            NodeRuntime::Empty { mock_keystore } => match crypto_type {
                CryptoType::Ed25519 => {
                    let mut mock_keystore = mock_keystore.lock().unwrap();
                    let public_key = mock_keystore.get_ed25519_public_key(&Some(tweak_data));
                    return Ok(public_key);
                }
                CryptoType::Secp256k1 => {
                    let mut mock_keystore = mock_keystore.lock().unwrap();
                    let public_key = mock_keystore.get_secp_public_key(&Some(tweak_data));
                    return Ok(public_key);
                }
                _ => {
                    return Err(EMPTY_NODE_ERROR.to_string());
                }
            },
        }
    }
    pub fn sign(
        &self,
        crypto_type: CryptoType,
        message: Vec<u8>,
        tweak_data: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        match self {
            NodeRuntime::Node {
                tss_node,
                runtime,
                timeout,
            } => {
                let auto_dkg = runtime
                    .block_on(tss_node.auto_dkg_async(timeout.clone()))
                    .map_err(|e| e.to_string())?;
                let pkid = auto_dkg
                    .get_pkid_by_crypto_type(crypto_type)
                    .map_err(|e| e.to_string())?;
                let signature = runtime
                    .block_on(tss_node.sign_async(pkid, message, Some(tweak_data), timeout.clone()))
                    .map_err(|e| e.to_string())?;
                Ok(signature.signature())
            }
            NodeRuntime::Empty { mock_keystore } => match crypto_type {
                CryptoType::Ed25519 => {
                    let mut mock_keystore = mock_keystore.lock().unwrap();
                    let signature = mock_keystore.ed25519_sign(&Some(tweak_data), message);
                    return Ok(signature);
                }
                CryptoType::Secp256k1 => {
                    let mut mock_keystore = mock_keystore.lock().unwrap();
                    let signature =
                        mock_keystore.secp_sign_prehash_recoverable(&Some(tweak_data), message)?;
                    return Ok(signature);
                }
                _ => return Err(EMPTY_NODE_ERROR.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha3::{Digest, Keccak256};
    use std::path::Path;
    #[test]
    fn test_mock_keystore() {
        if Path::new(".test_mock_keystore").exists() {
            std::fs::remove_dir_all(".test_mock_keystore").unwrap();
        }
        // create .test directory
        std::fs::create_dir(".test_mock_keystore").unwrap();
        let runtime = NodeRuntime::new_empty(&PathBuf::from(".test_mock_keystore"));
        let public_key1 = runtime.get_public_key(CryptoType::Secp256k1, b"test".to_vec());
        println!("public_key: {:?}", public_key1);
        let message = b"test";

        let mut hasher = Keccak256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        let signature1 = runtime.sign(CryptoType::Secp256k1, hash.to_vec(), b"test".to_vec());
        println!("signature: {:?}", signature1);
        let public_key2 = runtime.get_public_key(CryptoType::Secp256k1, b"test".to_vec());
        println!("public_key2: {:?}", public_key2);
        let signature2 = runtime.sign(CryptoType::Secp256k1, hash.to_vec(), b"test".to_vec());
        assert!(signature2.clone().unwrap().len() == 65);
        println!("signature2: {:?}", signature2);
        assert_eq!(public_key1, public_key2);
        assert_eq!(signature1, signature2);
        std::fs::remove_dir_all(".test_mock_keystore").unwrap();
    }
}
